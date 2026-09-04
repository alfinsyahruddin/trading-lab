use async_trait::async_trait;
use redis::aio::ConnectionManager;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::entities::app_error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerCompany {
    pub symbol: String,
    pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerResponse {
    pub results: Vec<ScreenerCompany>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTransaction {
    pub symbol: String,
    pub date: String,
    pub close: f64,
    pub volume: u64,
    pub market_cap: f64,
}

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
            "[SectorsClient] Cache miss, hitting Sectors API: {}?where={}&limit={}&order_by=symbol",
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

#[derive(Deserialize)]
struct DefaultScreenerResponse {
    results: Vec<ScreenerCompany>,
}
