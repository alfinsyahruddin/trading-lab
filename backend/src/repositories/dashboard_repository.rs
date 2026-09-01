use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::entities::app_error::AppError;

pub struct DashboardRepository {
    pool: PgPool,
}

#[derive(FromRow)]
pub struct StatsRow {
    pub total_stars_received: i64,
    pub total_strategies: i64,
    pub total_backtests: i64,
}

#[derive(FromRow)]
pub struct LeaderboardRow {
    pub id: Uuid,
    pub name: String,
    pub owner_name: String,
    pub strategy_name: String,
    pub year: i32,
    pub initial_cash: f64,
    pub backtest_duration_months: i32,
    pub buy_fee_percentage: f64,
    pub sell_fee_percentage: f64,
    pub status: String,
    pub net_pnl: f64,
    pub net_pnl_percentage: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub trades_processed: i32,
    pub star_count: i64,
    pub is_starred_by_me: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct PortfolioPointRow {
    pub backtest_job_id: Uuid,
    pub date: NaiveDate,
    pub net_value: f64,
}

impl DashboardRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<StatsRow, AppError> {
        let stats = sqlx::query_as::<_, StatsRow>(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM backtest_stars s JOIN backtest_jobs j ON s.backtest_job_id = j.id WHERE j.user_id = $1) as total_stars_received,
                (SELECT COUNT(*) FROM trading_strategies WHERE user_id = $1) as total_strategies,
                (SELECT COUNT(*) FROM backtest_jobs WHERE user_id = $1) as total_backtests
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(stats)
    }

    pub async fn leaderboard_by_percentage(
        &self,
        viewer_id: Uuid,
    ) -> Result<Vec<LeaderboardRow>, AppError> {
        let rows = sqlx::query_as::<_, LeaderboardRow>(
            r#"
            SELECT 
                b.id,
                b.name,
                u.name as owner_name,
                b.strategy_name,
                b.year,
                b.initial_cash,
                b.backtest_duration_months,
                b.buy_fee_percentage,
                b.sell_fee_percentage,
                b.status::text as status,
                r.net_pnl,
                r.net_pnl_percentage,
                r.win_rate,
                r.profit_factor,
                r.trades_processed,
                (SELECT COUNT(*) FROM backtest_stars WHERE backtest_job_id = b.id) as star_count,
                EXISTS(SELECT 1 FROM backtest_stars WHERE backtest_job_id = b.id AND user_id = $1) as is_starred_by_me,
                b.created_at
            FROM backtest_jobs b
            JOIN users u ON b.user_id = u.id
            JOIN backtest_results r ON b.id = r.backtest_job_id
            WHERE b.is_public = true AND b.status = 'DONE'
            ORDER BY r.net_pnl_percentage DESC
            LIMIT 10
            "#,
        )
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(rows)
    }

    pub async fn leaderboard_by_stars(
        &self,
        viewer_id: Uuid,
    ) -> Result<Vec<LeaderboardRow>, AppError> {
        let rows = sqlx::query_as::<_, LeaderboardRow>(
            r#"
            SELECT 
                b.id,
                b.name,
                u.name as owner_name,
                b.strategy_name,
                b.year,
                b.initial_cash,
                b.backtest_duration_months,
                b.buy_fee_percentage,
                b.sell_fee_percentage,
                b.status::text as status,
                r.net_pnl,
                r.net_pnl_percentage,
                r.win_rate,
                r.profit_factor,
                r.trades_processed,
                (SELECT COUNT(*) FROM backtest_stars WHERE backtest_job_id = b.id) as star_count,
                EXISTS(SELECT 1 FROM backtest_stars WHERE backtest_job_id = b.id AND user_id = $1) as is_starred_by_me,
                b.created_at
            FROM backtest_jobs b
            JOIN users u ON b.user_id = u.id
            JOIN backtest_results r ON b.id = r.backtest_job_id
            WHERE b.is_public = true AND b.status = 'DONE'
            ORDER BY star_count DESC, r.net_pnl_percentage DESC
            LIMIT 10
            "#,
        )
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(rows)
    }

    pub async fn portfolio_history_for_jobs(
        &self,
        job_ids: &[Uuid],
    ) -> Result<Vec<PortfolioPointRow>, AppError> {
        let rows = sqlx::query_as::<_, PortfolioPointRow>(
            r#"
            SELECT 
                backtest_job_id,
                date,
                net_value
            FROM backtest_portfolio_history
            WHERE backtest_job_id = ANY($1)
            ORDER BY backtest_job_id, date
            "#,
        )
        .bind(job_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(rows)
    }

    pub async fn add_star(&self, user_id: Uuid, backtest_job_id: Uuid) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::from)?;

        let job_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM backtest_jobs WHERE id = $1 AND is_public = true)",
        )
        .bind(backtest_job_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::from)?;

        if !job_exists {
            return Err(AppError::not_found(
                "Backtest not found or not public".to_string(),
            ));
        }

        sqlx::query(
            "INSERT INTO backtest_stars (user_id, backtest_job_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(user_id)
        .bind(backtest_job_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::from)?;

        tx.commit().await.map_err(AppError::from)?;

        Ok(())
    }

    pub async fn remove_star(&self, user_id: Uuid, backtest_job_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM backtest_stars WHERE user_id = $1 AND backtest_job_id = $2")
            .bind(user_id)
            .bind(backtest_job_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }
}
