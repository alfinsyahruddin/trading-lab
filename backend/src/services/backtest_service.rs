use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use futures_util::future::join_all;
use uuid::Uuid;
use validator::Validate;

use crate::{
    clients::sectors_client::SectorsClientTrait,
    entities::{
        app_error::AppError,
        backtest::{
            BacktestJobResponse, BacktestPortfolioHistoryRecord, BacktestResultRecord,
            BacktestTradeRecord, CreateBacktestJobRequest, MostTradedResponse, TopEntryResponse,
            UpdateBacktestJobRequest,
        },
        trading_strategy::{StrategyRuleCondition, StrategyRuleGroup},
    },
    enums::backtest_status::BacktestStatus,
    repositories::{
        backtest_repository::{BacktestRepository, CreateBacktestParams},
        trading_strategy_repository::TradingStrategyRepository,
    },
};

pub struct BacktestService {
    repo: Arc<BacktestRepository>,
    strategy_repo: Arc<TradingStrategyRepository>,
    sectors: Arc<dyn SectorsClientTrait>,
}

impl BacktestService {
    pub fn new(
        repo: Arc<BacktestRepository>,
        strategy_repo: Arc<TradingStrategyRepository>,
        sectors: Arc<dyn SectorsClientTrait>,
    ) -> Self {
        Self {
            repo,
            strategy_repo,
            sectors,
        }
    }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<BacktestJobResponse>, AppError> {
        let jobs = self.repo.list_by_user(user_id).await?;
        let mut responses = Vec::with_capacity(jobs.len());

        for job in jobs {
            let result = self.repo.find_result_by_job(job.id).await?;
            let portfolio = self.repo.find_portfolio_history(job.id).await?;

            let response = BacktestJobResponse {
                id: job.id,
                user_id: job.user_id,
                strategy_id: job.strategy_id,
                strategy_name: job.strategy_name,
                name: job.name,
                year: job.year,
                initial_cash: job.initial_cash,
                max_holding_stocks: job.max_holding_stocks,
                backtest_duration_months: job.backtest_duration_months,
                buy_fee_percentage: job.buy_fee_percentage,
                sell_fee_percentage: job.sell_fee_percentage,
                is_public: job.is_public,
                status: job.status,
                error_message: job.error_message,
                result: result.map(Into::into),
                portfolio_history: Some(portfolio.into_iter().map(Into::into).collect()),
                most_traded: None,
                top_gainers: None,
                top_losers: None,
                trade_history: None,
                created_at: job.created_at,
                updated_at: job.updated_at,
            };
            responses.push(response);
        }

        Ok(responses)
    }

    pub async fn get(&self, id: Uuid, user_id: Uuid) -> Result<BacktestJobResponse, AppError> {
        let job = self.repo.find_by_id_and_user(id, user_id).await?;
        let result = self.repo.find_result_by_job(id).await?;
        let portfolio = self.repo.find_portfolio_history(id).await?;
        let most_traded = self.repo.find_most_traded(id).await?;
        let top_gainers = self.repo.find_top_gainers(id).await?;
        let top_losers = self.repo.find_top_losers(id).await?;
        let trades = self.repo.find_trades(id).await?;

        Ok(BacktestJobResponse {
            id: job.id,
            user_id: job.user_id,
            strategy_id: job.strategy_id,
            strategy_name: job.strategy_name,
            name: job.name,
            year: job.year,
            initial_cash: job.initial_cash,
            max_holding_stocks: job.max_holding_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
            is_public: job.is_public,
            status: job.status,
            error_message: job.error_message,
            result: result.map(Into::into),
            portfolio_history: Some(portfolio.into_iter().map(Into::into).collect()),
            most_traded: Some(
                most_traded
                    .into_iter()
                    .map(|r| MostTradedResponse {
                        code: r.code,
                        total: r.total,
                        pnl: r.pnl,
                        pnl_percentage: r.pnl_percentage,
                    })
                    .collect(),
            ),
            top_gainers: Some(
                top_gainers
                    .into_iter()
                    .map(|r| TopEntryResponse {
                        code: r.code,
                        pnl: r.pnl,
                        pnl_percentage: r.pnl_percentage,
                    })
                    .collect(),
            ),
            top_losers: Some(
                top_losers
                    .into_iter()
                    .map(|r| TopEntryResponse {
                        code: r.code,
                        pnl: r.pnl,
                        pnl_percentage: r.pnl_percentage,
                    })
                    .collect(),
            ),
            trade_history: Some(trades.into_iter().map(Into::into).collect()),
            created_at: job.created_at,
            updated_at: job.updated_at,
        })
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repo.delete(id, user_id).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        req: UpdateBacktestJobRequest,
    ) -> Result<BacktestJobResponse, AppError> {
        req.validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        if req.is_empty() {
            return Err(AppError::bad_request("No fields provided for update"));
        }

        if let Some(is_public) = req.is_public {
            self.repo.update_visibility(id, user_id, is_public).await?;
        }

        self.get(id, user_id).await
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        req: CreateBacktestJobRequest,
    ) -> Result<BacktestJobResponse, AppError> {
        req.validate()
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        let valid_durations = [1, 3, 6, 12];
        if !valid_durations.contains(&req.backtest_duration_months) {
            return Err(AppError::BadRequest(
                "backtest_duration_months must be 1, 3, 6, or 12".into(),
            ));
        }

        let strategy = self
            .strategy_repo
            .find_by_id_and_user(req.strategy_id, user_id)
            .await?;

        let is_public = req.is_public.unwrap_or(false);
        let params = CreateBacktestParams {
            user_id,
            strategy_id: strategy.id,
            strategy_name: strategy.name.clone(),
            name: req.name.clone(),
            year: req.year,
            initial_cash: req.initial_cash,
            max_holding_stocks: req.max_holding_stocks,
            backtest_duration_months: req.backtest_duration_months,
            buy_fee_percentage: req.buy_fee_percentage,
            sell_fee_percentage: req.sell_fee_percentage,
            is_public,
        };

        let job = self.repo.create(params).await?;

        let job_id = job.id;
        let strategy_id = strategy.id;
        let repo = self.repo.clone();
        let strategy_repo = self.strategy_repo.clone();
        let sectors = self.sectors.clone();

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                repo.clone(),
                strategy_repo,
                sectors,
                job_id,
                strategy_id,
                user_id,
                req,
            )
            .await
            {
                eprintln!("Backtest {} failed: {:?}", job_id, e);
                let _ = repo
                    .update_status(job_id, BacktestStatus::Failed, Some(&e.to_string()))
                    .await;
            }
        });

        Ok(BacktestJobResponse {
            id: job.id,
            user_id: job.user_id,
            strategy_id: job.strategy_id,
            strategy_name: job.strategy_name,
            name: job.name,
            year: job.year,
            initial_cash: job.initial_cash,
            max_holding_stocks: job.max_holding_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
            is_public: job.is_public,
            status: job.status,
            error_message: job.error_message,
            result: None,
            portfolio_history: None,
            most_traded: None,
            top_gainers: None,
            top_losers: None,
            trade_history: None,
            created_at: job.created_at,
            updated_at: job.updated_at,
        })
    }
}

async fn run_backtest(
    repo: Arc<BacktestRepository>,
    strategy_repo: Arc<TradingStrategyRepository>,
    sectors: Arc<dyn SectorsClientTrait>,
    job_id: Uuid,
    strategy_id: Uuid,
    user_id: Uuid,
    req: CreateBacktestJobRequest,
) -> Result<(), AppError> {
    repo.update_status(job_id, BacktestStatus::Processing, None)
        .await?;

    let strategy = strategy_repo
        .find_by_id_and_user(strategy_id, user_id)
        .await?;

    let (where_query, volume_filters) = build_where_query(&strategy.rules.0, req.year);
    eprintln!("[Backtest] Built screener query: {}", where_query);

    let stocks = if where_query.is_empty() {
        return Err(AppError::bad_request("Screener query is empty"));
    } else {
        sectors.screener(&where_query).await?
    };

    if stocks.is_empty() {
        return Err(AppError::bad_request(
            "No stocks matched the screening criteria",
        ));
    }
    eprintln!(
        "[Backtest] Screened {} stocks: {:?}",
        stocks.len(),
        stocks.iter().map(|s| &s.symbol).collect::<Vec<_>>()
    );

    let today = Utc::now().naive_utc().date();
    let mut start_date = NaiveDate::from_ymd_opt(req.year + 1, 1, 1).unwrap();
    if start_date > today {
        start_date = NaiveDate::from_ymd_opt(req.year, 1, 1).unwrap();
    }

    let raw_end_date = chrono::NaiveDate::checked_add_months(
        start_date,
        chrono::Months::new(req.backtest_duration_months as u32),
    )
    .unwrap();

    let end_date = std::cmp::min(raw_end_date, today);
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    eprintln!(
        "[Backtest] Backtest date range: {} to {}",
        start_str, end_str
    );

    let mut fetch_futures = Vec::new();
    for stock in &stocks {
        let symbol = stock.symbol.clone();
        let start = start_str.clone();
        let end = end_str.clone();
        let sectors_client = sectors.clone();

        fetch_futures.push(async move {
            let data = sectors_client
                .daily_transactions(&symbol, &start, &end)
                .await;
            (symbol, data)
        });
    }

    let results = join_all(fetch_futures).await;

    let mut stock_data_list = Vec::new();
    for (symbol, data_result) in results {
        if let Ok(data) = data_result {
            let mut valid_data = Vec::new();
            for d in data {
                let date = NaiveDate::parse_from_str(&d.date, "%Y-%m-%d").unwrap();
                let vol = d.volume as f64;

                // apply volume filters
                let mut pass = true;
                for f in &volume_filters {
                    let val: f64 = f.value.parse().unwrap_or(0.0);
                    #[allow(clippy::collapsible_match)]
                    match f.operator.as_str() {
                        ">" => {
                            if vol <= val {
                                pass = false
                            }
                        }
                        "<" => {
                            if vol >= val {
                                pass = false
                            }
                        }
                        ">=" => {
                            if vol < val {
                                pass = false
                            }
                        }
                        "<=" => {
                            if vol > val {
                                pass = false
                            }
                        }
                        "=" => {
                            if vol != val {
                                pass = false
                            }
                        }
                        "!=" => {
                            if vol == val {
                                pass = false
                            }
                        }
                        _ => {}
                    }
                }

                if pass {
                    valid_data.push((date, d.close, d.volume));
                }
            }

            if !valid_data.is_empty() {
                valid_data.sort_by_key(|&(date, _, _)| date);
                stock_data_list.push(StockData {
                    code: symbol,
                    daily_data: valid_data,
                });
            }
        }
    }

    if stock_data_list.is_empty() {
        return Err(AppError::bad_request(
            "No stock data available after filtering",
        ));
    }

    let (result_record, portfolio_records, trade_records) = simulate_backtest(
        job_id,
        &stock_data_list,
        req.initial_cash,
        req.max_holding_stocks,
        strategy.tp_percentage,
        strategy.sl_percentage,
        strategy.max_holding_period_days,
        req.buy_fee_percentage,
        req.sell_fee_percentage,
    );

    repo.save_result(job_id, &result_record, &portfolio_records, &trade_records)
        .await?;
    repo.update_status(job_id, BacktestStatus::Done, None)
        .await?;

    Ok(())
}

struct StockData {
    code: String,
    daily_data: Vec<(NaiveDate, f64, u64)>, // date, close, volume
}

#[derive(Debug, Clone)]
struct Position {
    code: String,
    lots: i32,
    buy_price: f64,
    buy_value: f64,
    buy_fee: f64,
    buy_date: NaiveDate,
}

#[allow(clippy::too_many_arguments)]
fn simulate_backtest(
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
        let current_stock_data = date_to_stock_data.get(&current_date).unwrap();

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
                if current_close >= tp_price {
                    exit_reason = Some("TAKE_PROFIT");
                } else if current_close <= sl_price {
                    exit_reason = Some("STOP_LOSS");
                } else if hold_days >= (max_holding_period_days as i64) {
                    exit_reason = Some("MAX_HOLDING_TIME");
                }

                if let Some(reason) = exit_reason {
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
                        exit_reason: reason.to_string(),
                        lot: pos.lots,
                        buy_price: pos.buy_price,
                        buy_value: pos.buy_value,
                        sell_price: current_close,
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
            // Find last known close price
            let mut last_close = pos.buy_price;
            if let Some(&c) = current_stock_data.get(&pos.code) {
                last_close = c;
            } else {
                // Should look backward in stock.daily_data in real impl, but this approximation is okay
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
            gross_value: available_cash + gross_positions,
        });
    }

    // Force close remaining open positions
    if let Some(last_date) = portfolio_history.last().map(|p| p.date) {
        let current_stock_data = date_to_stock_data.get(&last_date).unwrap();
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
    let mut wins = 0;
    let mut losses = 0;
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

    let win_rate = if trades_processed > 0 {
        (wins as f64) / (trades_processed as f64) * 100.0
    } else {
        0.0
    };
    let profit_factor = if sum_losing < 0.0 {
        sum_winning / sum_losing.abs()
    } else {
        sum_winning
    };

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

    let mut sharpe_ratio = 0.0;
    let mut portfolio_volatility = 0.0;

    if !daily_returns.is_empty() {
        let avg_daily_return: f64 =
            daily_returns.iter().sum::<f64>() / (daily_returns.len() as f64);
        let variance: f64 = daily_returns
            .iter()
            .map(|&r| (r - avg_daily_return).powi(2))
            .sum::<f64>()
            / (daily_returns.len() as f64);
        let std_daily_return = variance.sqrt();

        let risk_free_daily = 0.02 / 252.0;
        if std_daily_return > 0.0 {
            sharpe_ratio =
                (avg_daily_return - risk_free_daily) / std_daily_return * (252.0_f64).sqrt();
        }
        portfolio_volatility = std_daily_return * (252.0_f64).sqrt() * 100.0;
    }

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
    };

    (result_record, portfolio_history, completed_trades)
}

pub fn build_where_query(
    groups: &[StrategyRuleGroup],
    year: i32,
) -> (String, Vec<StrategyRuleCondition>) {
    let mut valid_group_clauses = Vec::new();
    let mut volume_filters = Vec::new();

    for group in groups {
        let mut condition_parts: Vec<(String, Option<String>)> = Vec::new();

        for cond in &group.conditions {
            if cond.variable.eq_ignore_ascii_case("volume") {
                volume_filters.push(cond.clone());
                continue;
            }

            let variable = if cond.variable.eq_ignore_ascii_case("price") {
                "last_close_price".to_string()
            } else {
                format!("{}[{}]", cond.variable, year)
            };

            let cond_str = format!("{}{}{}", variable, cond.operator, cond.value);
            let conn = cond.connector_to_next.as_deref().map(|c| c.to_lowercase());
            condition_parts.push((cond_str, conn));
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
        return (String::new(), volume_filters);
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

    (full_query, volume_filters)
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

        let (where_query, volume_filters) = build_where_query(&groups, 2025);
        assert_eq!(
            where_query,
            "(pb[2025]<1.0) and (last_close_price>50 and last_close_price<=1000)"
        );
        assert_eq!(volume_filters.len(), 1);
        assert_eq!(volume_filters[0].variable, "volume");
    }
}
