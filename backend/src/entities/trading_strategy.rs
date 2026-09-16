use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StrategyRuleCondition {
    pub id: String,
    pub variable: String,
    pub operator: String,
    pub value: String,
    pub connector_to_next: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StrategyRuleGroup {
    pub id: String,
    pub connector_to_next: Option<String>,
    pub conditions: Vec<StrategyRuleCondition>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StrategyOwnerResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct TradingStrategyRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tp_percentage: f64,
    pub sl_percentage: f64,
    pub max_holding_period_days: i32,
    pub rules: Json<Vec<StrategyRuleGroup>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(default)]
    pub owner_name: Option<String>,
    #[sqlx(default)]
    pub owner_email: Option<String>,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingStrategyResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tp_percentage: f64,
    pub sl_percentage: f64,
    pub risk_reward_ratio: Option<f64>,
    pub max_holding_period_days: i32,
    pub rules: Vec<StrategyRuleGroup>,
    pub owner: Option<StrategyOwnerResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TradingStrategyRecord> for TradingStrategyResponse {
    fn from(record: TradingStrategyRecord) -> Self {
        let risk_reward_ratio = if record.sl_percentage.abs() > f64::EPSILON {
            Some(record.tp_percentage / record.sl_percentage)
        } else {
            None
        };

        let owner = match (record.owner_name, record.owner_email) {
            (Some(name), Some(email)) => Some(StrategyOwnerResponse {
                id: record.user_id,
                name,
                email,
            }),
            _ => None,
        };

        Self {
            id: record.id,
            user_id: record.user_id,
            name: record.name,
            description: record.description,
            tp_percentage: record.tp_percentage,
            sl_percentage: record.sl_percentage,
            risk_reward_ratio,
            max_holding_period_days: record.max_holding_period_days,
            rules: record.rules.0,
            owner,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTradingStrategyRequest {
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

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTradingStrategyRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(range(min = 0.0001, max = 10000.0))]
    pub tp_percentage: Option<f64>,
    #[validate(range(min = 0.0001, max = 10000.0))]
    pub sl_percentage: Option<f64>,
    #[validate(range(min = 1, max = 3650))]
    pub max_holding_period_days: Option<i32>,
    pub rules: Option<Vec<StrategyRuleGroup>>,
}

impl UpdateTradingStrategyRequest {
    pub const fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.tp_percentage.is_none()
            && self.sl_percentage.is_none()
            && self.max_holding_period_days.is_none()
            && self.rules.is_none()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct DuplicateTradingStrategyRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_risk_reward_ratio() {
        let record = TradingStrategyRecord {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Momentum Alpha".to_string(),
            description: Some("Test strategy".to_string()),
            tp_percentage: 10.0,
            sl_percentage: 5.0,
            max_holding_period_days: 30,
            rules: Json(vec![]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            owner_name: None,
            owner_email: None,
        };

        let response = TradingStrategyResponse::from(record);
        assert_eq!(response.risk_reward_ratio, Some(2.0));
        assert_eq!(response.name, "Momentum Alpha");
        assert_eq!(response.max_holding_period_days, 30);
    }

    #[test]
    fn should_validate_create_strategy_request() {
        let valid = CreateTradingStrategyRequest {
            name: "Valid Strategy".to_string(),
            description: None,
            tp_percentage: 15.0,
            sl_percentage: 5.0,
            max_holding_period_days: 10,
            rules: Some(vec![]),
        };
        assert!(valid.validate().is_ok());

        let invalid = CreateTradingStrategyRequest {
            name: "".to_string(),
            description: None,
            tp_percentage: -1.0,
            sl_percentage: 0.0,
            max_holding_period_days: 0,
            rules: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn should_detect_empty_update_request() {
        let empty = UpdateTradingStrategyRequest {
            name: None,
            description: None,
            tp_percentage: None,
            sl_percentage: None,
            max_holding_period_days: None,
            rules: None,
        };
        assert!(empty.is_empty());

        let non_empty = UpdateTradingStrategyRequest {
            name: Some("New Name".to_string()),
            description: None,
            tp_percentage: None,
            sl_percentage: None,
            max_holding_period_days: None,
            rules: None,
        };
        assert!(!non_empty.is_empty());
    }
}
