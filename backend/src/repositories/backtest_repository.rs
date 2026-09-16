use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub use crate::entities::backtest::{CreateBacktestParams, MostTradedRow, TopEntryRow};

use crate::{
    entities::{
        app_error::AppError,
        backtest::{
            BacktestJobRecord, BacktestPortfolioHistoryRecord, BacktestResultRecord,
            BacktestTradeRecord,
        },
    },
    enums::backtest_status::BacktestStatus,
};

#[derive(Clone)]
pub struct BacktestRepository {
    pool: PgPool,
}

impl BacktestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        params: CreateBacktestParams,
    ) -> Result<BacktestJobRecord, AppError> {
        let record = sqlx::query_as::<_, BacktestJobRecord>(
            r#"
            INSERT INTO backtest_jobs (
                user_id, strategy_id, strategy_name, name, year, initial_cash, max_holding_stocks,
                max_stocks, backtest_duration_months, buy_fee_percentage, sell_fee_percentage, is_public, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
        )
        .bind(params.user_id)
        .bind(params.strategy_id)
        .bind(params.strategy_name)
        .bind(params.name)
        .bind(params.year)
        .bind(params.initial_cash)
        .bind(params.max_holding_stocks)
        .bind(params.max_stocks)
        .bind(params.backtest_duration_months)
        .bind(params.buy_fee_percentage)
        .bind(params.sell_fee_percentage)
        .bind(params.is_public)
        .bind(BacktestStatus::Pending)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("23505") {
                    return AppError::Conflict("Backtest with this name already exists".into());
                }
            }
            AppError::from(e)
        })?;

        Ok(record)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<BacktestJobRecord>, AppError> {
        let records = sqlx::query_as::<_, BacktestJobRecord>(
            "SELECT * FROM backtest_jobs WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn find_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<BacktestJobRecord, AppError> {
        let record = sqlx::query_as::<_, BacktestJobRecord>(
            "SELECT * FROM backtest_jobs WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Backtest not found".into()))?;

        Ok(record)
    }

    pub async fn find_by_id_for_user_or_public(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<BacktestJobRecord, AppError> {
        let record = sqlx::query_as::<_, BacktestJobRecord>(
            r#"
            SELECT 
                b.id,
                b.user_id,
                b.strategy_id,
                b.strategy_name,
                b.name,
                b.year,
                b.initial_cash,
                b.max_holding_stocks,
                b.max_stocks,
                b.backtest_duration_months,
                b.buy_fee_percentage,
                b.sell_fee_percentage,
                b.is_public,
                b.status,
                b.error_message,
                b.created_at,
                b.updated_at,
                u.name as owner_name,
                u.email as owner_email
            FROM backtest_jobs b
            JOIN users u ON b.user_id = u.id
            WHERE b.id = $1 AND (b.user_id = $2 OR b.is_public = true)
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Backtest not found".into()))?;

        Ok(record)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        status: BacktestStatus,
        error_message: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE backtest_jobs SET status = $1, error_message = $2, updated_at = $3 WHERE id = $4",
        )
        .bind(status)
        .bind(error_message)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn reset_for_rerun(&self, id: Uuid) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM backtest_trades WHERE backtest_job_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM backtest_portfolio_history WHERE backtest_job_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM backtest_results WHERE backtest_job_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE backtest_jobs SET status = $1, error_message = NULL, updated_at = $2 WHERE id = $3")
            .bind(BacktestStatus::Pending)
            .bind(Utc::now())
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn save_result(
        &self,
        job_id: Uuid,
        result: &BacktestResultRecord,
        portfolio: &[BacktestPortfolioHistoryRecord],
        trades: &[BacktestTradeRecord],
    ) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO backtest_results (
                backtest_job_id, available_cash, trades_processed, net_pnl, net_pnl_percentage,
                gross_pnl, gross_pnl_percentage, win_rate, profit_factor, wins, losses,
                sharpe_ratio, max_profit, max_profit_percentage, max_loss, max_loss_percentage,
                avg_profit, avg_profit_percentage, avg_loss, avg_loss_percentage, avg_hold_time_days,
                total_fees, avg_win_hold_days, avg_loss_hold_days, portfolio_volatility
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            "#,
        )
        .bind(job_id)
        .bind(result.available_cash)
        .bind(result.trades_processed)
        .bind(result.net_pnl)
        .bind(result.net_pnl_percentage)
        .bind(result.gross_pnl)
        .bind(result.gross_pnl_percentage)
        .bind(result.win_rate)
        .bind(result.profit_factor)
        .bind(result.wins)
        .bind(result.losses)
        .bind(result.sharpe_ratio)
        .bind(result.max_profit)
        .bind(result.max_profit_percentage)
        .bind(result.max_loss)
        .bind(result.max_loss_percentage)
        .bind(result.avg_profit)
        .bind(result.avg_profit_percentage)
        .bind(result.avg_loss)
        .bind(result.avg_loss_percentage)
        .bind(result.avg_hold_time_days)
        .bind(result.total_fees)
        .bind(result.avg_win_hold_days)
        .bind(result.avg_loss_hold_days)
        .bind(result.portfolio_volatility)
        .execute(&mut *tx)
        .await?;

        // Batch insert for portfolio
        // Since we shouldn't use macros, we can just loop, or use UNNEST if possible.
        // Looping in a transaction is fine since records might not be too huge. Or use QueryBuilder.
        // I will use sqlx::QueryBuilder.

        if !portfolio.is_empty() {
            let mut pb = sqlx::QueryBuilder::new(
                "INSERT INTO backtest_portfolio_history (backtest_job_id, date, net_value, gross_value) "
            );
            pb.push_values(portfolio, |mut b, p| {
                b.push_bind(job_id)
                    .push_bind(p.date)
                    .push_bind(p.net_value)
                    .push_bind(p.gross_value);
            });
            pb.build().execute(&mut *tx).await?;
        }

        if !trades.is_empty() {
            let mut tb = sqlx::QueryBuilder::new(
                r#"INSERT INTO backtest_trades (
                    backtest_job_id, code, pnl, pnl_percentage, exit_reason, lot, buy_price,
                    buy_value, sell_price, sell_value, buy_fee, sell_fee, buy_date, sell_date
                ) "#,
            );
            tb.push_values(trades, |mut b, t| {
                b.push_bind(job_id)
                    .push_bind(t.code.clone())
                    .push_bind(t.pnl)
                    .push_bind(t.pnl_percentage)
                    .push_bind(t.exit_reason.clone())
                    .push_bind(t.lot)
                    .push_bind(t.buy_price)
                    .push_bind(t.buy_value)
                    .push_bind(t.sell_price)
                    .push_bind(t.sell_value)
                    .push_bind(t.buy_fee)
                    .push_bind(t.sell_fee)
                    .push_bind(t.buy_date)
                    .push_bind(t.sell_date);
            });
            tb.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn find_result_by_job(
        &self,
        job_id: Uuid,
    ) -> Result<Option<BacktestResultRecord>, AppError> {
        let record = sqlx::query_as::<_, BacktestResultRecord>(
            "SELECT * FROM backtest_results WHERE backtest_job_id = $1",
        )
        .bind(job_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn update_ai_summary(
        &self,
        job_id: Uuid,
        summary: &[String],
    ) -> Result<(), AppError> {
        let json_val = serde_json::to_value(summary).unwrap_or_default();
        sqlx::query("UPDATE backtest_results SET ai_summary = $1 WHERE backtest_job_id = $2")
            .bind(json_val)
            .bind(job_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    pub async fn find_portfolio_history(
        &self,
        job_id: Uuid,
    ) -> Result<Vec<BacktestPortfolioHistoryRecord>, AppError> {
        let records = sqlx::query_as::<_, BacktestPortfolioHistoryRecord>(
            "SELECT * FROM backtest_portfolio_history WHERE backtest_job_id = $1 ORDER BY date ASC",
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn find_trades(&self, job_id: Uuid) -> Result<Vec<BacktestTradeRecord>, AppError> {
        let records = sqlx::query_as::<_, BacktestTradeRecord>(
            "SELECT * FROM backtest_trades WHERE backtest_job_id = $1 ORDER BY sell_date DESC",
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn find_most_traded(&self, job_id: Uuid) -> Result<Vec<MostTradedRow>, AppError> {
        let records = sqlx::query_as::<_, MostTradedRow>(
            r#"
            SELECT code, COUNT(*) as total, SUM(pnl) as pnl, 
            CASE WHEN SUM(buy_value + buy_fee) > 0 THEN SUM(pnl) / SUM(buy_value + buy_fee) * 100.0 ELSE 0 END as pnl_percentage 
            FROM backtest_trades 
            WHERE backtest_job_id = $1 
            GROUP BY code 
            ORDER BY total DESC, code ASC 
            LIMIT 6
            "#,
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn find_top_gainers(&self, job_id: Uuid) -> Result<Vec<TopEntryRow>, AppError> {
        let records = sqlx::query_as::<_, TopEntryRow>(
            r#"
            SELECT code, SUM(pnl) as pnl, 
            CASE WHEN SUM(buy_value + buy_fee) > 0 THEN SUM(pnl) / SUM(buy_value + buy_fee) * 100.0 ELSE 0 END as pnl_percentage 
            FROM backtest_trades 
            WHERE backtest_job_id = $1 
            GROUP BY code 
            HAVING SUM(pnl) > 0 
            ORDER BY pnl DESC 
            LIMIT 6
            "#,
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn find_top_losers(&self, job_id: Uuid) -> Result<Vec<TopEntryRow>, AppError> {
        let records = sqlx::query_as::<_, TopEntryRow>(
            r#"
            SELECT code, SUM(pnl) as pnl, 
            CASE WHEN SUM(buy_value + buy_fee) > 0 THEN SUM(pnl) / SUM(buy_value + buy_fee) * 100.0 ELSE 0 END as pnl_percentage 
            FROM backtest_trades 
            WHERE backtest_job_id = $1 
            GROUP BY code 
            HAVING SUM(pnl) < 0 
            ORDER BY pnl ASC 
            LIMIT 6
            "#,
        )
        .bind(job_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM backtest_jobs WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Backtest not found".into()));
        }

        Ok(())
    }

    pub async fn update_visibility(
        &self,
        id: Uuid,
        user_id: Uuid,
        is_public: bool,
    ) -> Result<BacktestJobRecord, AppError> {
        let record = sqlx::query_as::<_, BacktestJobRecord>(
            "UPDATE backtest_jobs
             SET is_public = $3, updated_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND user_id = $2
             RETURNING *",
        )
        .bind(id)
        .bind(user_id)
        .bind(is_public)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Backtest not found".into()))?;

        Ok(record)
    }
}
