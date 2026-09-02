use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::trading_strategy::StrategyRuleGroup;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyRulePayload {
    pub group_index: usize,
    pub condition_index: Option<usize>,
    pub variable: String,
    pub operator: String,
    pub value: String,
    pub connector_to_next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyAiSuggestion {
    pub id: String,
    #[serde(default = "default_suggestion_type")]
    pub suggestion_type: String,
    pub field: Option<String>,
    pub current_value: Option<f64>,
    pub suggested_value: Option<f64>,
    pub rule_action: Option<String>,
    pub rule_payload: Option<StrategyRulePayload>,
    pub title: String,
    pub reason: String,
}

fn default_suggestion_type() -> String {
    "PARAMETER".to_string()
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
    pub rules: Option<Vec<StrategyRuleGroup>>,
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
            rules: Some(vec![]),
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
            rules: None,
        };
        assert!(invalid_tp.validate().is_err());

        let invalid_name = StrategyAiSuggestionsRequest {
            name: "".to_string(),
            description: None,
            tp_percentage: 10.0,
            sl_percentage: 5.0,
            max_holding_period_days: 10,
            rules: None,
        };
        assert!(invalid_name.validate().is_err());
    }

    #[test]
    fn should_deserialize_parameter_and_rule_suggestions() {
        let json_data = r#"[
            {
                "id": "sug-1",
                "suggestion_type": "PARAMETER",
                "field": "sl_percentage",
                "current_value": 5.0,
                "suggested_value": 3.5,
                "title": "Optimize Risk-Reward",
                "reason": "Improves capital preservation"
            },
            {
                "id": "sug-2",
                "suggestion_type": "RULE",
                "rule_action": "ADD_CONDITION",
                "rule_payload": {
                    "group_index": 0,
                    "condition_index": null,
                    "variable": "market_cap",
                    "operator": ">=",
                    "value": "1000000000000",
                    "connector_to_next": "AND"
                },
                "title": "Filter Micro-Caps",
                "reason": "Avoid illiquid stocks"
            }
        ]"#;

        let suggestions: Vec<StrategyAiSuggestion> =
            serde_json::from_str(json_data).expect("Should deserialize correctly");
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].suggestion_type, "PARAMETER");
        assert_eq!(suggestions[0].field.as_deref(), Some("sl_percentage"));
        assert_eq!(suggestions[0].suggested_value, Some(3.5));

        assert_eq!(suggestions[1].suggestion_type, "RULE");
        assert_eq!(suggestions[1].rule_action.as_deref(), Some("ADD_CONDITION"));
        let payload = suggestions[1].rule_payload.as_ref().unwrap();
        assert_eq!(payload.variable, "market_cap");
        assert_eq!(payload.operator, ">=");
        assert_eq!(payload.value, "1000000000000");
    }
}
