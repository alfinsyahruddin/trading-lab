use crate::entities::{
    backtest::BacktestResultRecord, strategy_suggestion::StrategyAiSuggestionsRequest,
    trading_strategy::StrategyRuleGroup,
};

/// Formats strategy rule groups into a readable text format for AI prompt ingestion.
pub fn format_strategy_rules(rules: Option<&[StrategyRuleGroup]>) -> String {
    match rules {
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
    }
}

/// Generates the prompt string for backtest AI insights.
pub fn get_backtest_ai_insights_prompt(
    job_name: &str,
    strategy_name: &str,
    year: i32,
    duration_months: i32,
    initial_cash: f64,
    result: &BacktestResultRecord,
) -> String {
    format!(
        r#"You are an expert quantitative trading analyst specializing in the Indonesia Stock Exchange (IDX).
Analyze the following backtest simulation results for strategy "{strategy_name}" (Backtest: "{job_name}", Year: {year}, Duration: {duration_months} months, Initial Cash: Rp{initial_cash:.0}):

Key Backtest Metrics:
- Net PnL: Rp{net_pnl:.0} ({net_pnl_pct:.2}%)
- Gross PnL: Rp{gross_pnl:.0} ({gross_pnl_pct:.2}%)
- Total Fees: Rp{total_fees:.0}
- Total Trades: {trades_processed} (Wins: {wins}, Losses: {losses}, Win Rate: {win_rate:.2}%)
- Profit Factor: {profit_factor:.2}
- Sharpe Ratio: {sharpe_ratio:.2}
- Portfolio Volatility: {volatility:.2}%
- Max Profit: Rp{max_profit:.0} ({max_profit_pct:.2}%), Max Loss: Rp{max_loss:.0} ({max_loss_pct:.2}%)
- Avg Profit: Rp{avg_profit:.0} ({avg_profit_pct:.2}%), Avg Loss: Rp{avg_loss:.0} ({avg_loss_pct:.2}%)
- Avg Holding Time: {avg_hold:.1} days (Avg Win Hold: {win_hold:.1} days, Avg Loss Hold: {loss_hold:.1} days)

Generate high-impact AI insights consisting of a JSON array of up to 4 clear, insightful keypoint strings (maximum 4 strings).
Each keypoint should be 1-2 concise sentences addressing one of the following 4 dimensions:
1. Overall Profitability & Return: Net return vs duration, capital growth vs initial cash, win rate vs profit factor, and fee drag impact.
2. Risk & Volatility Profile: Sharpe ratio evaluation, risk-adjusted performance, and downside/drawdown risk.
3. Holding & Execution Efficiency: Win holding duration vs loss holding duration (discipline in cutting losses vs letting winners run).
4. Strategic Verdict & Actionable Improvement: Concrete recommendation for parameter tuning (e.g. SL, TP, or holding window) under Indonesian market conditions.

Formatting and Style Guidelines:
- Highlight key metrics, numbers, and stats in Markdown bold (e.g. **+31.2%**, **-20.0%**, **0.0%**, **Rp 12.5M**, **64%**, **2.3 PF**, **Sharpe 1.8**, **7.5 days**).
- For all percentage returns or changes, ALWAYS include the directional sign prefix: use '+' for positive returns (e.g. **+31.2%**), '-' for negative returns (e.g. **-20.0%**), and '0%' or '0.0%' for neutral/flat returns (e.g. **0.0%**).
- Use Markdown italics for analytical emphasis where appropriate (e.g. *disciplined risk management*, *minimal fee drag*).
- Keep each point crisp, direct, and actionable for traders.

Return ONLY a valid JSON array of strings:
["Keypoint 1...", "Keypoint 2...", "Keypoint 3...", "Keypoint 4..."]"#,
        strategy_name = strategy_name,
        job_name = job_name,
        year = year,
        duration_months = duration_months,
        initial_cash = initial_cash,
        net_pnl = result.net_pnl,
        net_pnl_pct = result.net_pnl_percentage,
        gross_pnl = result.gross_pnl,
        gross_pnl_pct = result.gross_pnl_percentage,
        total_fees = result.total_fees,
        trades_processed = result.trades_processed,
        wins = result.wins,
        losses = result.losses,
        win_rate = result.win_rate,
        profit_factor = result.profit_factor,
        sharpe_ratio = result.sharpe_ratio,
        volatility = result.portfolio_volatility,
        max_profit = result.max_profit,
        max_profit_pct = result.max_profit_percentage,
        max_loss = result.max_loss,
        max_loss_pct = result.max_loss_percentage,
        avg_profit = result.avg_profit,
        avg_profit_pct = result.avg_profit_percentage,
        avg_loss = result.avg_loss,
        avg_loss_pct = result.avg_loss_percentage,
        avg_hold = result.avg_hold_time_days,
        win_hold = result.avg_win_hold_days,
        loss_hold = result.avg_loss_hold_days,
    )
}

/// Generates the prompt string for trading strategy AI refinement suggestions.
pub fn get_strategy_ai_suggestions_prompt(
    name: &str,
    description: Option<&str>,
    tp_percentage: f64,
    sl_percentage: f64,
    max_holding_period_days: i32,
    rules: Option<&[StrategyRuleGroup]>,
) -> String {
    let risk_reward_ratio = if sl_percentage > 0.0 {
        tp_percentage / sl_percentage
    } else {
        0.0
    };

    let desc = description.unwrap_or("No description provided");
    let rules_str = format_strategy_rules(rules);

    format!(
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
- Price & Market: "price", "volume", "value", "market_cap", "shares_outstanding", "last_1_week_foreign_flow", "last_1_month_foreign_flow", "last_3_months_foreign_flow"
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
        name = name,
        desc = desc,
        tp = tp_percentage,
        sl = sl_percentage,
        rr = risk_reward_ratio,
        holding = max_holding_period_days,
        rules_str = rules_str,
    )
}

/// Convenience helper to generate strategy suggestions prompt directly from a `StrategyAiSuggestionsRequest`.
pub fn get_strategy_ai_suggestions_prompt_from_request(
    request: &StrategyAiSuggestionsRequest,
) -> String {
    get_strategy_ai_suggestions_prompt(
        &request.name,
        request.description.as_deref(),
        request.tp_percentage,
        request.sl_percentage,
        request.max_holding_period_days,
        request.rules.as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::trading_strategy::StrategyRuleCondition;
    use uuid::Uuid;

    #[test]
    fn should_format_empty_or_none_rules() {
        assert_eq!(format_strategy_rules(None), "No conditional rules defined.");
        assert_eq!(
            format_strategy_rules(Some(&[])),
            "No conditional rules defined."
        );
    }

    #[test]
    fn should_format_strategy_rules_correctly() {
        let rules = vec![
            StrategyRuleGroup {
                id: "group-0".to_string(),
                conditions: vec![
                    StrategyRuleCondition {
                        id: "c-0".to_string(),
                        variable: "price".to_string(),
                        operator: ">".to_string(),
                        value: "200".to_string(),
                        connector_to_next: Some("AND".to_string()),
                    },
                    StrategyRuleCondition {
                        id: "c-1".to_string(),
                        variable: "pe".to_string(),
                        operator: "<=".to_string(),
                        value: "15".to_string(),
                        connector_to_next: None,
                    },
                ],
                connector_to_next: Some("OR".to_string()),
            },
            StrategyRuleGroup {
                id: "group-1".to_string(),
                conditions: vec![StrategyRuleCondition {
                    id: "c-2".to_string(),
                    variable: "market_cap".to_string(),
                    operator: ">=".to_string(),
                    value: "1000000000000".to_string(),
                    connector_to_next: None,
                }],
                connector_to_next: None,
            },
        ];

        let formatted = format_strategy_rules(Some(&rules));
        assert!(formatted.contains("Group 0 (next: OR):"));
        assert!(formatted.contains("[Condition 0]: price > 200 (next: AND)"));
        assert!(formatted.contains("[Condition 1]: pe <= 15 (next: AND)"));
        assert!(formatted.contains("Group 1 (next: AND):"));
        assert!(formatted.contains("[Condition 0]: market_cap >= 1000000000000 (next: AND)"));
    }

    #[test]
    fn should_generate_backtest_ai_insights_prompt() {
        let result = BacktestResultRecord {
            id: Uuid::new_v4(),
            backtest_job_id: Uuid::new_v4(),
            available_cash: 110_000_000.0,
            trades_processed: 25,
            net_pnl: 10_000_000.0,
            net_pnl_percentage: 10.0,
            gross_pnl: 10_500_000.0,
            gross_pnl_percentage: 10.5,
            win_rate: 64.0,
            profit_factor: 2.3,
            wins: 16,
            losses: 9,
            sharpe_ratio: 1.8,
            max_profit: 3_000_000.0,
            max_profit_percentage: 8.0,
            max_loss: -1_000_000.0,
            max_loss_percentage: -3.0,
            avg_profit: 1_200_000.0,
            avg_profit_percentage: 4.0,
            avg_loss: -600_000.0,
            avg_loss_percentage: -2.0,
            avg_hold_time_days: 7.5,
            total_fees: 500_000.0,
            avg_win_hold_days: 8.0,
            avg_loss_hold_days: 4.0,
            portfolio_volatility: 14.2,
            ai_insights: None,
        };

        let prompt = get_backtest_ai_insights_prompt(
            "Backtest Job 1",
            "Strategy Alpha",
            2024,
            12,
            100_000_000.0,
            &result,
        );

        assert!(prompt.contains("strategy \"Strategy Alpha\""));
        assert!(prompt.contains("Backtest: \"Backtest Job 1\""));
        assert!(prompt.contains("Year: 2024, Duration: 12 months, Initial Cash: Rp100000000"));
        assert!(prompt.contains("Net PnL: Rp10000000 (10.00%)"));
        assert!(prompt.contains("Win Rate: 64.00%"));
        assert!(prompt.contains("Sharpe Ratio: 1.80"));
        assert!(prompt.contains("Portfolio Volatility: 14.20%"));
        assert!(prompt.contains("maximum 4 strings"));
        assert!(prompt.contains("win rate vs profit factor"));
        assert!(!prompt.contains("2. Win/Loss Performance"));
    }

    #[test]
    fn should_generate_strategy_ai_suggestions_prompt() {
        let prompt = get_strategy_ai_suggestions_prompt(
            "Breakout Trend",
            Some("Momentum strategy"),
            15.0,
            5.0,
            30,
            None,
        );

        assert!(prompt.contains("Strategy Name: \"Breakout Trend\""));
        assert!(prompt.contains("Description: \"Momentum strategy\""));
        assert!(prompt.contains("Take Profit (TP): 15.00%"));
        assert!(prompt.contains("Stop Loss (SL): 5.00%"));
        assert!(prompt.contains("Risk-to-Reward Ratio: 3.00:1"));
        assert!(prompt.contains("Max Holding Period: 30 days"));
        assert!(prompt.contains("No conditional rules defined."));
    }

    #[test]
    fn should_generate_strategy_prompt_with_zero_sl_and_no_description() {
        let prompt =
            get_strategy_ai_suggestions_prompt("Zero SL Strategy", None, 10.0, 0.0, 10, None);

        assert!(prompt.contains("Description: \"No description provided\""));
        assert!(prompt.contains("Risk-to-Reward Ratio: 0.00:1"));
    }

    #[test]
    fn should_generate_strategy_prompt_from_request() {
        let req = StrategyAiSuggestionsRequest {
            name: "Value Inversion".to_string(),
            description: Some("Deep value".to_string()),
            tp_percentage: 20.0,
            sl_percentage: 10.0,
            max_holding_period_days: 60,
            rules: Some(vec![StrategyRuleGroup {
                id: "group-0".to_string(),
                conditions: vec![StrategyRuleCondition {
                    id: "c-0".to_string(),
                    variable: "pb".to_string(),
                    operator: "<".to_string(),
                    value: "1.0".to_string(),
                    connector_to_next: None,
                }],
                connector_to_next: None,
            }]),
        };

        let prompt = get_strategy_ai_suggestions_prompt_from_request(&req);
        assert!(prompt.contains("Strategy Name: \"Value Inversion\""));
        assert!(prompt.contains("[Condition 0]: pb < 1.0 (next: AND)"));
    }
}
