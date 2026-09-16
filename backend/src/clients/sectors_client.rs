use async_trait::async_trait;
use redis::aio::ConnectionManager;
use reqwest::Client;

use crate::entities::app_error::AppError;

pub use crate::entities::sectors::{
    DailyTransaction, DefaultScreenerResponse, ScreenerCompany, ScreenerResponse,
};

#[async_trait]
pub trait SectorsClientTrait: Send + Sync {
    async fn screener(
        &self,
        where_query: &str,
        limit: i32,
    ) -> Result<Vec<ScreenerCompany>, AppError>;
    async fn daily_transactions(
        &self,
        symbol: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<DailyTransaction>, AppError>;
}

pub struct SectorsClient {
    http: Client,
    api_key: String,
    redis: ConnectionManager,
}

impl SectorsClient {
    pub fn new(http: Client, api_key: String, redis: ConnectionManager) -> Self {
        Self {
            http,
            api_key,
            redis,
        }
    }
}

#[async_trait]
impl SectorsClientTrait for SectorsClient {
    async fn screener(
        &self,
        where_query: &str,
        limit: i32,
    ) -> Result<Vec<ScreenerCompany>, AppError> {
        let key = format!("sectors:screener:{}:{}", where_query, limit);
        let mut redis = self.redis.clone();

        let cached: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut redis).await?;
        if let Some(json) = cached {
            return Ok(serde_json::from_str(&json).unwrap_or_default());
        }

        let url = "https://api.sectors.app/v2/companies/";
        let limit_str = limit.to_string();
        eprintln!(
            "[SectorsClient] Cache miss, hitting Sectors API: {}?where={}&limit={}&order_by=symbol&include_query_values=true",
            url, where_query, limit_str
        );

        let mut retries = 0;
        let response = loop {
            let res = self
                .http
                .get(url)
                .query(&[
                    ("where", where_query),
                    ("limit", &limit_str),
                    ("order_by", "symbol"),
                    ("include_query_values", "true"),
                ])
                .header("Authorization", &self.api_key)
                .send()
                .await
                .map_err(|_e| AppError::Internal)?;

            if res.status().as_u16() == 429 && retries < 2 {
                retries += 1;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }

            break res;
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            eprintln!(
                "[SectorsClient] Screener API failed with status {}: {}",
                status, body
            );
            return Err(AppError::bad_request(format!(
                "Sectors Screener API error ({}): {}",
                status, body
            )));
        }

        let data: DefaultScreenerResponse = response.json().await.map_err(|e| {
            eprintln!(
                "[SectorsClient] Failed to decode screener JSON response: {:?}",
                e
            );
            AppError::Internal
        })?;
        let result = data.results;

        if let Ok(json) = serde_json::to_string(&result) {
            let _: () = redis::cmd("SET")
                .arg(&key)
                .arg(&json)
                .query_async(&mut redis)
                .await?;
        }

        Ok(result)
    }

    async fn daily_transactions(
        &self,
        symbol: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<DailyTransaction>, AppError> {
        let clean_symbol = symbol.trim_end_matches(".JK");
        let key = format!("sectors:daily:{}:{}:{}", clean_symbol, start, end);
        let mut redis = self.redis.clone();

        let cached: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut redis).await?;
        if let Some(json) = cached {
            return Ok(serde_json::from_str(&json).unwrap_or_default());
        }

        let url = format!(
            "https://api.sectors.app/v2/daily/{}/?start={}&end={}",
            clean_symbol, start, end
        );
        eprintln!("[SectorsClient] Cache miss, hitting Sectors API: {}", url);

        let mut retries = 0;
        let response = loop {
            let res = self
                .http
                .get(&url)
                .header("Authorization", &self.api_key)
                .send()
                .await
                .map_err(|e| {
                    eprintln!("[SectorsClient] HTTP request failed for {}: {:?}", url, e);
                    AppError::Internal
                })?;

            if res.status().as_u16() == 429 && retries < 2 {
                retries += 1;
                eprintln!(
                    "[SectorsClient] Got 429 for {}, retrying (attempt {}) after 1s...",
                    url, retries
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }

            break res;
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            eprintln!(
                "[SectorsClient] Daily Transactions API failed for {} with status {}: {}",
                clean_symbol, status, body
            );
            return Err(AppError::bad_request(format!(
                "Sectors Daily API error for {} ({}): {}",
                clean_symbol, status, body
            )));
        }

        let result: Vec<DailyTransaction> = response.json().await.map_err(|e| {
            eprintln!(
                "[SectorsClient] Failed to decode daily transactions JSON for {}: {:?}",
                clean_symbol, e
            );
            AppError::Internal
        })?;

        if let Ok(json) = serde_json::to_string(&result) {
            let _: () = redis::cmd("SET")
                .arg(&key)
                .arg(&json)
                .query_async(&mut redis)
                .await?;
        }

        Ok(result)
    }
}

// Ensure urlencoding crate is used if needed.
// Need to add `urlencoding` to Cargo.toml. Wait, I can just use reqwest query params or `urlencoding`.
// The prompt says "When building the where query for screener, URL-encode special characters using `urlencoding` or manual encoding. Actually the reqwest client can handle query params — but since the where param has complex SQL-like syntax, just pass it as-is in the URL and let reqwest encode it."

// Wait, the prompt says: "Actually the reqwest client can handle query params — but since the where param has complex SQL-like syntax, just pass it as-is in the URL and let reqwest encode it."
// So I should let reqwest handle it, or use urlencoding. I'll just change to reqwest's query builder.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_screener_company_with_query_values() {
        let json_data = r#"{
            "results": [
                {
                    "symbol": "BBCA.JK",
                    "company_name": "PT Bank Central Asia Tbk.",
                    "query_values": {
                        "sub_sector": "Banks",
                        "market_cap": 753611199412500
                    }
                },
                {
                    "symbol": "BMRI.JK",
                    "company_name": "PT Bank Mandiri (Persero) Tbk."
                }
            ]
        }"#;

        let res: DefaultScreenerResponse = serde_json::from_str(json_data).expect("deserialize");
        assert_eq!(res.results.len(), 2);

        let bbca = &res.results[0];
        assert_eq!(bbca.symbol, "BBCA.JK");
        assert_eq!(bbca.company_name, "PT Bank Central Asia Tbk.");
        let qv = bbca.query_values.as_ref().expect("has query_values");
        assert_eq!(qv["sub_sector"], "Banks");
        assert_eq!(qv["market_cap"], 753611199412500_i64);

        let bmri = &res.results[1];
        assert_eq!(bmri.symbol, "BMRI.JK");
        assert!(bmri.query_values.is_none());

        // Test serialization to Redis format and back
        let serialized = serde_json::to_string(&res.results).expect("serialize results");
        let deserialized: Vec<ScreenerCompany> =
            serde_json::from_str(&serialized).expect("deserialize results");
        assert_eq!(deserialized[0].query_values, bbca.query_values);
    }
}
