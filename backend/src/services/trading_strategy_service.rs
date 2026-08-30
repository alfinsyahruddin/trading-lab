use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    entities::{
        app_error::AppError,
        trading_strategy::{
            CreateTradingStrategyRequest, DuplicateTradingStrategyRequest, TradingStrategyResponse,
            UpdateTradingStrategyRequest,
        },
    },
    repositories::trading_strategy_repository::{
        CreateStrategyRecordParams, TradingStrategyRepository, UpdateStrategyRecordParams,
    },
};

pub struct TradingStrategyService {
    strategies: Arc<TradingStrategyRepository>,
}

impl TradingStrategyService {
    pub fn new(strategies: Arc<TradingStrategyRepository>) -> Self {
        Self { strategies }
    }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<TradingStrategyResponse>, AppError> {
        let records = self.strategies.list_by_user(user_id).await?;
        Ok(records
            .into_iter()
            .map(TradingStrategyResponse::from)
            .collect())
    }

    pub async fn get(&self, id: Uuid, user_id: Uuid) -> Result<TradingStrategyResponse, AppError> {
        let record = self.strategies.find_by_id_and_user(id, user_id).await?;
        Ok(TradingStrategyResponse::from(record))
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreateTradingStrategyRequest,
    ) -> Result<TradingStrategyResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        let name = request.name.trim();
        if name.is_empty() {
            return Err(AppError::bad_request("Strategy name cannot be empty"));
        }

        let description = request
            .description
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let is_public = request.is_public.unwrap_or(false);
        let rules = request.rules.unwrap_or_default();

        let record = self
            .strategies
            .create(CreateStrategyRecordParams {
                user_id,
                name,
                description,
                is_public,
                tp_percentage: request.tp_percentage,
                sl_percentage: request.sl_percentage,
                max_holding_period_days: request.max_holding_period_days,
                rules: &rules,
            })
            .await?;

        Ok(TradingStrategyResponse::from(record))
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateTradingStrategyRequest,
    ) -> Result<TradingStrategyResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        if request.is_empty() {
            return Err(AppError::bad_request("No fields provided for update"));
        }

        let name = request.name.as_deref().map(str::trim);
        if let Some(n) = name {
            if n.is_empty() {
                return Err(AppError::bad_request("Strategy name cannot be empty"));
            }
        }

        let description = request.description.as_deref().map(|d| {
            let trimmed = d.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });

        let record = self
            .strategies
            .update(UpdateStrategyRecordParams {
                id,
                user_id,
                name,
                description,
                is_public: request.is_public,
                tp_percentage: request.tp_percentage,
                sl_percentage: request.sl_percentage,
                max_holding_period_days: request.max_holding_period_days,
                rules: request.rules.as_deref(),
            })
            .await?;

        Ok(TradingStrategyResponse::from(record))
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.strategies.delete(id, user_id).await?;
        Ok(())
    }

    pub async fn duplicate(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: DuplicateTradingStrategyRequest,
    ) -> Result<TradingStrategyResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        let name = request.name.trim();
        if name.is_empty() {
            return Err(AppError::bad_request("Strategy name cannot be empty"));
        }

        let record = self.strategies.duplicate(id, user_id, name).await?;
        Ok(TradingStrategyResponse::from(record))
    }
}
