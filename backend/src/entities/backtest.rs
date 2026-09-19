use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::enums::backtest_status::BacktestStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestOwnerResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, FromRow)]
pub struct BacktestJobRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub strategy_id: Uuid,
    pub strategy_name: String,
    pub name: String,
    pub year: i32,
    pub initial_cash: f64,
    pub max_holding_stocks: i32,
    pub max_stocks: i32,
    pub backtest_duration_months: i32,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub is_public: bool,
    pub status: BacktestStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(default)]
    pub owner_name: Option<String>,
    #[sqlx(default)]
    pub owner_email: Option<String>,
    #[sqlx(default)]
    pub start_date: Option<NaiveDate>,
    #[sqlx(default)]
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow)]
pub struct BacktestResultRecord {
    pub id: Uuid,
    pub backtest_job_id: Uuid,
    pub available_cash: f64,
    pub trades_processed: i32,
    pub net_pnl: f64,
    pub net_pnl_percentage: f64,
    pub gross_pnl: f64,
    pub gross_pnl_percentage: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub wins: i32,
    pub losses: i32,
    pub sharpe_ratio: f64,
    pub max_profit: f64,
    pub max_profit_percentage: f64,
    pub max_loss: f64,
    pub max_loss_percentage: f64,
    pub avg_profit: f64,
    pub avg_profit_percentage: f64,
    pub avg_loss: f64,
    pub avg_loss_percentage: f64,
    pub avg_hold_time_days: f64,
    pub total_fees: f64,
    pub avg_win_hold_days: f64,
    pub avg_loss_hold_days: f64,
    pub portfolio_volatility: f64,
    pub ai_summary: Option<sqlx::types::Json<Vec<String>>>,
}

#[derive(Debug, FromRow)]
pub struct BacktestPortfolioHistoryRecord {
    pub id: Uuid,
    pub backtest_job_id: Uuid,
    pub date: NaiveDate,
    pub net_value: f64,
    pub gross_value: f64,
}

#[derive(Debug, FromRow)]
pub struct BacktestTradeRecord {
    pub id: Uuid,
    pub backtest_job_id: Uuid,
    pub code: String,
    pub pnl: f64,
    pub pnl_percentage: f64,
    pub exit_reason: String,
    pub lot: i32,
    pub buy_price: f64,
    pub buy_value: f64,
    pub sell_price: f64,
    pub sell_value: f64,
    pub buy_fee: f64,
    pub sell_fee: f64,
    pub buy_date: NaiveDate,
    pub sell_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct CreateBacktestParams {
    pub user_id: Uuid,
    pub strategy_id: Uuid,
    pub strategy_name: String,
    pub name: String,
    pub year: i32,
    pub initial_cash: f64,
    pub max_holding_stocks: i32,
    pub max_stocks: i32,
    pub backtest_duration_months: i32,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub is_public: bool,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow)]
pub struct MostTradedRow {
    pub code: String,
    pub total: i64,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

#[derive(Debug, FromRow)]
pub struct TopEntryRow {
    pub code: String,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

#[derive(Debug, Clone)]
pub struct StockData {
    pub code: String,
    pub daily_data: Vec<(NaiveDate, f64, u64)>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub code: String,
    pub lots: i32,
    pub buy_price: f64,
    pub buy_value: f64,
    pub buy_fee: f64,
    pub buy_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct BacktestJobResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub strategy_id: Uuid,
    pub strategy_name: String,
    pub name: String,
    pub year: i32,
    pub initial_cash: f64,
    pub max_holding_stocks: i32,
    pub max_stocks: i32,
    pub backtest_duration_months: i32,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub is_public: bool,
    pub status: BacktestStatus,
    pub error_message: Option<String>,
    pub owner: Option<BacktestOwnerResponse>,
    pub result: Option<BacktestResultResponse>,
    pub portfolio_history: Option<Vec<PortfolioHistoryResponse>>,
    pub most_traded: Option<Vec<MostTradedResponse>>,
    pub top_gainers: Option<Vec<TopEntryResponse>>,
    pub top_losers: Option<Vec<TopEntryResponse>>,
    pub trade_history: Option<Vec<TradeHistoryResponse>>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BacktestResultResponse {
    pub available_cash: f64,
    pub trades_processed: i32,
    pub net_pnl: f64,
    pub net_pnl_percentage: f64,
    pub gross_pnl: f64,
    pub gross_pnl_percentage: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub wins: i32,
    pub losses: i32,
    pub sharpe_ratio: f64,
    pub max_profit: f64,
    pub max_profit_percentage: f64,
    pub max_loss: f64,
    pub max_loss_percentage: f64,
    pub avg_profit: f64,
    pub avg_profit_percentage: f64,
    pub avg_loss: f64,
    pub avg_loss_percentage: f64,
    pub avg_hold_time_days: f64,
    pub total_fees: f64,
    pub avg_win_hold_days: f64,
    pub avg_loss_hold_days: f64,
    pub portfolio_volatility: f64,
    pub ai_summary: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PortfolioHistoryResponse {
    pub date: NaiveDate,
    pub net_value: f64,
    pub gross_value: f64,
}

#[derive(Debug, Serialize)]
pub struct MostTradedResponse {
    pub code: String,
    pub total: i64,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct TopEntryResponse {
    pub code: String,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct TradeHistoryResponse {
    pub id: Uuid,
    pub code: String,
    pub company_name: Option<String>,
    pub pnl: f64,
    pub pnl_percentage: f64,
    pub exit_reason: String,
    pub lot: i32,
    pub buy_price: f64,
    pub buy_value: f64,
    pub sell_price: f64,
    pub sell_value: f64,
    pub buy_fee: f64,
    pub sell_fee: f64,
    pub buy_date: NaiveDate,
    pub sell_date: NaiveDate,
    pub query_values: Option<serde_json::Value>,
}

const fn default_max_stocks() -> i32 {
    24
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CreateBacktestJobRequest {
    pub strategy_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub is_public: Option<bool>,
    #[validate(range(min = 2020, max = 2030))]
    pub year: i32,
    #[validate(range(min = 1_000_000.0, max = 100_000_000_000.0))]
    pub initial_cash: f64,
    #[validate(range(min = 1, max = 50))]
    pub max_holding_stocks: i32,
    #[serde(default = "default_max_stocks")]
    #[validate(range(min = 1, max = 100))]
    pub max_stocks: i32,
    pub backtest_duration_months: i32,
    #[validate(range(min = 0.0, max = 10.0))]
    pub buy_fee_percentage: f64,
    #[validate(range(min = 0.0, max = 10.0))]
    pub sell_fee_percentage: f64,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBacktestJobRequest {
    pub is_public: Option<bool>,
}

impl UpdateBacktestJobRequest {
    pub const fn is_empty(&self) -> bool {
        self.is_public.is_none()
    }
}

impl From<BacktestResultRecord> for BacktestResultResponse {
    fn from(record: BacktestResultRecord) -> Self {
        Self {
            available_cash: record.available_cash,
            trades_processed: record.trades_processed,
            net_pnl: record.net_pnl,
            net_pnl_percentage: record.net_pnl_percentage,
            gross_pnl: record.gross_pnl,
            gross_pnl_percentage: record.gross_pnl_percentage,
            win_rate: record.win_rate,
            profit_factor: record.profit_factor,
            wins: record.wins,
            losses: record.losses,
            sharpe_ratio: record.sharpe_ratio,
            max_profit: record.max_profit,
            max_profit_percentage: record.max_profit_percentage,
            max_loss: record.max_loss,
            max_loss_percentage: record.max_loss_percentage,
            avg_profit: record.avg_profit,
            avg_profit_percentage: record.avg_profit_percentage,
            avg_loss: record.avg_loss,
            avg_loss_percentage: record.avg_loss_percentage,
            avg_hold_time_days: record.avg_hold_time_days,
            total_fees: record.total_fees,
            avg_win_hold_days: record.avg_win_hold_days,
            avg_loss_hold_days: record.avg_loss_hold_days,
            portfolio_volatility: record.portfolio_volatility,
            ai_summary: record.ai_summary.map(|j| j.0),
        }
    }
}

impl From<BacktestPortfolioHistoryRecord> for PortfolioHistoryResponse {
    fn from(record: BacktestPortfolioHistoryRecord) -> Self {
        Self {
            date: record.date,
            net_value: record.net_value,
            gross_value: record.gross_value,
        }
    }
}

impl TradeHistoryResponse {
    pub fn from_record_with_info(
        record: BacktestTradeRecord,
        company_name: Option<String>,
        query_values: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: record.id,
            code: record.code,
            company_name,
            pnl: record.pnl,
            pnl_percentage: record.pnl_percentage,
            exit_reason: record.exit_reason,
            lot: record.lot,
            buy_price: record.buy_price,
            buy_value: record.buy_value,
            sell_price: record.sell_price,
            sell_value: record.sell_value,
            buy_fee: record.buy_fee,
            sell_fee: record.sell_fee,
            buy_date: record.buy_date,
            sell_date: record.sell_date,
            query_values,
        }
    }
}

impl From<BacktestTradeRecord> for TradeHistoryResponse {
    fn from(record: BacktestTradeRecord) -> Self {
        Self::from_record_with_info(record, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_create_backtest_job_request() {
        let valid = CreateBacktestJobRequest {
            strategy_id: Uuid::new_v4(),
            name: "Momentum Backtest 2024".to_string(),
            is_public: Some(true),
            year: 2024,
            initial_cash: 100_000_000.0,
            max_holding_stocks: 3,
            max_stocks: 24,
            backtest_duration_months: 12,
            buy_fee_percentage: 0.15,
            sell_fee_percentage: 0.25,
            start_date: None,
            end_date: None,
        };
        assert!(valid.validate().is_ok());

        let invalid_name = CreateBacktestJobRequest {
            name: "".to_string(),
            ..valid.clone()
        };
        assert!(invalid_name.validate().is_err());

        let invalid_year = CreateBacktestJobRequest {
            year: 2010,
            ..valid.clone()
        };
        assert!(invalid_year.validate().is_err());

        let invalid_cash = CreateBacktestJobRequest {
            initial_cash: 500.0,
            ..valid.clone()
        };
        assert!(invalid_cash.validate().is_err());

        let invalid_holding = CreateBacktestJobRequest {
            max_holding_stocks: 0,
            ..valid.clone()
        };
        assert!(invalid_holding.validate().is_err());

        let invalid_max_stocks = CreateBacktestJobRequest {
            max_stocks: 0,
            ..valid.clone()
        };
        assert!(invalid_max_stocks.validate().is_err());

        let invalid_fees = CreateBacktestJobRequest {
            buy_fee_percentage: -1.0,
            ..valid
        };
        assert!(invalid_fees.validate().is_err());
    }

    #[test]
    fn should_convert_records_to_responses() {
        let job_id = Uuid::new_v4();
        let result_record = BacktestResultRecord {
            id: Uuid::new_v4(),
            backtest_job_id: job_id,
            available_cash: 105_000_000.0,
            trades_processed: 10,
            net_pnl: 5_000_000.0,
            net_pnl_percentage: 5.0,
            gross_pnl: 5_500_000.0,
            gross_pnl_percentage: 5.5,
            win_rate: 70.0,
            profit_factor: 2.1,
            wins: 7,
            losses: 3,
            sharpe_ratio: 1.45,
            max_profit: 2_000_000.0,
            max_profit_percentage: 6.0,
            max_loss: -800_000.0,
            max_loss_percentage: -2.4,
            avg_profit: 1_000_000.0,
            avg_profit_percentage: 3.0,
            avg_loss: -500_000.0,
            avg_loss_percentage: -1.5,
            avg_hold_time_days: 5.2,
            total_fees: 500_000.0,
            avg_win_hold_days: 6.0,
            avg_loss_hold_days: 3.3,
            portfolio_volatility: 12.5,
            ai_summary: None,
        };

        let res = BacktestResultResponse::from(result_record);
        assert_eq!(res.trades_processed, 10);
        assert_eq!(res.win_rate, 70.0);
        assert_eq!(res.net_pnl, 5_000_000.0);
    }

    #[test]
    fn should_serialize_backtest_status_in_job_response() {
        let job = BacktestJobResponse {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            strategy_name: "Strategy A".to_string(),
            name: "Job A".to_string(),
            year: 2024,
            initial_cash: 100_000_000.0,
            max_holding_stocks: 3,
            max_stocks: 24,
            backtest_duration_months: 12,
            buy_fee_percentage: 0.15,
            sell_fee_percentage: 0.25,
            is_public: false,
            status: BacktestStatus::Processing,
            error_message: None,
            owner: None,
            result: None,
            portfolio_history: None,
            most_traded: None,
            top_gainers: None,
            top_losers: None,
            trade_history: None,
            start_date: None,
            end_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&job).expect("serialize job");
        assert!(json.contains("\"status\":\"PROCESSING\""));
    }

    #[test]
    fn should_populate_trade_history_with_company_name_and_query_values() {
        let record = BacktestTradeRecord {
            id: Uuid::new_v4(),
            backtest_job_id: Uuid::new_v4(),
            code: "BBCA.JK".to_string(),
            pnl: 500_000.0,
            pnl_percentage: 5.0,
            exit_reason: "TAKE_PROFIT".to_string(),
            lot: 10,
            buy_price: 9000.0,
            buy_value: 9_000_000.0,
            sell_price: 9450.0,
            sell_value: 9_450_000.0,
            buy_fee: 13500.0,
            sell_fee: 23625.0,
            buy_date: NaiveDate::from_ymd_opt(2024, 1, 10).unwrap(),
            sell_date: NaiveDate::from_ymd_opt(2024, 1, 25).unwrap(),
        };

        let qv = serde_json::json!({
            "sub_sector": "Banks",
            "market_cap": 753611199412500_i64
        });

        let resp = TradeHistoryResponse::from_record_with_info(
            record,
            Some("PT Bank Central Asia Tbk.".to_string()),
            Some(qv.clone()),
        );

        assert_eq!(
            resp.company_name.as_deref(),
            Some("PT Bank Central Asia Tbk.")
        );
        assert_eq!(resp.query_values, Some(qv));

        let json = serde_json::to_string(&resp).expect("serialize trade response");
        assert!(json.contains("\"company_name\":\"PT Bank Central Asia Tbk.\""));
        assert!(json.contains("\"sub_sector\":\"Banks\""));
    }
}
