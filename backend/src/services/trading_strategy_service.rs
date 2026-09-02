use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    clients::llm_client::LLMTrait,
    entities::{
        app_error::AppError,
        strategy_suggestion::{StrategyAiSuggestion, StrategyAiSuggestionsRequest},
        trading_strategy::{
            CreateTradingStrategyRequest, DuplicateTradingStrategyRequest, TradingStrategyResponse,
            UpdateTradingStrategyRequest,
        },
    },
    repositories::trading_strategy_repository::{
        CreateStrategyRecordParams, TradingStrategyRepository, UpdateStrategyRecordParams,
    },
    services::settings_service::SettingsService,
};

pub struct TradingStrategyService {
    strategies: Arc<TradingStrategyRepository>,
    llm: Arc<dyn LLMTrait>,
    settings: Arc<SettingsService>,
}

impl TradingStrategyService {
    pub fn new(
        strategies: Arc<TradingStrategyRepository>,
        llm: Arc<dyn LLMTrait>,
        settings: Arc<SettingsService>,
    ) -> Self {
        Self {
            strategies,
            llm,
            settings,
        }
    }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<TradingStrategyResponse>, AppError> {
        let records = self.strategies.list_by_user(user_id).await?;
        Ok(records
            .into_iter()
            .map(TradingStrategyResponse::from)
            .collect())
    }

    pub async fn get(&self, id: Uuid, user_id: Uuid) -> Result<TradingStrategyResponse, AppError> {
        let record = self
            .strategies
            .find_by_id_for_user_or_public(id, user_id)
            .await?;
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
        let rules = request.rules.unwrap_or_default();

        let record = self
            .strategies
            .create(CreateStrategyRecordParams {
                user_id,
                name,
                description,
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

    pub async fn get_ai_suggestions(
        &self,
        request: StrategyAiSuggestionsRequest,
    ) -> Result<Vec<StrategyAiSuggestion>, AppError> {
        request
            .validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        let ai_enabled = self.settings.get_ai_enabled().await.unwrap_or(false);
        if !ai_enabled {
            return Ok(vec![]);
        }

        let risk_reward_ratio = if request.sl_percentage > 0.0 {
            request.tp_percentage / request.sl_percentage
        } else {
            0.0
        };

        let description = request
            .description
            .as_deref()
            .unwrap_or("No description provided");

        let prompt = format!(
            r#"You are an expert quantitative trading strategist specializing in Indonesia Stock Exchange (IDX) equity trading.
Analyze the following trading strategy parameters:
- Strategy Name: "{name}"
- Description: "{desc}"
- Take Profit (TP): {tp:.2}%
- Stop Loss (SL): {sl:.2}%
- Risk-to-Reward Ratio: {rr:.2}:1
- Max Holding Period: {holding} days

Evaluate these parameters against Indonesia Stock Exchange (IDX) market realities (e.g. asymmetrical auto-rejection limit ARA/ARB, average swing duration of 5-20 days, and minimum 1:2 to 1:3 risk-to-reward ratio for positive expectancy).

Generate actionable parameter refinement suggestions.
Available fields you can suggest modifications for are ONLY:
- "tp_percentage" (Take Profit percentage, e.g. 8.0 to 30.0)
- "sl_percentage" (Stop Loss percentage, e.g. 2.5 to 8.0)
- "max_holding_period_days" (Max holding period in days, e.g. 5 to 60)

Format the output strictly as a JSON array of suggestions (maximum 3 items):
[
  {{
    "id": "suggestion-1",
    "field": "sl_percentage",
    "title": "Optimize Risk-Reward Ratio",
    "current_value": {sl:.2},
    "suggested_value": 3.5,
    "reason": "Lowering stop loss from {sl:.2}% to 3.5% elevates your Risk:Reward ratio from {rr:.2} to 2.86:1, improving capital preservation during market pullbacks."
  }}
]

If the strategy parameters are already completely optimal, return an empty array [].
Return ONLY valid JSON."#,
            name = request.name,
            desc = description,
            tp = request.tp_percentage,
            sl = request.sl_percentage,
            rr = risk_reward_ratio,
            holding = request.max_holding_period_days,
        );

        let suggestions: Result<Vec<StrategyAiSuggestion>, _> =
            crate::clients::llm_client::generate_structured(&*self.llm, &prompt).await;

        match suggestions {
            Ok(sug) => {
                let valid_fields = ["tp_percentage", "sl_percentage", "max_holding_period_days"];
                let filtered: Vec<StrategyAiSuggestion> = sug
                    .into_iter()
                    .filter(|s| valid_fields.contains(&s.field.as_str()) && s.suggested_value > 0.0)
                    .take(3)
                    .collect();
                Ok(filtered)
            }
            Err(e) => {
                eprintln!(
                    "[TradingStrategyService] Failed to generate AI suggestions: {:?}",
                    e
                );
                // Return empty list on AI failure so user is not blocked
                Ok(vec![])
            }
        }
    }
}
