use std::collections::{HashMap, VecDeque};

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    entities::backtest::{
        BacktestPortfolioHistoryRecord, BacktestResultRecord, BacktestTradeRecord, Position,
        StockData,
    },
    helpers::math_helper,
};

#[allow(clippy::too_many_arguments)]
pub fn simulate_backtest(
    job_id: Uuid,
    stocks: &[StockData],
    initial_cash: f64,
    max_holding_stocks: i32,
    tp_percentage: f64,
    sl_percentage: f64,
    max_holding_period_days: i32,
    buy_fee_percentage: f64,
    sell_fee_percentage: f64,
) -> (
    BacktestResultRecord,
    Vec<BacktestPortfolioHistoryRecord>,
    Vec<BacktestTradeRecord>,
) {
    let entry_capital = initial_cash / (max_holding_stocks as f64);
    let mut available_cash = initial_cash;
    let mut cumulative_fees = 0.0;
    let mut open_positions: Vec<Position> = Vec::new();
    let mut stock_queue: VecDeque<usize> = (0..stocks.len()).collect();
    let mut completed_trades: Vec<BacktestTradeRecord> = Vec::new();
    let mut portfolio_history: Vec<BacktestPortfolioHistoryRecord> = Vec::new();

    let mut all_dates: Vec<NaiveDate> = Vec::new();
    let mut date_to_stock_data: HashMap<NaiveDate, HashMap<String, f64>> = HashMap::new();

    for stock in stocks {
        for (date, close, _) in &stock.daily_data {
            all_dates.push(*date);
            date_to_stock_data
                .entry(*date)
                .or_default()
                .insert(stock.code.clone(), *close);
        }
    }

    all_dates.sort();
    all_dates.dedup();

    for current_date in all_dates {
        let Some(current_stock_data) = date_to_stock_data.get(&current_date) else {
            continue;
        };

        // a. Check exits
        let mut i = (open_positions.len() as i32) - 1;
        while i >= 0 {
            let idx = i as usize;
            let pos = &open_positions[idx];
            if let Some(&current_close) = current_stock_data.get(&pos.code) {
                let hold_days = (current_date - pos.buy_date).num_days();
                let tp_price = pos.buy_price * (1.0 + tp_percentage / 100.0);
                let sl_price = pos.buy_price * (1.0 - sl_percentage / 100.0);

                let mut exit_reason = None;
                let mut sell_price = current_close;

                if current_close >= tp_price {
                    exit_reason = Some("TAKE_PROFIT");
                    sell_price = tp_price;
                } else if current_close <= sl_price {
                    exit_reason = Some("STOP_LOSS");
                    sell_price = sl_price;
                } else if hold_days >= (max_holding_period_days as i64) {
                    exit_reason = Some("MAX_HOLDING_TIME");
                    sell_price = current_close;
                }

                if let Some(reason) = exit_reason {
                    let sell_value = (pos.lots as f64) * 100.0 * sell_price;
                    let sell_fee_amount = sell_value * sell_fee_percentage / 100.0;
                    let net_sell = sell_value - sell_fee_amount;
                    let pnl = net_sell - (pos.buy_value + pos.buy_fee);
                    let pnl_percentage = pnl / (pos.buy_value + pos.buy_fee) * 100.0;

                    available_cash += net_sell;
                    cumulative_fees += sell_fee_amount;

                    completed_trades.push(BacktestTradeRecord {
                        id: Uuid::new_v4(),
                        backtest_job_id: job_id,
                        code: pos.code.clone(),
                        pnl,
                        pnl_percentage,
                        exit_reason: reason.to_string(),
                        lot: pos.lots,
                        buy_price: pos.buy_price,
                        buy_value: pos.buy_value,
                        sell_price,
                        sell_value,
                        buy_fee: pos.buy_fee,
                        sell_fee: sell_fee_amount,
                        buy_date: pos.buy_date,
                        sell_date: current_date,
                    });

                    open_positions.remove(idx);
                }
            }
            i -= 1;
        }

        // b. Buy new stocks
        let mut attempts = 0;
        let queue_len = stock_queue.len();
        while open_positions.len() < (max_holding_stocks as usize) && attempts < queue_len {
            if let Some(stock_idx) = stock_queue.pop_front() {
                attempts += 1;
                let stock = &stocks[stock_idx];

                let is_already_owned = open_positions.iter().any(|p| p.code == stock.code);
                if is_already_owned {
                    stock_queue.push_back(stock_idx);
                    continue;
                }

                if let Some(&current_close) = current_stock_data.get(&stock.code) {
                    let lots = (entry_capital / (100.0 * current_close)).floor() as i32;
                    if lots > 0 {
                        let buy_value = (lots as f64) * 100.0 * current_close;
                        let buy_fee_amount = buy_value * buy_fee_percentage / 100.0;
                        let total_cost = buy_value + buy_fee_amount;

                        if total_cost <= available_cash {
                            available_cash -= total_cost;
                            cumulative_fees += buy_fee_amount;
                            open_positions.push(Position {
                                code: stock.code.clone(),
                                lots,
                                buy_price: current_close,
                                buy_value,
                                buy_fee: buy_fee_amount,
                                buy_date: current_date,
                            });
                        } else {
                            stock_queue.push_back(stock_idx);
                        }
                    } else {
                        stock_queue.push_back(stock_idx);
                    }
                } else {
                    stock_queue.push_front(stock_idx);
                    break;
                }
            } else {
                break;
            }
        }

        // c. Record portfolio value
        let mut gross_positions = 0.0;
        let mut net_positions = 0.0;

        for pos in &open_positions {
            let mut last_close = pos.buy_price;
            if let Some(&c) = current_stock_data.get(&pos.code) {
                last_close = c;
            }

            let pos_val = (pos.lots as f64) * 100.0 * last_close;
            gross_positions += pos_val;
            net_positions += pos_val - (pos_val * sell_fee_percentage / 100.0);
        }

        portfolio_history.push(BacktestPortfolioHistoryRecord {
            id: Uuid::new_v4(),
            backtest_job_id: job_id,
            date: current_date,
            net_value: available_cash + net_positions,
            gross_value: available_cash + cumulative_fees + gross_positions,
        });
    }

    // Force close remaining open positions
    if let Some(last_date) = portfolio_history.last().map(|p| p.date) {
        let empty_map = HashMap::new();
        let current_stock_data = date_to_stock_data.get(&last_date).unwrap_or(&empty_map);
        for pos in open_positions {
            let mut current_close = pos.buy_price;
            if let Some(&c) = current_stock_data.get(&pos.code) {
                current_close = c;
            }

            let sell_value = (pos.lots as f64) * 100.0 * current_close;
            let sell_fee_amount = sell_value * sell_fee_percentage / 100.0;
            let net_sell = sell_value - sell_fee_amount;
            let pnl = net_sell - (pos.buy_value + pos.buy_fee);
            let pnl_percentage = pnl / (pos.buy_value + pos.buy_fee) * 100.0;

            available_cash += net_sell;

            completed_trades.push(BacktestTradeRecord {
                id: Uuid::new_v4(),
                backtest_job_id: job_id,
                code: pos.code.clone(),
                pnl,
                pnl_percentage,
                exit_reason: "MAX_HOLDING_TIME".to_string(),
                lot: pos.lots,
                buy_price: pos.buy_price,
                buy_value: pos.buy_value,
                sell_price: current_close,
                sell_value,
                buy_fee: pos.buy_fee,
                sell_fee: sell_fee_amount,
                buy_date: pos.buy_date,
                sell_date: last_date,
            });
        }
    }

    // Calculate stats
    let trades_processed = completed_trades.len() as i32;
    let mut wins: i32 = 0;
    let mut losses: i32 = 0;
    let mut sum_winning = 0.0;
    let mut sum_losing = 0.0;
    let mut sum_win_pct = 0.0;
    let mut sum_loss_pct = 0.0;
    let mut max_profit = 0.0;
    let mut max_loss = 0.0;
    let mut max_profit_percentage = 0.0;
    let mut max_loss_percentage = 0.0;
    let mut sum_hold_days = 0.0;
    let mut sum_win_hold_days = 0.0;
    let mut sum_loss_hold_days = 0.0;
    let mut total_fees = 0.0;
    let mut net_pnl = 0.0;
    let mut gross_pnl = 0.0;

    for t in &completed_trades {
        net_pnl += t.pnl;
        gross_pnl += t.sell_value - t.buy_value;
        total_fees += t.buy_fee + t.sell_fee;
        let hold = (t.sell_date - t.buy_date).num_days() as f64;
        sum_hold_days += hold;

        if t.pnl > 0.0 {
            wins += 1;
            sum_winning += t.pnl;
            sum_win_pct += t.pnl_percentage;
            sum_win_hold_days += hold;
            if t.pnl > max_profit {
                max_profit = t.pnl;
            }
            if t.pnl_percentage > max_profit_percentage {
                max_profit_percentage = t.pnl_percentage;
            }
        } else {
            losses += 1;
            sum_losing += t.pnl;
            sum_loss_pct += t.pnl_percentage;
            sum_loss_hold_days += hold;
            if t.pnl < max_loss {
                max_loss = t.pnl;
            }
            if t.pnl_percentage < max_loss_percentage {
                max_loss_percentage = t.pnl_percentage;
            }
        }
    }

    let win_rate = math_helper::calculate_win_rate(wins as usize, trades_processed as usize);
    let profit_factor = math_helper::calculate_profit_factor(sum_winning, sum_losing);

    let avg_profit = if wins > 0 {
        sum_winning / (wins as f64)
    } else {
        0.0
    };
    let avg_profit_percentage = if wins > 0 {
        sum_win_pct / (wins as f64)
    } else {
        0.0
    };
    let avg_loss = if losses > 0 {
        sum_losing / (losses as f64)
    } else {
        0.0
    };
    let avg_loss_percentage = if losses > 0 {
        sum_loss_pct / (losses as f64)
    } else {
        0.0
    };
    let avg_hold_time_days = if trades_processed > 0 {
        sum_hold_days / (trades_processed as f64)
    } else {
        0.0
    };
    let avg_win_hold_days = if wins > 0 {
        sum_win_hold_days / (wins as f64)
    } else {
        0.0
    };
    let avg_loss_hold_days = if losses > 0 {
        sum_loss_hold_days / (losses as f64)
    } else {
        0.0
    };

    let net_pnl_percentage = net_pnl / initial_cash * 100.0;
    let gross_pnl_percentage = gross_pnl / initial_cash * 100.0;

    let mut daily_returns = Vec::new();
    for i in 1..portfolio_history.len() {
        let prev = portfolio_history[i - 1].net_value;
        let curr = portfolio_history[i].net_value;
        if prev > 0.0 {
            daily_returns.push((curr - prev) / prev);
        }
    }

    let (sharpe_ratio, portfolio_volatility) =
        math_helper::calculate_sharpe_ratio(&daily_returns, 0.02);

    let result_record = BacktestResultRecord {
        id: Uuid::new_v4(),
        backtest_job_id: job_id,
        available_cash,
        trades_processed,
        net_pnl,
        net_pnl_percentage,
        gross_pnl,
        gross_pnl_percentage,
        win_rate,
        profit_factor,
        wins,
        losses,
        sharpe_ratio,
        max_profit,
        max_profit_percentage,
        max_loss,
        max_loss_percentage,
        avg_profit,
        avg_profit_percentage,
        avg_loss,
        avg_loss_percentage,
        avg_hold_time_days,
        total_fees,
        avg_win_hold_days,
        avg_loss_hold_days,
        portfolio_volatility,
        ai_summary: None,
    };

    (result_record, portfolio_history, completed_trades)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_portfolio_history_with_distinct_net_and_gross_values() {
        let job_id = Uuid::new_v4();
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2024, 1, 4).unwrap();

        let stock = StockData {
            code: "BBCA".to_string(),
            daily_data: vec![
                (date1, 1000.0, 10000),
                (date2, 1100.0, 10000), // hits TP (10%)
                (date3, 1100.0, 10000),
            ],
        };

        let initial_cash = 10_000_000.0;
        let (_result, history, trades) = simulate_backtest(
            job_id,
            &[stock],
            initial_cash,
            1,
            10.0, // 10% TP
            5.0,  // 5% SL
            10,
            0.15, // 0.15% buy fee
            0.25, // 0.25% sell fee
        );

        assert_eq!(trades.len(), 1);
        assert_eq!(history.len(), 3);

        let last_entry = history.last().unwrap();
        assert!(last_entry.gross_value > last_entry.net_value);
        assert!(
            (last_entry.gross_value
                - last_entry.net_value
                - (trades[0].buy_fee + trades[0].sell_fee))
                .abs()
                < 1e-6
        );
    }

    #[test]
    fn should_strictly_use_sl_and_tp_percentages_for_exit_price() {
        let job_id = Uuid::new_v4();
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();

        let stock_sl = StockData {
            code: "BBCA".to_string(),
            daily_data: vec![
                (date1, 1000.0, 10000), // Buy at 1000
                (date2, 940.0, 10000),  // Close at 940 (-6%), triggers SL (-5% = 950)
            ],
        };

        let (_, _, trades_sl) = simulate_backtest(
            job_id,
            &[stock_sl],
            10_000_000.0,
            1,
            10.0, // 10% TP
            5.0,  // 5% SL
            10,
            0.0, // 0 fee for exact math test
            0.0,
        );

        assert_eq!(trades_sl.len(), 1);
        assert_eq!(trades_sl[0].exit_reason, "STOP_LOSS");
        assert_eq!(trades_sl[0].sell_price, 950.0);
        assert_eq!(trades_sl[0].pnl_percentage, -5.0);

        let stock_tp = StockData {
            code: "BBRI".to_string(),
            daily_data: vec![
                (date1, 1000.0, 10000), // Buy at 1000
                (date2, 1120.0, 10000), // Close at 1120 (+12%), triggers TP (+10% = 1100)
            ],
        };

        let (_, _, trades_tp) = simulate_backtest(
            job_id,
            &[stock_tp],
            10_000_000.0,
            1,
            10.0, // 10% TP
            5.0,  // 5% SL
            10,
            0.0,
            0.0,
        );

        assert_eq!(trades_tp.len(), 1);
        assert_eq!(trades_tp[0].exit_reason, "TAKE_PROFIT");
        assert_eq!(trades_tp[0].sell_price, 1100.0);
        assert_eq!(trades_tp[0].pnl_percentage, 10.0);
    }

    #[test]
    fn simulate_backtest_with_zero_trades_edge_case() {
        let job_id = Uuid::new_v4();
        let (result, portfolio, trades) =
            simulate_backtest(job_id, &[], 10_000_000.0, 1, 10.0, 5.0, 10, 0.15, 0.25);

        assert_eq!(result.trades_processed, 0);
        assert_eq!(result.wins, 0);
        assert_eq!(result.losses, 0);
        assert_eq!(result.win_rate, 0.0);
        assert_eq!(result.profit_factor, 0.0);
        assert_eq!(result.sharpe_ratio, 0.0);
        assert_eq!(result.portfolio_volatility, 0.0);
        assert_eq!(result.total_fees, 0.0);
        assert_eq!(result.available_cash, 10_000_000.0);
        assert!(trades.is_empty());
        assert!(portfolio.is_empty());
    }

    #[test]
    fn simulate_backtest_with_100_percent_wins_and_zero_losses() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let job_id = Uuid::new_v4();

        let stock = StockData {
            code: "BBCA".to_string(),
            daily_data: vec![
                (date1, 1000.0, 10000),
                (date2, 1150.0, 10000), // +15% > 10% TP
            ],
        };

        let (result, _, trades) = simulate_backtest(
            job_id,
            &[stock],
            10_000_000.0,
            1,
            10.0, // 10% TP
            5.0,  // 5% SL
            10,
            0.0,
            0.0,
        );

        assert_eq!(result.trades_processed, 1);
        assert_eq!(result.wins, 1);
        assert_eq!(result.losses, 0);
        assert_eq!(result.win_rate, 100.0);
        assert!(result.profit_factor > 0.0);
        assert_eq!(result.profit_factor, result.net_pnl);
        assert_eq!(trades[0].exit_reason, "TAKE_PROFIT");
    }

    #[test]
    fn simulate_backtest_with_zero_variance_equity_curve_sharpe() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap();
        let job_id = Uuid::new_v4();

        let stock = StockData {
            code: "FLAT".to_string(),
            daily_data: vec![
                (date1, 1000.0, 10000),
                (date2, 1000.0, 10000),
                (date3, 1000.0, 10000),
            ],
        };

        let (result, portfolio, _) =
            simulate_backtest(job_id, &[stock], 10_000_000.0, 1, 10.0, 5.0, 10, 0.0, 0.0);

        assert_eq!(portfolio.len(), 3);
        assert_eq!(portfolio[0].net_value, portfolio[1].net_value);
        assert_eq!(portfolio[1].net_value, portfolio[2].net_value);
        assert_eq!(result.sharpe_ratio, 0.0);
        assert_eq!(result.portfolio_volatility, 0.0);
    }

    #[test]
    fn simulate_backtest_trading_fee_deductions() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let job_id = Uuid::new_v4();

        let stock = StockData {
            code: "FEE1".to_string(),
            daily_data: vec![(date1, 1000.0, 10000), (date2, 1000.0, 10000)],
        };

        let (result, _, trades) =
            simulate_backtest(job_id, &[stock], 250_000.0, 2, 10.0, 5.0, 1, 0.15, 0.25);

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];
        assert_eq!(trade.lot, 1);
        let expected_buy_val = 1.0 * 100.0 * 1000.0;
        let expected_buy_fee = expected_buy_val * 0.0015;
        let expected_sell_val = 1.0 * 100.0 * 1000.0;
        let expected_sell_fee = expected_sell_val * 0.0025;
        let expected_total_fees = expected_buy_fee + expected_sell_fee;

        assert_eq!(trade.buy_fee, expected_buy_fee);
        assert_eq!(trade.sell_fee, expected_sell_fee);
        assert_eq!(result.total_fees, expected_total_fees);
        assert_eq!(trade.pnl, -expected_total_fees);
        assert_eq!(result.net_pnl, -expected_total_fees);
    }
}
