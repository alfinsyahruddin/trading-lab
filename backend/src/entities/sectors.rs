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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForeignFlowDailyPoint {
    pub date: String,
    #[serde(default)]
    pub net_foreign_inflow: Option<i64>,
    #[serde(default)]
    pub foreign_buy_idr: Option<i64>,
    #[serde(default)]
    pub foreign_sell_idr: Option<i64>,
    #[serde(default)]
    pub foreign_share: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForeignFlowResponse {
    pub symbol: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub data: Vec<ForeignFlowDailyPoint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_foreign_flow_response_correctly() {
        let json_data = r#"{
            "symbol": "BBCA.JK",
            "start": "2025-05-01",
            "end": "2025-05-05",
            "data": [
                {
                    "date": "2025-05-02",
                    "net_foreign_inflow": 146476750000,
                    "foreign_buy_idr": 558094712500,
                    "foreign_sell_idr": 411617962500,
                    "foreign_share": 0.5875
                },
                {
                    "date": "2025-05-05",
                    "net_foreign_inflow": -5000000000,
                    "foreign_buy_idr": null,
                    "foreign_sell_idr": null,
                    "foreign_share": null
                }
            ]
        }"#;

        let res: ForeignFlowResponse =
            serde_json::from_str(json_data).expect("deserialize foreign flow");
        assert_eq!(res.symbol, "BBCA.JK");
        assert_eq!(res.start, "2025-05-01");
        assert_eq!(res.end, "2025-05-05");
        assert_eq!(res.data.len(), 2);
        assert_eq!(res.data[0].net_foreign_inflow, Some(146476750000));
        assert_eq!(res.data[0].foreign_share, Some(0.5875));
        assert_eq!(res.data[1].net_foreign_inflow, Some(-5000000000));
        assert_eq!(res.data[1].foreign_buy_idr, None);
    }
}
