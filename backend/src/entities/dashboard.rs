use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::enums::backtest_status::BacktestStatus;

#[derive(Debug, Serialize)]
pub struct DashboardStatsResponse {
    pub total_stars_received: i64,
    pub total_strategies: i64,
    pub total_backtests: i64,
    pub date_joined: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub id: Uuid,
    pub name: String,
    pub owner_name: String,
    pub strategy_name: String,
    pub year: i32,
    pub initial_cash: f64,
    pub backtest_duration_months: i32,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub status: BacktestStatus,
    pub net_pnl: f64,
    pub net_pnl_percentage: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub trades_processed: i32,
    pub star_count: i64,
    pub is_starred_by_me: bool,
    pub portfolio_history: Vec<LeaderboardPortfolioPoint>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardPortfolioPoint {
    pub date: NaiveDate,
    pub net_value: f64,
}
