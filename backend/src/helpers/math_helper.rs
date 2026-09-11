/// Calculates the win rate percentage from total trades and winning trades.
/// Returns 0.0 if total_trades is 0.
pub fn calculate_win_rate(wins: usize, total_trades: usize) -> f64 {
    if total_trades == 0 {
        0.0
    } else {
        (wins as f64) / (total_trades as f64) * 100.0
    }
}

/// Calculates the profit factor (gross profits / gross losses).
/// When sum_losing is 0.0 (no losses), returns sum_winning.
pub fn calculate_profit_factor(sum_winning: f64, sum_losing: f64) -> f64 {
    if sum_losing < 0.0 {
        sum_winning / sum_losing.abs()
    } else {
        sum_winning
    }
}

/// Calculates the annualized Sharpe ratio and portfolio volatility from daily returns.
/// Returns (0.0, 0.0) if returns are empty or daily return variance is zero.
pub fn calculate_sharpe_ratio(daily_returns: &[f64], risk_free_annual_rate: f64) -> (f64, f64) {
    if daily_returns.is_empty() {
        return (0.0, 0.0);
    }

    let n = daily_returns.len() as f64;
    let avg_daily_return = daily_returns.iter().sum::<f64>() / n;
    let variance = daily_returns
        .iter()
        .map(|&r| (r - avg_daily_return).powi(2))
        .sum::<f64>()
        / n;
    let std_daily_return = variance.sqrt();

    let risk_free_daily = risk_free_annual_rate / 252.0;
    let mut sharpe_ratio = 0.0;
    if std_daily_return > 0.0 {
        sharpe_ratio = (avg_daily_return - risk_free_daily) / std_daily_return * (252.0_f64).sqrt();
    }
    let portfolio_volatility = std_daily_return * (252.0_f64).sqrt() * 100.0;

    (sharpe_ratio, portfolio_volatility)
}

/// Calculates maximum drawdown percentage from an equity curve series.
/// Returns 0.0 if fewer than 2 data points or if values never decline below their peak.
pub fn calculate_max_drawdown(equity_curve: &[f64]) -> f64 {
    if equity_curve.len() < 2 {
        return 0.0;
    }

    let mut peak = equity_curve[0];
    let mut max_drawdown = 0.0;

    for &val in equity_curve {
        if val > peak {
            peak = val;
        } else if peak > 0.0 {
            let dd = (peak - val) / peak * 100.0;
            if dd > max_drawdown {
                max_drawdown = dd;
            }
        }
    }

    max_drawdown
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeFeeCalculation {
    pub buy_value: f64,
    pub buy_fee: f64,
    pub total_cost: f64,
    pub sell_value: f64,
    pub sell_fee: f64,
    pub net_sell: f64,
    pub total_fees: f64,
    pub pnl: f64,
    pub pnl_percentage: f64,
}

/// Calculates trade PnL taking into account IDX broker buy and sell fees.
pub fn calculate_trade_pnl(
    lots: i32,
    buy_price: f64,
    sell_price: f64,
    buy_fee_pct: f64,
    sell_fee_pct: f64,
) -> TradeFeeCalculation {
    let buy_value = (lots as f64) * 100.0 * buy_price;
    let buy_fee = buy_value * buy_fee_pct / 100.0;
    let total_cost = buy_value + buy_fee;

    let sell_value = (lots as f64) * 100.0 * sell_price;
    let sell_fee = sell_value * sell_fee_pct / 100.0;
    let net_sell = sell_value - sell_fee;

    let total_fees = buy_fee + sell_fee;
    let pnl = net_sell - total_cost;
    let pnl_percentage = if total_cost > 0.0 {
        pnl / total_cost * 100.0
    } else {
        0.0
    };

    TradeFeeCalculation {
        buy_value,
        buy_fee,
        total_cost,
        sell_value,
        sell_fee,
        net_sell,
        total_fees,
        pnl,
        pnl_percentage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win_rate_with_zero_trades_should_be_zero() {
        assert_eq!(calculate_win_rate(0, 0), 0.0);
    }

    #[test]
    fn win_rate_with_100_percent_wins() {
        assert_eq!(calculate_win_rate(10, 10), 100.0);
        assert_eq!(calculate_win_rate(1, 1), 100.0);
    }

    #[test]
    fn win_rate_with_partial_wins() {
        assert_eq!(calculate_win_rate(3, 4), 75.0);
        assert_eq!(calculate_win_rate(0, 5), 0.0);
    }

    #[test]
    fn profit_factor_with_zero_losses_returns_sum_winning() {
        // Zero losses edge case
        assert_eq!(calculate_profit_factor(5_000_000.0, 0.0), 5_000_000.0);
        assert_eq!(calculate_profit_factor(0.0, 0.0), 0.0);
    }

    #[test]
    fn profit_factor_with_losses() {
        let pf = calculate_profit_factor(6_000_000.0, -2_000_000.0);
        assert!((pf - 3.0).abs() < 1e-6);
    }

    #[test]
    fn sharpe_ratio_with_zero_variance_returns_zero() {
        // Identical returns -> std dev = 0 -> Sharpe ratio = 0.0
        let flat_returns = vec![0.005, 0.005, 0.005, 0.005];
        let (sharpe, vol) = calculate_sharpe_ratio(&flat_returns, 0.02);
        assert_eq!(sharpe, 0.0);
        assert_eq!(vol, 0.0);

        // Zero returns
        let zero_returns = vec![0.0, 0.0, 0.0];
        let (sharpe, vol) = calculate_sharpe_ratio(&zero_returns, 0.02);
        assert_eq!(sharpe, 0.0);
        assert_eq!(vol, 0.0);

        // Empty returns
        let (sharpe, vol) = calculate_sharpe_ratio(&[], 0.02);
        assert_eq!(sharpe, 0.0);
        assert_eq!(vol, 0.0);
    }

    #[test]
    fn sharpe_ratio_with_variance_calculates_correctly() {
        let returns = vec![0.01, -0.005, 0.015, -0.002, 0.02];
        let (sharpe, vol) = calculate_sharpe_ratio(&returns, 0.02);
        assert!(sharpe > 0.0);
        assert!(vol > 0.0);
    }

    #[test]
    fn max_drawdown_calculation_edge_cases() {
        // Less than 2 elements
        assert_eq!(calculate_max_drawdown(&[]), 0.0);
        assert_eq!(calculate_max_drawdown(&[100.0]), 0.0);

        // Monotonically increasing: 0% drawdown
        let all_gains = vec![100.0, 110.0, 120.0, 130.0];
        assert_eq!(calculate_max_drawdown(&all_gains), 0.0);

        // Simple drawdown: 100 -> 80 = 20% drawdown
        let dip = vec![100.0, 80.0, 95.0];
        assert!((calculate_max_drawdown(&dip) - 20.0).abs() < 1e-6);

        // Multiple troughs: peak 100 -> 90 (10%), recovery to 150 -> 75 (50%)
        let multi_peak = vec![100.0, 90.0, 150.0, 75.0, 120.0];
        assert!((calculate_max_drawdown(&multi_peak) - 50.0).abs() < 1e-6);

        // 100% loss: 100 -> 0
        let total_loss = vec![100.0, 50.0, 0.0];
        assert!((calculate_max_drawdown(&total_loss) - 100.0).abs() < 1e-6);
    }

    #[test]
    fn trade_fees_deduction_and_breakeven_pnl() {
        // 10 lots of stock at buy 1000, sell at 1000 (flat gross)
        // buy_fee = 0.15%, sell_fee = 0.25%
        let res = calculate_trade_pnl(10, 1000.0, 1000.0, 0.15, 0.25);
        assert_eq!(res.buy_value, 1_000_000.0);
        assert_eq!(res.buy_fee, 1_500.0);
        assert_eq!(res.total_cost, 1_001_500.0);

        assert_eq!(res.sell_value, 1_000_000.0);
        assert_eq!(res.sell_fee, 2_500.0);
        assert_eq!(res.net_sell, 997_500.0);

        assert_eq!(res.total_fees, 4_000.0);
        // Net PnL is negative strictly due to fees
        assert_eq!(res.pnl, -4_000.0);
        assert!((res.pnl_percentage - (-4000.0 / 1_001_500.0 * 100.0)).abs() < 1e-6);
    }

    #[test]
    fn trade_fees_with_zero_fees() {
        let res = calculate_trade_pnl(5, 2000.0, 2200.0, 0.0, 0.0);
        assert_eq!(res.buy_value, 1_000_000.0);
        assert_eq!(res.buy_fee, 0.0);
        assert_eq!(res.sell_value, 1_100_000.0);
        assert_eq!(res.sell_fee, 0.0);
        assert_eq!(res.total_fees, 0.0);
        assert_eq!(res.pnl, 100_000.0);
        assert_eq!(res.pnl_percentage, 10.0);
    }
}
