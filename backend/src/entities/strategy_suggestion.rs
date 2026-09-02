use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyAiSuggestion {
    pub id: String,
    pub field: String,
    pub title: String,
    pub current_value: f64,
    pub suggested_value: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct StrategyAiSuggestionsRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0.0001, max = 10000.0))]
    pub tp_percentage: f64,
    #[validate(range(min = 0.0001, max = 10000.0))]
    pub sl_percentage: f64,
    #[validate(range(min = 1, max = 3650))]
    pub max_holding_period_days: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_valid_strategy_ai_suggestions_request() {
        let valid = StrategyAiSuggestionsRequest {
            name: "Momentum Breakout".to_string(),
            description: Some("Test description".to_string()),
            tp_percentage: 12.5,
            sl_percentage: 4.5,
            max_holding_period_days: 20,
        };
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn should_reject_invalid_strategy_ai_suggestions_request() {
        let invalid_tp = StrategyAiSuggestionsRequest {
            name: "Test".to_string(),
            description: None,
            tp_percentage: -1.0,
            sl_percentage: 5.0,
            max_holding_period_days: 10,
        };
        assert!(invalid_tp.validate().is_err());

        let invalid_name = StrategyAiSuggestionsRequest {
            name: "".to_string(),
            description: None,
            tp_percentage: 10.0,
            sl_percentage: 5.0,
            max_holding_period_days: 10,
        };
        assert!(invalid_name.validate().is_err());
    }
}
