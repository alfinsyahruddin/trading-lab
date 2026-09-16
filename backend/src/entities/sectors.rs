use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenerCompany {
    pub symbol: String,
    pub company_name: String,
    #[serde(default)]
    pub query_values: Option<serde_json::Value>,
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

#[derive(Debug, Deserialize)]
pub struct DefaultScreenerResponse {
    pub results: Vec<ScreenerCompany>,
}
