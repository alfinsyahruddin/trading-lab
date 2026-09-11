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

        let rules_formatted = match &request.rules {
            Some(groups) if !groups.is_empty() => {
                let mut lines = Vec::new();
                for (g_idx, group) in groups.iter().enumerate() {
                    let mut cond_strs = Vec::new();
                    for (c_idx, c) in group.conditions.iter().enumerate() {
                        let conn = c.connector_to_next.as_deref().unwrap_or("AND");
                        cond_strs.push(format!(
                            "[Condition {}]: {} {} {} (next: {})",
                            c_idx, c.variable, c.operator, c.value, conn
                        ));
                    }
                    let grp_conn = group.connector_to_next.as_deref().unwrap_or("AND");
                    lines.push(format!(
                        "Group {} (next: {}):\n  {}",
                        g_idx,
                        grp_conn,
                        cond_strs.join("\n  ")
                    ));
                }
                lines.join("\n")
            }
            _ => "No conditional rules defined.".to_string(),
        };

        let prompt = format!(
            r#"You are an expert quantitative trading strategist specializing in Indonesia Stock Exchange (IDX) equity trading.
Analyze the following trading strategy parameters and where condition rules:
- Strategy Name: "{name}"
- Description: "{desc}"
- Take Profit (TP): {tp:.2}%
- Stop Loss (SL): {sl:.2}%
- Risk-to-Reward Ratio: {rr:.2}:1
- Max Holding Period: {holding} days

Current Where Condition Rules:
{rules_str}

Available Screener Variables for Rules:
- Price & Market: "price", "volume", "value", "market_cap", "shares_outstanding"
- Valuation Ratios: "pe", "pb", "ps", "pcf", "peg", "enterprise_to_ebitda", "enterprise_to_revenue"
- Profitability & Returns: "net_profit_margin", "gross_profit_margin", "operating_profit_margin", "roe", "roa", "roce"
- Dividends: "dividend_yield", "dividend_payout_ratio", "dps"
- Financial Health: "der", "debt_to_equity", "debt_to_asset", "current_ratio", "quick_ratio", "total_debt"
- Cash Flow & Growth: "free_cash_flow", "operating_cash_flow", "revenue_growth_yoy", "net_profit_growth_yoy"
- Classification: "sector", "sub_sector"

Available Rule Operators: "=", "!=", ">", "<", ">=", "<=", "~~", "in", "is"

Evaluate these parameters and rules against Indonesia Stock Exchange (IDX) realities:
1. Risk Parameters: Ensure TP/SL ratio provides a positive expectancy (at least 1:2 to 1:3 R:R), SL accounts for IDX volatility without premature stop-outs, and holding period aligns with swing/trend duration.
2. Screener Rules: Recommend adding or adjusting rules to filter out low-liquidity penny stocks (e.g. market_cap >= 1000000000000, price > 100 or 200), apply fundamental quality filters (e.g. roe > 10, pe < 20, der < 2.0), or align with the strategy's stated theme/description.

Generate actionable parameter AND rule refinement suggestions.
You may return up to 4 suggestions total (a mix of parameter adjustments and rule additions/edits).

Format the output strictly as a JSON array matching this schema:
[
  {{
    "id": "suggestion-1",
    "suggestion_type": "PARAMETER",
    "field": "sl_percentage",
    "current_value": {sl:.2},
    "suggested_value": 3.5,
    "rule_action": null,
    "rule_payload": null,
    "title": "Optimize Risk-Reward Ratio",
    "reason": "Lowering stop loss from {sl:.2}% to 3.5% elevates your Risk:Reward ratio from {rr:.2} to 2.86:1, improving capital preservation."
  }},
  {{
    "id": "suggestion-2",
    "suggestion_type": "RULE",
    "field": null,
    "current_value": null,
    "suggested_value": null,
    "rule_action": "ADD_CONDITION",
    "rule_payload": {{
      "group_index": 0,
      "condition_index": null,
      "variable": "market_cap",
      "operator": ">=",
      "value": "1000000000000",
      "connector_to_next": "AND"
    }},
    "title": "Filter Out Illiquid Micro-Caps",
    "reason": "Adding market_cap >= 1T IDR prevents screening illiquid small-cap stocks on IDX that are vulnerable to extreme slippage."
  }},
  {{
    "id": "suggestion-3",
    "suggestion_type": "RULE",
    "field": null,
    "current_value": null,
    "suggested_value": null,
    "rule_action": "EDIT_CONDITION",
    "rule_payload": {{
      "group_index": 0,
      "condition_index": 0,
      "variable": "price",
      "operator": ">",
      "value": "200",
      "connector_to_next": "AND"
    }},
    "title": "Avoid Penny Stock Price Band",
    "reason": "Raising minimum price from 50 to 200 avoids FCA (Full Call Auction) watchlist stocks on the IDX."
  }}
]

If the strategy parameters and rules are already completely optimal, return an empty array [].
Return ONLY valid JSON."#,
            name = request.name,
            desc = description,
            tp = request.tp_percentage,
            sl = request.sl_percentage,
            rr = risk_reward_ratio,
            holding = request.max_holding_period_days,
            rules_str = rules_formatted,
        );

        let suggestions: Result<Vec<StrategyAiSuggestion>, _> =
            crate::clients::llm_client::generate_structured(&*self.llm, &prompt).await;

        match suggestions {
            Ok(sug) => {
                let valid_fields = ["tp_percentage", "sl_percentage", "max_holding_period_days"];
                let valid_variables = [
                    "price",
                    "volume",
                    "value",
                    "market_cap",
                    "shares_outstanding",
                    "pe",
                    "pb",
                    "ps",
                    "pcf",
                    "peg",
                    "enterprise_to_ebitda",
                    "enterprise_to_revenue",
                    "pb_peer_avg",
                    "pe_peer_avg",
                    "ps_peer_avg",
                    "revenue",
                    "cost_of_revenue",
                    "gross_profit",
                    "operating_expense",
                    "operating_pnl",
                    "ebit",
                    "ebitda",
                    "earnings_before_tax",
                    "tax",
                    "earnings",
                    "non_operating_income_or_loss",
                    "net_profit_margin",
                    "gross_profit_margin",
                    "operating_profit_margin",
                    "roe",
                    "roa",
                    "roce",
                    "dividend",
                    "dividend_yield",
                    "dividend_payout_ratio",
                    "dps",
                    "eps",
                    "bps",
                    "operating_cash_flow",
                    "investing_cash_flow",
                    "financing_cash_flow",
                    "net_cash_flow",
                    "free_cash_flow",
                    "total_assets",
                    "current_assets",
                    "cash_and_equivalents",
                    "inventory",
                    "total_equity",
                    "total_liabilities",
                    "current_liabilities",
                    "total_debt",
                    "short_term_debt",
                    "long_term_debt",
                    "der",
                    "debt_to_equity",
                    "debt_to_asset",
                    "current_ratio",
                    "quick_ratio",
                    "interest_coverage_ratio",
                    "revenue_growth_yoy",
                    "net_profit_growth_yoy",
                    "operating_profit_growth_yoy",
                    "eps_growth_yoy",
                    "sector",
                    "sub_sector",
                ];
                let valid_operators = ["=", "!=", ">", "<", ">=", "<=", "~~", "in", "is"];

                let filtered: Vec<StrategyAiSuggestion> = sug
                    .into_iter()
                    .filter(|s| {
                        if s.suggestion_type.eq_ignore_ascii_case("RULE") {
                            if let Some(ref payload) = s.rule_payload {
                                valid_variables.contains(&payload.variable.as_str())
                                    && valid_operators.contains(&payload.operator.as_str())
                                    && !payload.value.trim().is_empty()
                            } else {
                                false
                            }
                        } else {
                            // Default to PARAMETER
                            if let Some(ref field) = s.field {
                                valid_fields.contains(&field.as_str())
                                    && s.suggested_value.map(|v| v > 0.0).unwrap_or(false)
                            } else {
                                false
                            }
                        }
                    })
                    .take(4)
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
