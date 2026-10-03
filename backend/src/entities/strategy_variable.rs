use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyVariableDef {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    #[serde(rename = "is_historical", alias = "isHistorical")]
    pub is_historical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyOperatorDef {
    pub value: &'static str,
    pub symbol: &'static str,
    pub label: &'static str,
    pub display: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyVariable {
    pub code: String,
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(rename = "is_historical", alias = "isHistorical")]
    pub is_historical: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyOperator {
    pub value: String,
    pub symbol: String,
    pub label: String,
    pub display: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyMetadataResponse {
    pub variables: Vec<StrategyVariable>,
    pub categories: Vec<String>,
    pub operators: Vec<StrategyOperator>,
}

impl From<StrategyVariableDef> for StrategyVariable {
    fn from(def: StrategyVariableDef) -> Self {
        Self {
            code: def.code.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            category: def.category.to_string(),
            is_historical: def.is_historical,
        }
    }
}

impl From<&StrategyVariableDef> for StrategyVariable {
    fn from(def: &StrategyVariableDef) -> Self {
        Self {
            code: def.code.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            category: def.category.to_string(),
            is_historical: def.is_historical,
        }
    }
}

impl From<StrategyOperatorDef> for StrategyOperator {
    fn from(def: StrategyOperatorDef) -> Self {
        Self {
            value: def.value.to_string(),
            symbol: def.symbol.to_string(),
            label: def.label.to_string(),
            display: def.display.to_string(),
        }
    }
}

impl From<&StrategyOperatorDef> for StrategyOperator {
    fn from(def: &StrategyOperatorDef) -> Self {
        Self {
            value: def.value.to_string(),
            symbol: def.symbol.to_string(),
            label: def.label.to_string(),
            display: def.display.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_strategy_variable_with_is_historical() {
        let var = StrategyVariable {
            code: "pe".to_string(),
            name: "P/E Ratio".to_string(),
            description: "Price-to-earnings ratio.".to_string(),
            category: "Valuation Ratios".to_string(),
            is_historical: true,
        };

        let json = serde_json::to_string(&var).expect("serialize");
        assert!(json.contains(r#""is_historical":true"#));

        let deserialized: StrategyVariable = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized, var);

        let camel_json = r#"{
            "code": "pe",
            "name": "P/E Ratio",
            "description": "Price-to-earnings ratio.",
            "category": "Valuation Ratios",
            "isHistorical": true
        }"#;
        let from_camel: StrategyVariable =
            serde_json::from_str(camel_json).expect("deserialize camel");
        assert!(from_camel.is_historical);
    }

    #[test]
    fn should_convert_strategy_variable_def_to_strategy_variable() {
        let def = StrategyVariableDef {
            code: "pb",
            name: "P/B Ratio",
            description: "Price-to-book ratio.",
            category: "Valuation Ratios",
            is_historical: true,
        };

        let var = StrategyVariable::from(def);
        assert_eq!(var.code, "pb");
        assert_eq!(var.name, "P/B Ratio");
        assert!(var.is_historical);

        let var_ref = StrategyVariable::from(&def);
        assert_eq!(var_ref, var);
    }

    #[test]
    fn should_convert_strategy_operator_def_to_strategy_operator() {
        let def = StrategyOperatorDef {
            value: ">=",
            symbol: "[>=]",
            label: "greater than or equals",
            display: "[>=] greater than or equals",
        };

        let op = StrategyOperator::from(def);
        assert_eq!(op.value, ">=");
        assert_eq!(op.symbol, "[>=]");

        let op_ref = StrategyOperator::from(&def);
        assert_eq!(op_ref, op);
    }

    #[test]
    fn should_serialize_and_deserialize_strategy_metadata_response() {
        let meta = StrategyMetadataResponse {
            variables: vec![StrategyVariable {
                code: "price".to_string(),
                name: "Price".to_string(),
                description: "Closing market price.".to_string(),
                category: "Price & Market".to_string(),
                is_historical: false,
            }],
            categories: vec!["Price & Market".to_string()],
            operators: vec![StrategyOperator {
                value: ">".to_string(),
                symbol: "[>]".to_string(),
                label: "greater than".to_string(),
                display: "[>] greater than".to_string(),
            }],
        };

        let json = serde_json::to_string(&meta).expect("serialize");
        let deserialized: StrategyMetadataResponse =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized, meta);
    }
}
