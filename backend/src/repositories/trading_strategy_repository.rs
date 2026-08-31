use sqlx::{postgres::PgQueryResult, types::Json, PgPool};
use uuid::Uuid;

use crate::entities::{
    app_error::AppError,
    trading_strategy::{StrategyRuleGroup, TradingStrategyRecord},
};

pub struct CreateStrategyRecordParams<'a> {
    pub user_id: Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub tp_percentage: f64,
    pub sl_percentage: f64,
    pub max_holding_period_days: i32,
    pub rules: &'a [StrategyRuleGroup],
}

pub struct UpdateStrategyRecordParams<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub tp_percentage: Option<f64>,
    pub sl_percentage: Option<f64>,
    pub max_holding_period_days: Option<i32>,
    pub rules: Option<&'a [StrategyRuleGroup]>,
}

#[derive(Clone)]
pub struct TradingStrategyRepository {
    db: PgPool,
}

impl TradingStrategyRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        params: CreateStrategyRecordParams<'_>,
    ) -> Result<TradingStrategyRecord, AppError> {
        let rules_json = Json(params.rules.to_vec());
        sqlx::query_as::<_, TradingStrategyRecord>(
            "INSERT INTO trading_strategies (user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules, created_at, updated_at"
        )
        .bind(params.user_id)
        .bind(params.name)
        .bind(params.description)
        .bind(params.tp_percentage)
        .bind(params.sl_percentage)
        .bind(params.max_holding_period_days)
        .bind(rules_json)
        .fetch_one(&self.db)
        .await
        .map_err(map_database_error)
    }

    pub async fn list_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<TradingStrategyRecord>, AppError> {
        sqlx::query_as::<_, TradingStrategyRecord>(
            "SELECT id, user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules, created_at, updated_at
             FROM trading_strategies
             WHERE user_id = $1
             ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id_and_user(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<TradingStrategyRecord, AppError> {
        sqlx::query_as::<_, TradingStrategyRecord>(
            "SELECT id, user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules, created_at, updated_at
             FROM trading_strategies
             WHERE id = $1 AND user_id = $2"
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::not_found("Trading strategy not found"))
    }

    pub async fn update(
        &self,
        params: UpdateStrategyRecordParams<'_>,
    ) -> Result<TradingStrategyRecord, AppError> {
        let rules_json = params.rules.map(|r| Json(r.to_vec()));
        sqlx::query_as::<_, TradingStrategyRecord>(
            "UPDATE trading_strategies
             SET name = COALESCE($3, name),
                 description = CASE WHEN $4 THEN $5 ELSE description END,
                 tp_percentage = COALESCE($6, tp_percentage),
                 sl_percentage = COALESCE($7, sl_percentage),
                 max_holding_period_days = COALESCE($8, max_holding_period_days),
                 rules = COALESCE($9, rules),
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $1 AND user_id = $2
             RETURNING id, user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules, created_at, updated_at"
        )
        .bind(params.id)
        .bind(params.user_id)
        .bind(params.name)
        .bind(params.description.is_some())
        .bind(params.description.flatten())
        .bind(params.tp_percentage)
        .bind(params.sl_percentage)
        .bind(params.max_holding_period_days)
        .bind(rules_json)
        .fetch_optional(&self.db)
        .await
        .map_err(map_database_error)?
        .ok_or_else(|| AppError::not_found("Trading strategy not found"))
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<PgQueryResult, AppError> {
        let result = sqlx::query("DELETE FROM trading_strategies WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.db)
            .await
            .map_err(AppError::from)?;

        if result.rows_affected() == 0 {
            return Err(AppError::not_found("Trading strategy not found"));
        }

        Ok(result)
    }

    pub async fn duplicate(
        &self,
        id: Uuid,
        user_id: Uuid,
        new_name: &str,
    ) -> Result<TradingStrategyRecord, AppError> {
        let source = self.find_by_id_and_user(id, user_id).await?;

        sqlx::query_as::<_, TradingStrategyRecord>(
            "INSERT INTO trading_strategies (user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, user_id, name, description, tp_percentage, sl_percentage, max_holding_period_days, rules, created_at, updated_at"
        )
        .bind(user_id)
        .bind(new_name)
        .bind(source.description)
        .bind(source.tp_percentage)
        .bind(source.sl_percentage)
        .bind(source.max_holding_period_days)
        .bind(source.rules)
        .fetch_one(&self.db)
        .await
        .map_err(map_database_error)
    }
}

fn map_database_error(error: sqlx::Error) -> AppError {
    match &error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23505") =>
        {
            AppError::conflict("A strategy with this name already exists")
        }
        _ => AppError::from(error),
    }
}
