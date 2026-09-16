use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    entities::{
        app_error::AppError,
        dashboard::{DashboardStatsResponse, LeaderboardEntry, LeaderboardPortfolioPoint},
    },
    enums::backtest_status::BacktestStatus,
    repositories::{
        dashboard_repository::{DashboardRepository, LeaderboardRow},
        user_repository::UserRepository,
    },
};

pub struct DashboardService {
    repo: DashboardRepository,
    user_repo: UserRepository,
}

impl DashboardService {
    pub fn new(repo: DashboardRepository, user_repo: UserRepository) -> Self {
        Self { repo, user_repo }
    }

    pub async fn get_stats(&self, user_id: Uuid) -> Result<DashboardStatsResponse, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;

        let stats = self.repo.get_user_stats(user_id).await?;

        Ok(DashboardStatsResponse {
            total_stars_received: stats.total_stars_received,
            total_strategies: stats.total_strategies,
            total_backtests: stats.total_backtests,
            date_joined: user.created_at,
        })
    }

    pub async fn leaderboard_by_percentage(
        &self,
        viewer_id: Uuid,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        let rows = self.repo.leaderboard_by_percentage(viewer_id).await?;
        self.enrich_with_portfolio(rows).await
    }

    pub async fn leaderboard_by_stars(
        &self,
        viewer_id: Uuid,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        let rows = self.repo.leaderboard_by_stars(viewer_id).await?;
        self.enrich_with_portfolio(rows).await
    }

    pub async fn star(&self, user_id: Uuid, backtest_id: Uuid) -> Result<(), AppError> {
        self.repo.add_star(user_id, backtest_id).await
    }

    pub async fn unstar(&self, user_id: Uuid, backtest_id: Uuid) -> Result<(), AppError> {
        self.repo.remove_star(user_id, backtest_id).await
    }

    async fn enrich_with_portfolio(
        &self,
        rows: Vec<LeaderboardRow>,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        if rows.is_empty() {
            return Ok(vec![]);
        }

        let job_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let portfolio_points = self.repo.portfolio_history_for_jobs(&job_ids).await?;

        let mut portfolio_map: HashMap<Uuid, Vec<LeaderboardPortfolioPoint>> = HashMap::new();
        for point in portfolio_points {
            portfolio_map
                .entry(point.backtest_job_id)
                .or_default()
                .push(LeaderboardPortfolioPoint {
                    date: point.date,
                    net_value: point.net_value,
                });
        }

        let mut entries = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "PENDING" => BacktestStatus::Pending,
                "PROCESSING" => BacktestStatus::Processing,
                "DONE" => BacktestStatus::Done,
                "FAILED" => BacktestStatus::Failed,
                _ => return Err(AppError::Internal),
            };

            let portfolio_history = portfolio_map.remove(&row.id).unwrap_or_default();

            entries.push(LeaderboardEntry {
                id: row.id,
                name: row.name,
                owner_name: row.owner_name,
                strategy_name: row.strategy_name,
                year: row.year,
                initial_cash: row.initial_cash,
                backtest_duration_months: row.backtest_duration_months,
                buy_fee_percentage: row.buy_fee_percentage,
                sell_fee_percentage: row.sell_fee_percentage,
                status,
                net_pnl: row.net_pnl,
                net_pnl_percentage: row.net_pnl_percentage,
                win_rate: row.win_rate,
                profit_factor: row.profit_factor,
                trades_processed: row.trades_processed,
                star_count: row.star_count,
                is_starred_by_me: row.is_starred_by_me,
                portfolio_history,
                created_at: row.created_at,
            });
        }

        Ok(entries)
    }
}
