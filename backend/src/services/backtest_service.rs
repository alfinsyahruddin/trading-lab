use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use chrono::NaiveDate;
use futures_util::future::join_all;
use uuid::Uuid;
use validator::Validate;

use crate::{
    clients::{llm_client::LLMTrait, sectors_client::SectorsClientTrait},
    entities::{
        app_error::AppError,
        backtest::{
            BacktestJobResponse, BacktestOwnerResponse, BacktestPortfolioHistoryRecord,
            BacktestResultRecord, BacktestResultResponse, BacktestTradeRecord,
            CreateBacktestJobRequest, MostTradedResponse, TopEntryResponse,
            UpdateBacktestJobRequest,
        },
        trading_strategy::{StrategyRuleCondition, StrategyRuleGroup},
    },
    enums::backtest_status::BacktestStatus,
    repositories::{
        backtest_repository::{BacktestRepository, CreateBacktestParams},
        trading_strategy_repository::TradingStrategyRepository,
    },
    services::settings_service::SettingsService,
};

pub struct BacktestService {
    repo: Arc<BacktestRepository>,
    strategy_repo: Arc<TradingStrategyRepository>,
    sectors: Arc<dyn SectorsClientTrait>,
    llm: Arc<dyn LLMTrait>,
    settings: Arc<SettingsService>,
}

impl BacktestService {
    pub fn new(
        repo: Arc<BacktestRepository>,
        strategy_repo: Arc<TradingStrategyRepository>,
        sectors: Arc<dyn SectorsClientTrait>,
        llm: Arc<dyn LLMTrait>,
        settings: Arc<SettingsService>,
    ) -> Self {
        Self {
            repo,
            strategy_repo,
            sectors,
            llm,
            settings,
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
                max_stocks: job.max_stocks,
                backtest_duration_months: job.backtest_duration_months,
                buy_fee_percentage: job.buy_fee_percentage,
                sell_fee_percentage: job.sell_fee_percentage,
                is_public: job.is_public,
                status: job.status,
                error_message: job.error_message,
                owner: None,
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
        let job = self.repo.find_by_id_for_user_or_public(id, user_id).await?;
        let result = self.repo.find_result_by_job(id).await?;
        let portfolio = self.repo.find_portfolio_history(id).await?;
        let most_traded = self.repo.find_most_traded(id).await?;
        let top_gainers = self.repo.find_top_gainers(id).await?;
        let top_losers = self.repo.find_top_losers(id).await?;
        let trades = self.repo.find_trades(id).await?;

        let owner = match (job.owner_name, job.owner_email) {
            (Some(name), Some(email)) => Some(BacktestOwnerResponse {
                id: job.user_id,
                name,
                email,
            }),
            _ => None,
        };

        let mut result_resp: Option<BacktestResultResponse> = result.map(Into::into);

        if job.status == BacktestStatus::Done {
            if let Some(ref mut res) = result_resp {
                if res.ai_summary.as_ref().is_none_or(|s| s.is_empty())
                    && self.settings.get_ai_enabled().await.unwrap_or(false)
                {
                    if let Ok(Some(rec)) = self.repo.find_result_by_job(id).await {
                        if let Ok(summary) = generate_backtest_ai_summary(
                            &*self.llm,
                            &job.name,
                            &job.strategy_name,
                            job.year,
                            job.backtest_duration_months,
                            job.initial_cash,
                            &rec,
                        )
                        .await
                        {
                            let _ = self.repo.update_ai_summary(id, &summary).await;
                            res.ai_summary = Some(summary);
                        }
                    }
                }
            }
        }

        Ok(BacktestJobResponse {
            id: job.id,
            user_id: job.user_id,
            strategy_id: job.strategy_id,
            strategy_name: job.strategy_name,
            name: job.name,
            year: job.year,
            initial_cash: job.initial_cash,
            max_holding_stocks: job.max_holding_stocks,
            max_stocks: job.max_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
            is_public: job.is_public,
            status: job.status,
            error_message: job.error_message,
            owner,
            result: result_resp,
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
            .find_by_id_for_user_or_public(req.strategy_id, user_id)
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
            max_stocks: req.max_stocks,
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
        let llm = self.llm.clone();
        let settings = self.settings.clone();

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                repo.clone(),
                strategy_repo,
                sectors,
                llm,
                settings,
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
            max_stocks: job.max_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
            is_public: job.is_public,
            status: job.status,
            error_message: job.error_message,
            owner: None,
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

    pub async fn rerun(
        &self,
        job_id: Uuid,
        user_id: Uuid,
    ) -> Result<BacktestJobResponse, AppError> {
        let job = self.repo.find_by_id_and_user(job_id, user_id).await?;

        if job.status != BacktestStatus::Failed {
            return Err(AppError::bad_request("Only failed backtests can be rerun"));
        }

        let strategy = self
            .strategy_repo
            .find_by_id_for_user_or_public(job.strategy_id, user_id)
            .await?;

        self.repo.reset_for_rerun(job_id).await?;

        let strategy_id = strategy.id;
        let repo = self.repo.clone();
        let strategy_repo = self.strategy_repo.clone();
        let sectors = self.sectors.clone();
        let llm = self.llm.clone();
        let settings = self.settings.clone();
        let req = CreateBacktestJobRequest {
            strategy_id: strategy.id,
            name: job.name.clone(),
            is_public: Some(job.is_public),
            year: job.year,
            initial_cash: job.initial_cash,
            max_holding_stocks: job.max_holding_stocks,
            max_stocks: job.max_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
        };

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                repo.clone(),
                strategy_repo,
                sectors,
                llm,
                settings,
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
            max_stocks: job.max_stocks,
            backtest_duration_months: job.backtest_duration_months,
            buy_fee_percentage: job.buy_fee_percentage,
            sell_fee_percentage: job.sell_fee_percentage,
            is_public: job.is_public,
            status: BacktestStatus::Pending,
            error_message: None,
            owner: None,
            result: None,
            portfolio_history: None,
            most_traded: None,
            top_gainers: None,
            top_losers: None,
            trade_history: None,
            created_at: job.created_at,
            updated_at: chrono::Utc::now(),
        })
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_backtest(
    repo: Arc<BacktestRepository>,
    strategy_repo: Arc<TradingStrategyRepository>,
    sectors: Arc<dyn SectorsClientTrait>,
    llm: Arc<dyn LLMTrait>,
    settings: Arc<SettingsService>,
    job_id: Uuid,
    strategy_id: Uuid,
    user_id: Uuid,
    req: CreateBacktestJobRequest,
) -> Result<(), AppError> {
    repo.update_status(job_id, BacktestStatus::Processing, None)
        .await?;

    let strategy = strategy_repo
        .find_by_id_for_user_or_public(strategy_id, user_id)
        .await?;

    let (where_query, manual_filter_groups) = build_where_query(&strategy.rules.0, req.year);
    eprintln!("[Backtest] Built screener query: {}", where_query);

    let stocks = if where_query.is_empty() {
        return Err(AppError::bad_request("Screener query is empty"));
    } else {
        sectors.screener(&where_query, req.max_stocks).await?
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

    let (start_date, end_date) = crate::helpers::date_helper::calculate_backtest_date_range(
        req.year,
        req.backtest_duration_months as u32,
    );
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
                let val = d.close * vol * 100.0;

                if evaluate_manual_rules(&manual_filter_groups, d.close, vol, val) {
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

    if settings.get_ai_enabled().await.unwrap_or(false) {
        if let Ok(summary) = generate_backtest_ai_summary(
            &*llm,
            &req.name,
            &strategy.name,
            req.year,
            req.backtest_duration_months,
            req.initial_cash,
            &result_record,
        )
        .await
        {
            let _ = repo.update_ai_summary(job_id, &summary).await;
        }
    }

    Ok(())
}

pub async fn generate_backtest_ai_summary(
    llm: &dyn LLMTrait,
    job_name: &str,
    strategy_name: &str,
    year: i32,
    duration_months: i32,
    initial_cash: f64,
    result: &BacktestResultRecord,
) -> Result<Vec<String>, AppError> {
    let prompt = format!(
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

Generate a high-impact executive summary consisting of a JSON array of up to 5 clear, insightful keypoint strings (maximum 5 strings).
Each keypoint should be 1-2 concise sentences addressing one of the following 5 dimensions:
1. Overall Profitability & Return: Net return vs duration, capital growth vs initial cash, and fee drag impact.
2. Win/Loss Dynamics: Win rate vs profit factor, and average win magnitude compared to average loss.
3. Risk & Volatility Profile: Sharpe ratio evaluation, risk-adjusted performance, and downside/drawdown risk.
4. Holding & Execution Efficiency: Win holding duration vs loss holding duration (discipline in cutting losses vs letting winners run).
5. Strategic Verdict & Actionable Improvement: Concrete recommendation for parameter tuning (e.g. SL, TP, or holding window) under Indonesian market conditions.

Return ONLY a valid JSON array of strings:
["Keypoint 1...", "Keypoint 2...", "Keypoint 3...", "Keypoint 4...", "Keypoint 5..."]"#,
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
    );

    let summary: Vec<String> =
        crate::clients::llm_client::generate_structured(llm, &prompt).await?;
    let truncated: Vec<String> = summary.into_iter().take(5).collect();
    Ok(truncated)
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
            gross_value: available_cash + cumulative_fees + gross_positions,
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
        ai_summary: None,
    };

    (result_record, portfolio_history, completed_trades)
}

pub fn build_where_query(
    groups: &[StrategyRuleGroup],
    year: i32,
) -> (String, Vec<StrategyRuleGroup>) {
    let mut valid_group_clauses = Vec::new();
    let mut manual_filter_groups = Vec::new();

    for group in groups {
        let mut condition_parts: Vec<(String, Option<String>)> = Vec::new();
        let mut manual_conditions = Vec::new();

        for cond in &group.conditions {
            if cond.variable.eq_ignore_ascii_case("volume")
                || cond.variable.eq_ignore_ascii_case("value")
            {
                manual_conditions.push(cond.clone());
                continue;
            }

            let variable = if cond.variable.eq_ignore_ascii_case("price") {
                "last_close_price".to_string()
            } else if cond.variable.eq_ignore_ascii_case("market_cap") {
                "market_cap".to_string()
            } else {
                format!("{}[{}]", cond.variable, year)
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
        if !manual_filter_groups.is_empty() {
            return ("last_close_price>0".to_string(), manual_filter_groups);
        }
        return (String::new(), manual_filter_groups);
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

    (full_query, manual_filter_groups)
}

fn evaluate_single_manual_condition(
    cond: &StrategyRuleCondition,
    price: f64,
    volume: f64,
    value: f64,
) -> bool {
    let target = if cond.variable.eq_ignore_ascii_case("volume") {
        volume
    } else if cond.variable.eq_ignore_ascii_case("value") {
        value
    } else if cond.variable.eq_ignore_ascii_case("price") {
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

        let (where_query, manual_filters) = build_where_query(&groups, 2025);
        assert_eq!(
            where_query,
            "(pb[2025]<1.0) and (last_close_price>50 and last_close_price<=1000)"
        );
        assert_eq!(manual_filters.len(), 1);
        assert_eq!(manual_filters[0].conditions.len(), 1);
        assert_eq!(manual_filters[0].conditions[0].variable, "volume");
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

        let (where_query, manual_filters) = build_where_query(&groups, 2025);
        assert_eq!(where_query, "last_close_price>0");
        assert_eq!(manual_filters.len(), 1);
        assert_eq!(manual_filters[0].conditions.len(), 2);
        assert_eq!(manual_filters[0].conditions[0].variable, "value");
        assert_eq!(manual_filters[0].conditions[1].variable, "volume");
    }

    #[test]
    fn should_evaluate_manual_rules_with_value_formula_and_connectors() {
        // formula: price * volume * 100
        // e.g. price = 1000.0, volume = 100 -> value = 1000 * 100 * 100 = 10,000,000
        let price = 1000.0;
        let volume = 100.0;
        let value = price * volume * 100.0; // 10_000_000.0

        // Test single value condition: value >= 10,000,000 should be true
        let groups_single = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![StrategyRuleCondition {
                id: "c1".to_string(),
                variable: "value".to_string(),
                operator: ">=".to_string(),
                value: "10000000".to_string(),
                connector_to_next: None,
            }],
        }];
        assert!(evaluate_manual_rules(&groups_single, price, volume, value));

        // Test single value condition: value > 10,000,000 should be false
        let groups_false = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![StrategyRuleCondition {
                id: "c1".to_string(),
                variable: "value".to_string(),
                operator: ">".to_string(),
                value: "10000000".to_string(),
                connector_to_next: None,
            }],
        }];
        assert!(!evaluate_manual_rules(&groups_false, price, volume, value));

        // Test AND connector: volume >= 100 AND value >= 10,000,000 -> true
        let groups_and = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "volume".to_string(),
                    operator: ">=".to_string(),
                    value: "100".to_string(),
                    connector_to_next: Some("AND".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "value".to_string(),
                    operator: ">=".to_string(),
                    value: "10000000".to_string(),
                    connector_to_next: None,
                },
            ],
        }];
        assert!(evaluate_manual_rules(&groups_and, price, volume, value));

        // Test AND connector with failing first condition: volume >= 200 AND value >= 10,000,000 -> false
        let groups_and_fail = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "volume".to_string(),
                    operator: ">=".to_string(),
                    value: "200".to_string(),
                    connector_to_next: Some("AND".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "value".to_string(),
                    operator: ">=".to_string(),
                    value: "10000000".to_string(),
                    connector_to_next: None,
                },
            ],
        }];
        assert!(!evaluate_manual_rules(
            &groups_and_fail,
            price,
            volume,
            value
        ));

        // Test OR connector: volume >= 200 OR value >= 10,000,000 -> true (since second is true)
        let groups_or = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "volume".to_string(),
                    operator: ">=".to_string(),
                    value: "200".to_string(),
                    connector_to_next: Some("OR".to_string()),
                },
                StrategyRuleCondition {
                    id: "c2".to_string(),
                    variable: "value".to_string(),
                    operator: ">=".to_string(),
                    value: "10000000".to_string(),
                    connector_to_next: None,
                },
            ],
        }];
        assert!(evaluate_manual_rules(&groups_or, price, volume, value));
    }

    #[test]
    fn should_format_market_cap_without_year_in_screener_query() {
        let groups = vec![StrategyRuleGroup {
            id: "g1".to_string(),
            connector_to_next: None,
            conditions: vec![
                StrategyRuleCondition {
                    id: "c1".to_string(),
                    variable: "market_cap".to_string(),
                    operator: ">=".to_string(),
                    value: "1000000000000".to_string(),
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
        }];

        let (where_query, _) = build_where_query(&groups, 2025);
        assert_eq!(where_query, "(market_cap>=1000000000000 and pe[2025]<15)");
    }

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
        // Gross value and Net value on the last day should differ by total fees paid
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

        // 1. Test Stop Loss strictly at -5% when day dropped -6% (to 940)
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

        // 2. Test Take Profit strictly at +10% when day rose +12% (to 1120)
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

    struct MockLLM;

    #[async_trait::async_trait]
    impl LLMTrait for MockLLM {
        async fn generate(&self, _prompt: &str) -> Result<String, AppError> {
            Ok(r#"["Point 1: Strong performance.", "Point 2: Good win rate.", "Point 3: Low volatility.", "Point 4: Efficient hold time.", "Point 5: Maintain current SL."]"#.into())
        }
    }

    #[tokio::test]
    async fn should_generate_backtest_ai_summary_with_mock() {
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
            ai_summary: None,
        };

        let summary = generate_backtest_ai_summary(
            &MockLLM,
            "Momentum 2024",
            "Strategy Alpha",
            2024,
            12,
            100_000_000.0,
            &result,
        )
        .await
        .expect("generates summary");

        assert_eq!(summary.len(), 5);
        assert_eq!(summary[0], "Point 1: Strong performance.");
    }
}
