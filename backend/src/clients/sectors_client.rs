use async_trait::async_trait;
use chrono::{Days, NaiveDate};
use redis::aio::ConnectionManager;
use reqwest::Client;

use crate::entities::app_error::AppError;

pub use crate::entities::sectors::{
    DailyTransaction, DefaultScreenerResponse, ForeignFlowDailyPoint, ForeignFlowResponse,
    ScreenerCompany, ScreenerResponse,
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
    async fn foreign_flow(
        &self,
        symbol: &str,
        start: &str,
        end: &str,
    ) -> Result<ForeignFlowResponse, AppError>;
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

/// Maximum number of days per Sectors daily-transaction API call.
const MAX_DAYS_PER_CHUNK: u64 = 90;

/// Maximum number of retries on HTTP 429 (rate-limited) responses.
const MAX_RETRIES: u32 = 2;

/// Fixed delay in seconds when HTTP 429 is received (matches the 60-second quota window).
const RETRY_DELAY_SECS: u64 = 60;

/// Splits a `start..=end` date range into consecutive chunks of at most
/// [`MAX_DAYS_PER_CHUNK`] days each, returning `(chunk_start, chunk_end)` pairs.
///
/// Returns an empty vec when `start > end`.
pub fn compute_date_chunks(start: NaiveDate, end: NaiveDate) -> Vec<(NaiveDate, NaiveDate)> {
    if start > end {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut chunk_start = start;

    while chunk_start <= end {
        let chunk_end_candidate = chunk_start
            .checked_add_days(Days::new(MAX_DAYS_PER_CHUNK - 1))
            .unwrap_or(end);
        let chunk_end = if chunk_end_candidate > end {
            end
        } else {
            chunk_end_candidate
        };
        chunks.push((chunk_start, chunk_end));
        chunk_start = chunk_end
            .checked_add_days(Days::new(1))
            .unwrap_or(chunk_end);
        // Guard against overflow at NaiveDate::MAX
        if chunk_start <= chunk_end {
            break;
        }
    }

    chunks
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

        let mut retries = 0u32;
        let response = loop {
            let res = self
                .http
                .get(url)
                .query(&[
                    ("where", where_query),
                    ("limit", &limit_str),
                    ("order_by", "last_close_price"),
                    ("include_query_values", "true"),
                ])
                .header("Authorization", &self.api_key)
                .send()
                .await
                .map_err(|_e| AppError::Internal)?;

            if res.status().as_u16() == 429 && retries < MAX_RETRIES {
                retries += 1;
                eprintln!(
                    "[SectorsClient] Got 429 for screener, retrying (attempt {}/{}) after {}s...",
                    retries, MAX_RETRIES, RETRY_DELAY_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
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

        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|e| {
            eprintln!("[SectorsClient] Invalid start date '{}': {:?}", start, e);
            AppError::bad_request(format!("Invalid start date: {}", start))
        })?;
        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|e| {
            eprintln!("[SectorsClient] Invalid end date '{}': {:?}", end, e);
            AppError::bad_request(format!("Invalid end date: {}", end))
        })?;

        let chunks = compute_date_chunks(start_date, end_date);
        if chunks.is_empty() {
            return Ok(Vec::new());
        }

        eprintln!(
            "[SectorsClient] Fetching daily transactions for {} from {} to {} ({} chunk(s))",
            clean_symbol,
            start,
            end,
            chunks.len()
        );

        let mut all_data: Vec<DailyTransaction> = Vec::new();

        for (chunk_start, chunk_end) in &chunks {
            let chunk_start_str = chunk_start.format("%Y-%m-%d").to_string();
            let chunk_end_str = chunk_end.format("%Y-%m-%d").to_string();
            let key = format!(
                "sectors:daily:{}:{}:{}",
                clean_symbol, chunk_start_str, chunk_end_str
            );
            let mut redis = self.redis.clone();

            // Check per-chunk cache
            let cached: Option<String> =
                redis::cmd("GET").arg(&key).query_async(&mut redis).await?;
            if let Some(json) = cached {
                let chunk_data: Vec<DailyTransaction> =
                    serde_json::from_str(&json).unwrap_or_default();
                all_data.extend(chunk_data);
                continue;
            }

            // Cache miss — hit the API for this chunk
            let url = format!(
                "https://api.sectors.app/v2/daily/{}/?start={}&end={}",
                clean_symbol, chunk_start_str, chunk_end_str
            );
            eprintln!("[SectorsClient] Cache miss, hitting Sectors API: {}", url);

            let mut retries = 0u32;
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

                if res.status().as_u16() == 429 && retries < MAX_RETRIES {
                    retries += 1;
                    eprintln!(
                        "[SectorsClient] Got 429 for {}, retrying (attempt {}/{}) after {}s...",
                        url, retries, MAX_RETRIES, RETRY_DELAY_SECS
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
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

            let chunk_data: Vec<DailyTransaction> = response.json().await.map_err(|e| {
                eprintln!(
                    "[SectorsClient] Failed to decode daily transactions JSON for {}: {:?}",
                    clean_symbol, e
                );
                AppError::Internal
            })?;

            // Cache this chunk in Redis (no expiration)
            if let Ok(json) = serde_json::to_string(&chunk_data) {
                let _: () = redis::cmd("SET")
                    .arg(&key)
                    .arg(&json)
                    .query_async(&mut redis)
                    .await?;
            }

            all_data.extend(chunk_data);
        }

        Ok(all_data)
    }

    async fn foreign_flow(
        &self,
        symbol: &str,
        start: &str,
        end: &str,
    ) -> Result<ForeignFlowResponse, AppError> {
        let clean_symbol = symbol.trim_end_matches(".JK");

        let key = format!("sectors:foreign_flow:{}:{}:{}", clean_symbol, start, end);
        let mut redis = self.redis.clone();

        // Check Redis cache first
        let cached: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut redis).await?;
        if let Some(json) = cached {
            if let Ok(data) = serde_json::from_str::<ForeignFlowResponse>(&json) {
                return Ok(data);
            }
        }

        let url = format!(
            "https://api.sectors.app/v2/foreign-flow/{}/?start={}&end={}",
            clean_symbol, start, end
        );
        eprintln!("[SectorsClient] Cache miss, hitting Sectors API: {}", url);

        let mut retries = 0u32;
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

            if res.status().as_u16() == 429 && retries < MAX_RETRIES {
                retries += 1;
                eprintln!(
                    "[SectorsClient] Got 429 for {}, retrying (attempt {}/{}) after {}s...",
                    url, retries, MAX_RETRIES, RETRY_DELAY_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
                continue;
            }

            break res;
        };

        if response.status().as_u16() == 404 {
            eprintln!(
                "[SectorsClient] Symbol {} not found in broker data (404), returning empty foreign flow",
                clean_symbol
            );
            return Ok(ForeignFlowResponse {
                symbol: clean_symbol.to_string(),
                start: start.to_string(),
                end: end.to_string(),
                data: Vec::new(),
            });
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            eprintln!(
                "[SectorsClient] Foreign Flow API failed for {} with status {}: {}",
                clean_symbol, status, body
            );
            return Err(AppError::bad_request(format!(
                "Sectors Foreign Flow API error ({}): {}",
                status, body
            )));
        }

        let data: ForeignFlowResponse = response.json().await.map_err(|e| {
            eprintln!(
                "[SectorsClient] Failed to decode foreign flow JSON response for {}: {:?}",
                clean_symbol, e
            );
            AppError::Internal
        })?;

        if let Ok(json) = serde_json::to_string(&data) {
            let _: () = redis::cmd("SET")
                .arg(&key)
                .arg(&json)
                .query_async(&mut redis)
                .await?;
        }

        Ok(data)
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

    // ── compute_date_chunks tests ──────────────────────────────────────

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    #[test]
    fn chunks_single_day_range() {
        let chunks = compute_date_chunks(d(2026, 1, 1), d(2026, 1, 1));
        assert_eq!(chunks, vec![(d(2026, 1, 1), d(2026, 1, 1))]);
    }

    #[test]
    fn chunks_range_under_90_days() {
        // 30-day range: Jan 1 – Jan 30
        let chunks = compute_date_chunks(d(2026, 1, 1), d(2026, 1, 30));
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], (d(2026, 1, 1), d(2026, 1, 30)));
    }

    #[test]
    fn chunks_exact_90_day_range() {
        // Exactly 90 days: Jan 1 – Mar 31 (31+28+31 = 90 days inclusive)
        let start = d(2026, 1, 1);
        let end = d(2026, 3, 31);
        let diff = (end - start).num_days();
        assert_eq!(diff, 89); // 90 days inclusive = 89 days difference

        let chunks = compute_date_chunks(start, end);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], (start, end));
    }

    #[test]
    fn chunks_91_day_range_produces_two_chunks() {
        // 91 days: Jan 1 – Apr 1
        let start = d(2026, 1, 1);
        let end = d(2026, 4, 1);
        let diff = (end - start).num_days() + 1;
        assert_eq!(diff, 91);

        let chunks = compute_date_chunks(start, end);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], (d(2026, 1, 1), d(2026, 3, 31))); // 90 days
        assert_eq!(chunks[1], (d(2026, 4, 1), d(2026, 4, 1))); // 1 day
    }

    #[test]
    fn chunks_180_day_range_produces_two_chunks() {
        // 180 days: Jan 1 – Jun 29 (31+28+31+30+31+29 = 180)
        let start = d(2026, 1, 1);
        let end = d(2026, 6, 29);
        let total_days = (end - start).num_days() + 1;
        assert_eq!(total_days, 180);

        let chunks = compute_date_chunks(start, end);
        assert_eq!(chunks.len(), 2);
        // First chunk: Jan 1 – Mar 31 (90 days)
        assert_eq!(chunks[0], (d(2026, 1, 1), d(2026, 3, 31)));
        // Second chunk: Apr 1 – Jun 29 (90 days)
        assert_eq!(chunks[1], (d(2026, 4, 1), d(2026, 6, 29)));
    }

    #[test]
    fn chunks_365_day_range_produces_five_chunks() {
        // Full year 2026: Jan 1 – Dec 31 = 365 days
        let start = d(2026, 1, 1);
        let end = d(2026, 12, 31);
        let total_days = (end - start).num_days() + 1;
        assert_eq!(total_days, 365);

        let chunks = compute_date_chunks(start, end);
        // 365 / 90 = 4.06 → 5 chunks (4×90 + 5)
        assert_eq!(chunks.len(), 5);

        // Verify no gaps and no overlaps
        for i in 1..chunks.len() {
            let prev_end = chunks[i - 1].1;
            let curr_start = chunks[i].0;
            assert_eq!(
                curr_start,
                prev_end.checked_add_days(Days::new(1)).unwrap(),
                "Chunk {} should start the day after chunk {} ends",
                i,
                i - 1
            );
        }

        // First chunk starts at start, last chunk ends at end
        assert_eq!(chunks[0].0, start);
        assert_eq!(chunks[chunks.len() - 1].1, end);

        // Each chunk is at most 90 days
        for (i, (cs, ce)) in chunks.iter().enumerate() {
            let chunk_days = (*ce - *cs).num_days() + 1;
            assert!(
                chunk_days <= 90,
                "Chunk {} has {} days, exceeds 90",
                i,
                chunk_days
            );
        }
    }

    #[test]
    fn chunks_start_after_end_returns_empty() {
        let chunks = compute_date_chunks(d(2026, 6, 1), d(2026, 1, 1));
        assert!(chunks.is_empty());
    }

    #[test]
    fn chunks_leap_year_boundary() {
        // Leap year 2024: Feb 28 – May 28 = 90 days inclusive
        let start = d(2024, 2, 28);
        let end = d(2024, 5, 27);
        let total_days = (end - start).num_days() + 1;
        assert_eq!(total_days, 90);

        let chunks = compute_date_chunks(start, end);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], (start, end));
    }

    #[test]
    fn chunks_cover_entire_range_without_gaps() {
        // Arbitrary range: 200 days
        let start = d(2025, 3, 15);
        let end = d(2025, 9, 30);
        let total_days = (end - start).num_days() + 1;
        assert_eq!(total_days, 200);

        let chunks = compute_date_chunks(start, end);
        // 200 / 90 = 2.22 → 3 chunks
        assert_eq!(chunks.len(), 3);

        // Verify continuity
        assert_eq!(chunks[0].0, start);
        for i in 1..chunks.len() {
            assert_eq!(
                chunks[i].0,
                chunks[i - 1].1.checked_add_days(Days::new(1)).unwrap()
            );
        }
        assert_eq!(chunks[chunks.len() - 1].1, end);

        // Verify total covered days
        let covered: i64 = chunks.iter().map(|(s, e)| (*e - *s).num_days() + 1).sum();
        assert_eq!(covered, total_days);
    }
}
