use crate::entities::trading_strategy::{StrategyRuleCondition, StrategyRuleGroup};

pub fn is_foreign_flow_variable(var: &str) -> bool {
    let v = var.trim();
    v.eq_ignore_ascii_case("last_1_week_foreign_flow")
        || v.eq_ignore_ascii_case("last_1_month_foreign_flow")
        || v.eq_ignore_ascii_case("last_3_months_foreign_flow")
}

pub fn build_where_query(
    groups: &[StrategyRuleGroup],
    year: i32,
) -> (String, Vec<StrategyRuleGroup>, Vec<StrategyRuleGroup>) {
    let mut valid_group_clauses = Vec::new();
    let mut manual_filter_groups = Vec::new();
    let mut foreign_flow_filter_groups = Vec::new();

    for group in groups {
        let mut condition_parts: Vec<(String, Option<String>)> = Vec::new();
        let mut manual_conditions = Vec::new();
        let mut foreign_flow_conditions = Vec::new();

        for cond in &group.conditions {
            let var_trimmed = cond.variable.trim();
            let is_foreign_flow = is_foreign_flow_variable(var_trimmed);
            let is_volume_or_value = var_trimmed.eq_ignore_ascii_case("volume")
                || var_trimmed.eq_ignore_ascii_case("value");
            let is_price = var_trimmed.eq_ignore_ascii_case("price");

            if is_foreign_flow {
                foreign_flow_conditions.push(cond.clone());
                continue;
            }

            if is_volume_or_value || is_price {
                manual_conditions.push(cond.clone());
            }

            if is_volume_or_value {
                continue;
            }

            let variable = if is_price {
                "last_close_price".to_string()
            } else if var_trimmed.eq_ignore_ascii_case("market_cap")
                || var_trimmed.to_ascii_lowercase().starts_with("market_cap[")
            {
                "market_cap".to_string()
            } else {
                format!("{}[{}]", var_trimmed, year)
            };

            let cond_str = format!("{}{}{}", variable, cond.operator, cond.value);
            let conn = cond.connector_to_next.as_deref().map(|c| c.to_lowercase());
            condition_parts.push((cond_str, conn));
        }

        if !manual_conditions.is_empty() {
            manual_filter_groups.push(StrategyRuleGroup {
                id: group.id.clone(),
                connector_to_next: group.connector_to_next.clone(),
                conditions: manual_conditions,
            });
        }

        if !foreign_flow_conditions.is_empty() {
            foreign_flow_filter_groups.push(StrategyRuleGroup {
                id: group.id.clone(),
                connector_to_next: group.connector_to_next.clone(),
                conditions: foreign_flow_conditions,
            });
        }

        if condition_parts.is_empty() {
            continue;
        }

        let mut group_clause = String::new();
        for (i, (cond_str, connector)) in condition_parts.iter().enumerate() {
            group_clause.push_str(cond_str);
            if i + 1 < condition_parts.len() {
                let conn = connector.as_deref().unwrap_or("and");
                group_clause.push(' ');
                group_clause.push_str(conn);
                group_clause.push(' ');
            }
        }

        let inter_conn = group
            .connector_to_next
            .as_deref()
            .map(|c| c.to_lowercase())
            .unwrap_or_else(|| "and".to_string());
        valid_group_clauses.push((format!("({})", group_clause), inter_conn));
    }

    if valid_group_clauses.is_empty() {
        if !manual_filter_groups.is_empty() || !foreign_flow_filter_groups.is_empty() {
            return (
                "last_close_price>0".to_string(),
                manual_filter_groups,
                foreign_flow_filter_groups,
            );
        }
        return (
            String::new(),
            manual_filter_groups,
            foreign_flow_filter_groups,
        );
    }

    let mut full_query = String::new();
    for (i, (group_str, inter_conn)) in valid_group_clauses.iter().enumerate() {
        full_query.push_str(group_str);
        if i + 1 < valid_group_clauses.len() {
            full_query.push(' ');
            full_query.push_str(inter_conn);
            full_query.push(' ');
        }
    }

    (full_query, manual_filter_groups, foreign_flow_filter_groups)
}

fn evaluate_single_manual_condition(
    cond: &StrategyRuleCondition,
    price: f64,
    volume: f64,
    value: f64,
) -> bool {
    let var = cond.variable.trim();
    let target = if var.eq_ignore_ascii_case("volume") {
        volume
    } else if var.eq_ignore_ascii_case("value") {
        value
    } else if var.eq_ignore_ascii_case("price") {
        price
    } else {
        return true;
    };

    let filter_val: f64 = match cond.value.trim().parse() {
        Ok(v) => v,
        Err(_) => return true,
    };

    match cond.operator.as_str() {
        ">" => target > filter_val,
        "<" => target < filter_val,
        ">=" => target >= filter_val,
        "<=" => target <= filter_val,
        "=" => (target - filter_val).abs() < 1e-6,
        "!=" => (target - filter_val).abs() >= 1e-6,
        _ => true,
    }
}

pub fn evaluate_manual_rules(
    groups: &[StrategyRuleGroup],
    price: f64,
    volume: f64,
    value: f64,
) -> bool {
    if groups.is_empty() {
        return true;
    }

    let mut group_results: Vec<(bool, Option<String>)> = Vec::new();

    for group in groups {
        if group.conditions.is_empty() {
            continue;
        }

        let mut group_res =
            evaluate_single_manual_condition(&group.conditions[0], price, volume, value);
        for i in 1..group.conditions.len() {
            let prev_conn = group.conditions[i - 1]
                .connector_to_next
                .as_deref()
                .unwrap_or("AND");
            let curr_res =
                evaluate_single_manual_condition(&group.conditions[i], price, volume, value);
            if prev_conn.eq_ignore_ascii_case("OR") {
                group_res = group_res || curr_res;
            } else {
                group_res = group_res && curr_res;
            }
        }

        group_results.push((group_res, group.connector_to_next.clone()));
    }

    if group_results.is_empty() {
        return true;
    }

    let mut overall_res = group_results[0].0;
    for i in 1..group_results.len() {
        let prev_conn = group_results[i - 1].1.as_deref().unwrap_or("AND");
        let curr_res = group_results[i].0;
        if prev_conn.eq_ignore_ascii_case("OR") {
            overall_res = overall_res || curr_res;
        } else {
            overall_res = overall_res && curr_res;
        }
    }

    overall_res
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct StockForeignFlowValues {
    pub last_1_week: f64,
    pub last_1_month: f64,
    pub last_3_months: f64,
}

fn evaluate_single_foreign_flow_condition(
    cond: &StrategyRuleCondition,
    values: &StockForeignFlowValues,
) -> bool {
    let var = cond.variable.trim();
    let target = if var.eq_ignore_ascii_case("last_1_week_foreign_flow") {
        values.last_1_week
    } else if var.eq_ignore_ascii_case("last_1_month_foreign_flow") {
        values.last_1_month
    } else if var.eq_ignore_ascii_case("last_3_months_foreign_flow") {
        values.last_3_months
    } else {
        return true;
    };

    let filter_val: f64 = match cond.value.trim().parse() {
        Ok(v) => v,
        Err(_) => return true,
    };

    match cond.operator.as_str() {
        ">" => target > filter_val,
        "<" => target < filter_val,
        ">=" => target >= filter_val,
        "<=" => target <= filter_val,
        "=" => (target - filter_val).abs() < 1e-6,
        "!=" => (target - filter_val).abs() >= 1e-6,
        _ => true,
    }
}

pub fn evaluate_foreign_flow_rules(
    groups: &[StrategyRuleGroup],
    values: &StockForeignFlowValues,
) -> bool {
    if groups.is_empty() {
        return true;
    }

    let mut group_results: Vec<(bool, Option<String>)> = Vec::new();

    for group in groups {
        if group.conditions.is_empty() {
            continue;
        }

        let mut group_res = evaluate_single_foreign_flow_condition(&group.conditions[0], values);
        for i in 1..group.conditions.len() {
            let prev_conn = group.conditions[i - 1]
                .connector_to_next
                .as_deref()
                .unwrap_or("AND");
            let curr_res = evaluate_single_foreign_flow_condition(&group.conditions[i], values);
            if prev_conn.eq_ignore_ascii_case("OR") {
                group_res = group_res || curr_res;
            } else {
                group_res = group_res && curr_res;
            }
        }

        group_results.push((group_res, group.connector_to_next.clone()));
    }

    if group_results.is_empty() {
        return true;
    }

    let mut overall_res = group_results[0].0;
    for i in 1..group_results.len() {
        let prev_conn = group_results[i - 1].1.as_deref().unwrap_or("AND");
        let curr_res = group_results[i].0;
        if prev_conn.eq_ignore_ascii_case("OR") {
            overall_res = overall_res || curr_res;
        } else {
            overall_res = overall_res && curr_res;
        }
    }

    overall_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_clean_screener_query_with_no_trailing_connectors() {
        let groups = vec![
            StrategyRuleGroup {
                id: "g1".to_string(),
                connector_to_next: Some("AND".to_string()),
                conditions: vec![StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "pb".to_string(),
                    operator: "<".to_string(),
                    value: "1.0".to_string(),
                    connector_to_next: Some("AND".to_string()),
                }],
            },
            StrategyRuleGroup {
                id: "g2".to_string(),
                connector_to_next: None,
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c2".to_string(),
                        variable: "price".to_string(),
                        operator: ">".to_string(),
                        value: "50".to_string(),
                        connector_to_next: Some("AND".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c3".to_string(),
                        variable: "price".to_string(),
                        operator: "<=".to_string(),
                        value: "1000".to_string(),
                        connector_to_next: Some("AND".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c4".to_string(),
                        variable: "volume".to_string(),
                        operator: ">=".to_string(),
                        value: "100000".to_string(),
                        connector_to_next: None,
                    },
                ],
            },
        ];

        let (where_query, manual_filters, _) = build_where_query(&groups, 2025);
        assert_eq!(
            where_query,
            "(pb[2025]<1.0) and (last_close_price>50 and last_close_price<=1000)"
        );
        assert_eq!(manual_filters.len(), 1);
        assert_eq!(manual_filters[0].conditions.len(), 3);
        assert_eq!(manual_filters[0].conditions[0].variable, "price");
        assert_eq!(manual_filters[0].conditions[1].variable, "price");
        assert_eq!(manual_filters[0].conditions[2].variable, "volume");
    }

    #[test]
    fn should_fallback_to_last_close_price_when_only_manual_filters() {
        let groups = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "value".to_string(),
                    operator: ">=".to_string(),
                    value: "1000000000".to_string(),
                    connector_to_next: Some("OR".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "volume".to_string(),
                    operator: ">=".to_string(),
                    value: "500000".to_string(),
                    connector_to_next: None,
                },
            ],
        }];

        let (where_query, manual_filters, _) = build_where_query(&groups, 2025);
        assert_eq!(where_query, "last_close_price>0");
        assert_eq!(manual_filters.len(), 1);
        assert_eq!(manual_filters[0].conditions.len(), 2);
        assert_eq!(manual_filters[0].conditions[0].variable, "value");
        assert_eq!(manual_filters[0].conditions[1].variable, "volume");
    }

    #[test]
    fn should_evaluate_manual_rules_with_value_formula_and_connectors() {
        let groups = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "value".to_string(),
                    operator: ">=".to_string(),
                    value: "5000000".to_string(),
                    connector_to_next: Some("AND".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "volume".to_string(),
                    operator: ">=".to_string(),
                    value: "100".to_string(),
                    connector_to_next: None,
                },
            ],
        }];

        assert!(evaluate_manual_rules(&groups, 1000.0, 100.0, 10000000.0));
        assert!(!evaluate_manual_rules(&groups, 1000.0, 50.0, 10000000.0));
        assert!(!evaluate_manual_rules(&groups, 1000.0, 100.0, 1000000.0));
    }

    #[test]
    fn should_format_market_cap_without_year_in_screener_query() {
        let groups = vec![
            StrategyRuleGroup {
                id: "g1".to_string(),
                connector_to_next: Some("AND".to_string()),
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c1".to_string(),
                        variable: "market_cap".to_string(),
                        operator: ">=".to_string(),
                        value: "1000000000".to_string(),
                        connector_to_next: Some("AND".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c2".to_string(),
                        variable: "pe".to_string(),
                        operator: "<".to_string(),
                        value: "15".to_string(),
                        connector_to_next: None,
                    },
                ],
            },
            StrategyRuleGroup {
                id: "g2".to_string(),
                connector_to_next: None,
                conditions: vec![StrategyRuleCondition {
                    id: "c3".to_string(),
                    variable: "market_cap[2024]".to_string(),
                    operator: ">".to_string(),
                    value: "500000000".to_string(),
                    connector_to_next: None,
                }],
            },
        ];

        let (where_query, _, _) = build_where_query(&groups, 2025);
        assert_eq!(
            where_query,
            "(market_cap>=1000000000 and pe[2025]<15) and (market_cap>500000000)"
        );
    }

    #[test]
    fn should_include_price_in_both_screener_query_and_manual_filters() {
        let groups = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "price".to_string(),
                    operator: ">=".to_string(),
                    value: "200".to_string(),
                    connector_to_next: Some("AND".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "price".to_string(),
                    operator: "<=".to_string(),
                    value: "5000".to_string(),
                    connector_to_next: None,
                },
            ],
        }];

        let (where_query, manual_filters, _) = build_where_query(&groups, 2024);
        assert_eq!(
            where_query,
            "(last_close_price>=200 and last_close_price<=5000)"
        );
        assert_eq!(manual_filters.len(), 1);
        assert_eq!(manual_filters[0].conditions.len(), 2);
        assert_eq!(manual_filters[0].conditions[0].variable, "price");
        assert_eq!(manual_filters[0].conditions[1].variable, "price");
    }

    #[test]
    fn should_separate_foreign_flow_rules_from_screener_query() {
        let groups = vec![
            StrategyRuleGroup {
                id: "g1".to_string(),
                connector_to_next: Some("AND".to_string()),
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c1".to_string(),
                        variable: "pb".to_string(),
                        operator: "<".to_string(),
                        value: "1.5".to_string(),
                        connector_to_next: Some("AND".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c2".to_string(),
                        variable: "last_1_week_foreign_flow".to_string(),
                        operator: ">".to_string(),
                        value: "0".to_string(),
                        connector_to_next: None,
                    },
                ],
            },
            StrategyRuleGroup {
                id: "g2".to_string(),
                connector_to_next: None,
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c3".to_string(),
                        variable: "last_1_month_foreign_flow".to_string(),
                        operator: ">=".to_string(),
                        value: "5000000000".to_string(),
                        connector_to_next: Some("OR".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c4".to_string(),
                        variable: "last_3_months_foreign_flow".to_string(),
                        operator: ">=".to_string(),
                        value: "10000000000".to_string(),
                        connector_to_next: None,
                    },
                ],
            },
        ];

        let (where_query, manual_filters, ff_filters) = build_where_query(&groups, 2024);
        assert_eq!(where_query, "(pb[2024]<1.5)");
        assert_eq!(manual_filters.len(), 0);
        assert_eq!(ff_filters.len(), 2);
        assert_eq!(ff_filters[0].conditions.len(), 1);
        assert_eq!(
            ff_filters[0].conditions[0].variable,
            "last_1_week_foreign_flow"
        );
        assert_eq!(ff_filters[1].conditions.len(), 2);
        assert_eq!(
            ff_filters[1].conditions[0].variable,
            "last_1_month_foreign_flow"
        );
        assert_eq!(
            ff_filters[1].conditions[1].variable,
            "last_3_months_foreign_flow"
        );
    }

    #[test]
    fn should_fallback_to_last_close_price_when_only_foreign_flow_rules() {
        let groups = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![StrategyRuleCondition {
                id: "c1".to_string(),
                variable: "last_1_week_foreign_flow".to_string(),
                operator: ">".to_string(),
                value: "1000000".to_string(),
                connector_to_next: None,
            }],
        }];

        let (where_query, manual_filters, ff_filters) = build_where_query(&groups, 2024);
        assert_eq!(where_query, "last_close_price>0");
        assert_eq!(manual_filters.len(), 0);
        assert_eq!(ff_filters.len(), 1);
        assert_eq!(
            ff_filters[0].conditions[0].variable,
            "last_1_week_foreign_flow"
        );
    }

    #[test]
    fn should_evaluate_foreign_flow_rules_correctly() {
        let groups = vec![
            StrategyRuleGroup {
                id: "g1".to_string(),
                connector_to_next: Some("AND".to_string()),
                conditions: vec![StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "last_1_week_foreign_flow".to_string(),
                    operator: ">".to_string(),
                    value: "0".to_string(),
                    connector_to_next: None,
                }],
            },
            StrategyRuleGroup {
                id: "g2".to_string(),
                connector_to_next: None,
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c2".to_string(),
                        variable: "last_1_month_foreign_flow".to_string(),
                        operator: ">=".to_string(),
                        value: "1000000000".to_string(),
                        connector_to_next: Some("OR".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c3".to_string(),
                        variable: "last_3_months_foreign_flow".to_string(),
                        operator: ">=".to_string(),
                        value: "5000000000".to_string(),
                        connector_to_next: None,
                    },
                ],
            },
        ];

        // Case 1: Both pass
        let values1 = StockForeignFlowValues {
            last_1_week: 500_000_000.0,
            last_1_month: 2_000_000_000.0,
            last_3_months: 1_000_000_000.0,
        };
        assert!(evaluate_foreign_flow_rules(&groups, &values1));

        // Case 2: 1w fails (<= 0)
        let values2 = StockForeignFlowValues {
            last_1_week: -100_000.0,
            last_1_month: 2_000_000_000.0,
            last_3_months: 10_000_000_000.0,
        };
        assert!(!evaluate_foreign_flow_rules(&groups, &values2));

        // Case 3: 1w passes, 1m fails but 3m passes (due to OR)
        let values3 = StockForeignFlowValues {
            last_1_week: 100.0,
            last_1_month: 500_000.0,
            last_3_months: 6_000_000_000.0,
        };
        assert!(evaluate_foreign_flow_rules(&groups, &values3));

        // Case 4: 1w passes, both 1m and 3m fail
        let values4 = StockForeignFlowValues {
            last_1_week: 100.0,
            last_1_month: 500_000.0,
            last_3_months: 1_000_000.0,
        };
        assert!(!evaluate_foreign_flow_rules(&groups, &values4));
    }
}
