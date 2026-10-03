use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDate;
use futures_util::StreamExt;
use redis::aio::ConnectionManager;
use uuid::Uuid;
use validator::Validate;

pub use crate::entities::backtest::{Position, StockData};
pub use crate::services::backtest::engine::simulate_backtest;
pub use crate::services::backtest::rules::{
    build_where_query, evaluate_foreign_flow_rules, evaluate_manual_rules, StockForeignFlowValues,
};

use crate::{
    clients::{llm_client::LLMTrait, sectors_client::SectorsClientTrait},
    entities::{
        app_error::AppError,
        backtest::{
            BacktestJobResponse, BacktestOwnerResponse, BacktestResultRecord,
            BacktestResultResponse, CreateBacktestJobRequest, MostTradedResponse, TopEntryResponse,
            TradeHistoryResponse, UpdateBacktestJobRequest,
        },
        sectors::{DailyTransaction, ForeignFlowResponse, ScreenerCompany},
    },
    enums::backtest_status::BacktestStatus,
    helpers::prompt_helper,
    repositories::{
        backtest_repository::{BacktestRepository, CreateBacktestParams},
        trading_strategy_repository::TradingStrategyRepository,
    },
    services::settings_service::SettingsService,
};

pub struct BacktestService {
    repo: BacktestRepository,
    strategy_repo: TradingStrategyRepository,
    sectors: Arc<dyn SectorsClientTrait>,
    llm: Arc<dyn LLMTrait>,
    settings: SettingsService,
    redis: ConnectionManager,
}

impl BacktestService {
    pub fn new(
        repo: BacktestRepository,
        strategy_repo: TradingStrategyRepository,
        sectors: Arc<dyn SectorsClientTrait>,
        llm: Arc<dyn LLMTrait>,
        settings: SettingsService,
        redis: ConnectionManager,
    ) -> Self {
        Self {
            repo,
            strategy_repo,
            sectors,
            llm,
            settings,
            redis,
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
                start_date: job.start_date,
                end_date: job.end_date,
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
                if res.ai_insights.as_ref().is_none_or(|s| s.is_empty())
                    && self.settings.get_ai_enabled().await.unwrap_or(false)
                {
                    if let Ok(Some(rec)) = self.repo.find_result_by_job(id).await {
                        if let Ok(insights) = generate_backtest_ai_insights(
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
                            let _ = self.repo.update_ai_insights(id, &insights).await;
                            res.ai_insights = Some(insights);
                        }
                    }
                }
            }
        }

        let trade_info_map = self
            .get_trade_query_values(
                job.id,
                job.strategy_id,
                job.user_id,
                job.year,
                job.max_stocks,
            )
            .await;

        let trade_history: Vec<TradeHistoryResponse> = trades
            .into_iter()
            .map(|t| {
                let code_upper = t.code.to_uppercase();
                let clean_code = t.code.replace(".JK", "").trim().to_uppercase();
                let info = trade_info_map
                    .get(&code_upper)
                    .or_else(|| trade_info_map.get(&clean_code));
                let company_name = info
                    .and_then(|v| v.get("company_name"))
                    .and_then(|c| c.as_str())
                    .map(String::from);
                let query_values = info
                    .and_then(|v| v.get("query_values"))
                    .cloned()
                    .filter(|q| !q.is_null());
                TradeHistoryResponse::from_record_with_info(t, company_name, query_values)
            })
            .collect();

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
            trade_history: Some(trade_history),
            start_date: job.start_date,
            end_date: job.end_date,
            created_at: job.created_at,
            updated_at: job.updated_at,
        })
    }

    async fn get_trade_query_values(
        &self,
        job_id: Uuid,
        strategy_id: Uuid,
        user_id: Uuid,
        year: i32,
        max_stocks: i32,
    ) -> HashMap<String, serde_json::Value> {
        let key = format!("backtest:trade_query_values:{}", job_id);
        let mut redis_conn = self.redis.clone();
        let cached: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut redis_conn)
            .await
            .unwrap_or(None);

        if let Some(json_str) = cached {
            if let Ok(map) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&json_str) {
                if !map.is_empty() {
                    return map;
                }
            }
        }

        // Fallback: check sectors:screener cache
        if let Ok(strategy) = self
            .strategy_repo
            .find_by_id_for_user_or_public(strategy_id, user_id)
            .await
        {
            let (where_query, _, _) = build_where_query(&strategy.rules.0, year);
            if !where_query.is_empty() {
                let screener_key = format!("sectors:screener:{}:{}", where_query, max_stocks);
                let screener_cached: Option<String> = redis::cmd("GET")
                    .arg(&screener_key)
                    .query_async(&mut redis_conn)
                    .await
                    .unwrap_or(None);

                if let Some(s_json) = screener_cached {
                    if let Ok(companies) = serde_json::from_str::<Vec<ScreenerCompany>>(&s_json) {
                        let map = build_trade_info_map(&companies);
                        if !map.is_empty() {
                            if let Ok(json_str) = serde_json::to_string(&map) {
                                let _: Result<(), _> = redis::cmd("SET")
                                    .arg(&key)
                                    .arg(&json_str)
                                    .query_async(&mut redis_conn)
                                    .await;
                            }
                            return map;
                        }
                    }
                }
            }
        }

        HashMap::new()
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let key = format!("backtest:trade_query_values:{}", id);
        let mut redis_conn = self.redis.clone();
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut redis_conn)
            .await;
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

        let (duration_months, start_date, end_date) =
            if let (Some(start), Some(end)) = (req.start_date, req.end_date) {
                let min_date = chrono::NaiveDate::from_ymd_opt(req.year + 1, 1, 1)
                    .ok_or_else(|| AppError::bad_request("Invalid backtest data year"))?;
                let today = chrono::Utc::now().date_naive();

                if start < min_date {
                    return Err(AppError::bad_request(format!(
                        "start_date cannot be earlier than {}",
                        min_date
                    )));
                }
                if start > end {
                    return Err(AppError::bad_request("start_date cannot be after end_date"));
                }
                if end > today {
                    return Err(AppError::bad_request("end_date cannot exceed current date"));
                }

                let days = (end - start).num_days() + 1;
                let calculated_months = ((days as f64) / 30.0).ceil().max(1.0) as i32;
                let duration = if req.backtest_duration_months > 0 {
                    req.backtest_duration_months
                } else {
                    calculated_months
                };
                (duration, Some(start), Some(end))
            } else {
                let valid_durations = [1, 3, 6, 12];
                if !valid_durations.contains(&req.backtest_duration_months) {
                    return Err(AppError::BadRequest(
                        "backtest_duration_months must be 1, 3, 6, or 12".into(),
                    ));
                }
                (req.backtest_duration_months, None, None)
            };

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
            backtest_duration_months: duration_months,
            buy_fee_percentage: req.buy_fee_percentage,
            sell_fee_percentage: req.sell_fee_percentage,
            is_public,
            start_date,
            end_date,
        };

        let job = self.repo.create(params).await?;

        let job_id = job.id;
        let strategy_id = strategy.id;
        let repo = self.repo.clone();
        let strategy_repo = self.strategy_repo.clone();
        let sectors = self.sectors.clone();
        let llm = self.llm.clone();
        let settings = self.settings.clone();
        let redis = self.redis.clone();

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                &repo,
                &strategy_repo,
                sectors,
                llm.as_ref(),
                &settings,
                &redis,
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
            start_date: job.start_date,
            end_date: job.end_date,
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
        let redis = self.redis.clone();
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
            start_date: job.start_date,
            end_date: job.end_date,
        };

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                &repo,
                &strategy_repo,
                sectors,
                llm.as_ref(),
                &settings,
                &redis,
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
            start_date: job.start_date,
            end_date: job.end_date,
            created_at: job.created_at,
            updated_at: chrono::Utc::now(),
        })
    }
}

fn build_trade_info_map(companies: &[ScreenerCompany]) -> HashMap<String, serde_json::Value> {
    let mut map = HashMap::new();
    for stock in companies {
        let val = serde_json::json!({
            "company_name": stock.company_name,
            "query_values": stock.query_values,
        });
        map.insert(stock.symbol.to_uppercase(), val.clone());
        let clean = stock.symbol.replace(".JK", "").trim().to_uppercase();
        map.insert(clean, val);
    }
    map
}

/// Builds the Redis cache keys for a stock across all date chunks.
pub fn build_daily_chunk_keys(symbol: &str, chunks: &[(NaiveDate, NaiveDate)]) -> Vec<String> {
    let clean_symbol = symbol.trim_end_matches(".JK");
    chunks
        .iter()
        .map(|(chunk_start, chunk_end)| {
            format!(
                "sectors:daily:{}:{}:{}",
                clean_symbol,
                chunk_start.format("%Y-%m-%d"),
                chunk_end.format("%Y-%m-%d")
            )
        })
        .collect()
}

/// Retrieves cached daily transactions for screened stocks from Redis.
/// Returns:
/// - A map of `(symbol -> Vec<DailyTransaction>)` for stocks where ALL date chunks were found in cache.
/// - A vector of `ScreenerCompany` for stocks that were NOT completely found in Redis.
pub async fn get_cached_stock_transactions(
    redis: &ConnectionManager,
    stocks: &[ScreenerCompany],
    chunks: &[(NaiveDate, NaiveDate)],
) -> (HashMap<String, Vec<DailyTransaction>>, Vec<ScreenerCompany>) {
    if stocks.is_empty() {
        return (HashMap::new(), Vec::new());
    }
    if chunks.is_empty() {
        return (HashMap::new(), stocks.to_vec());
    }

    let mut all_keys: Vec<String> = Vec::with_capacity(stocks.len() * chunks.len());
    for stock in stocks {
        all_keys.extend(build_daily_chunk_keys(&stock.symbol, chunks));
    }

    let mut redis_conn = redis.clone();
    let mget_result: Option<Vec<Option<String>>> = redis::cmd("MGET")
        .arg(&all_keys)
        .query_async(&mut redis_conn)
        .await
        .ok();

    let mut cached_stocks_data = HashMap::new();
    let mut stocks_to_fetch = Vec::new();
    let chunk_count = chunks.len();

    if let Some(all_cached_chunks) = mget_result {
        if all_cached_chunks.len() == all_keys.len() {
            for (stock_idx, stock) in stocks.iter().enumerate() {
                let start_idx = stock_idx * chunk_count;
                let end_idx = start_idx + chunk_count;
                let stock_chunks = &all_cached_chunks[start_idx..end_idx];

                let mut stock_data = Vec::new();
                let mut all_found = true;

                for chunk_opt in stock_chunks {
                    if let Some(json_str) = chunk_opt {
                        if let Ok(transactions) =
                            serde_json::from_str::<Vec<DailyTransaction>>(json_str)
                        {
                            stock_data.extend(transactions);
                        } else {
                            all_found = false;
                            break;
                        }
                    } else {
                        all_found = false;
                        break;
                    }
                }

                if all_found {
                    eprintln!(
                        "[Backtest] Stock {} retrieved from Redis cache ({} transactions)",
                        stock.symbol,
                        stock_data.len()
                    );
                    cached_stocks_data.insert(stock.symbol.clone(), stock_data);
                } else {
                    eprintln!(
                        "[Backtest] Stock {} not found in Redis, queued for API fetch",
                        stock.symbol
                    );
                    stocks_to_fetch.push(stock.clone());
                }
            }
            return (cached_stocks_data, stocks_to_fetch);
        }
    }

    eprintln!("[Backtest] Redis cache lookup failed or returned mismatched count; fetching all stocks via API");
    (HashMap::new(), stocks.to_vec())
}

async fn get_cached_foreign_flow(
    redis: &ConnectionManager,
    stocks: &[ScreenerCompany],
    start: &str,
    end: &str,
) -> (HashMap<String, ForeignFlowResponse>, Vec<ScreenerCompany>) {
    if stocks.is_empty() {
        return (HashMap::new(), Vec::new());
    }

    let mut all_keys = Vec::with_capacity(stocks.len());
    for stock in stocks {
        let clean = stock.symbol.trim_end_matches(".JK");
        all_keys.push(format!("sectors:foreign_flow:{}:{}:{}", clean, start, end));
    }

    let mut redis_conn = redis.clone();
    let mut cmd = redis::cmd("MGET");
    for key in &all_keys {
        cmd.arg(key);
    }

    let mget_result: Result<Vec<Option<String>>, _> = cmd.query_async(&mut redis_conn).await;

    let mut cached = HashMap::new();
    let mut to_fetch = Vec::new();

    if let Ok(results) = mget_result {
        if results.len() == stocks.len() {
            for (stock, res_opt) in stocks.iter().zip(results) {
                if let Some(json_str) = res_opt {
                    if let Ok(data) = serde_json::from_str::<ForeignFlowResponse>(&json_str) {
                        eprintln!(
                            "[Backtest] Stock {} foreign flow retrieved from Redis cache ({} data points)",
                            stock.symbol,
                            data.data.len()
                        );
                        cached.insert(stock.symbol.clone(), data);
                        continue;
                    }
                }
                eprintln!(
                    "[Backtest] Stock {} foreign flow not found in Redis, queued for API fetch",
                    stock.symbol
                );
                to_fetch.push(stock.clone());
            }
            return (cached, to_fetch);
        }
    }

    (HashMap::new(), stocks.to_vec())
}

/// Converts a raw [`DailyTransaction`] list into sorted `(date, close, volume)` triples.
fn process_transactions(data: Vec<DailyTransaction>) -> Vec<(NaiveDate, f64, u64)> {
    let mut valid = Vec::new();
    for d in data {
        let Some(date) = NaiveDate::parse_from_str(&d.date, "%Y-%m-%d").ok() else {
            continue;
        };
        valid.push((date, d.close, d.volume));
    }
    valid.sort_by_key(|&(date, _, _)| date);
    valid
}

#[allow(clippy::too_many_arguments)]
async fn run_backtest(
    repo: &BacktestRepository,
    strategy_repo: &TradingStrategyRepository,
    sectors: Arc<dyn SectorsClientTrait>,
    llm: &dyn LLMTrait,
    settings: &SettingsService,
    redis: &ConnectionManager,
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

    let (where_query, manual_filter_groups, foreign_flow_filter_groups) =
        build_where_query(&strategy.rules.0, req.year);
    eprintln!("[Backtest] Built screener query: {}", where_query);

    let mut stocks = if where_query.is_empty() {
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

    let (start_date, end_date) = match (req.start_date, req.end_date) {
        (Some(start), Some(end)) => (start, end),
        _ => crate::helpers::date_helper::calculate_backtest_date_range(
            req.year,
            req.backtest_duration_months as u32,
        ),
    };
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    eprintln!(
        "[Backtest] Backtest date range: {} to {}",
        start_str, end_str
    );

    // ── Foreign Flow Pre-Filtering ───────────────────────────────────────────
    if !foreign_flow_filter_groups.is_empty() {
        let mut max_days = 7u64;
        for group in &foreign_flow_filter_groups {
            for cond in &group.conditions {
                let v = cond.variable.trim();
                if v.eq_ignore_ascii_case("last_3_months_foreign_flow") {
                    max_days = max_days.max(90);
                } else if v.eq_ignore_ascii_case("last_1_month_foreign_flow") {
                    max_days = max_days.max(30);
                }
            }
        }

        let end_date_ff = start_date
            .checked_sub_days(chrono::Days::new(1))
            .unwrap_or(start_date);
        let start_date_ff = start_date
            .checked_sub_days(chrono::Days::new(max_days))
            .unwrap_or(end_date_ff);
        let start_str_ff = start_date_ff.format("%Y-%m-%d").to_string();
        let end_str_ff = end_date_ff.format("%Y-%m-%d").to_string();

        let cutoff_1w = start_date
            .checked_sub_days(chrono::Days::new(7))
            .unwrap_or(end_date_ff);
        let cutoff_1m = start_date
            .checked_sub_days(chrono::Days::new(30))
            .unwrap_or(end_date_ff);
        let cutoff_3m = start_date
            .checked_sub_days(chrono::Days::new(90))
            .unwrap_or(end_date_ff);

        eprintln!(
            "[Backtest] Foreign flow pre-filtering window: {} to {} (max {} days)",
            start_str_ff, end_str_ff, max_days
        );

        let (mut cached_ff, ff_to_fetch) =
            get_cached_foreign_flow(redis, &stocks, &start_str_ff, &end_str_ff).await;

        let mut fetched_ff: HashMap<String, ForeignFlowResponse> = HashMap::new();

        if !ff_to_fetch.is_empty() {
            eprintln!(
                "[Backtest] Fetching foreign flow for {} stock(s) via rate-limited Sectors client",
                ff_to_fetch.len()
            );

            let futures = ff_to_fetch.into_iter().map(|stock| {
                let s = sectors.clone();
                let symbol = stock.symbol.clone();
                let start = start_str_ff.clone();
                let end = end_str_ff.clone();
                async move {
                    let result = s.foreign_flow(&symbol, &start, &end).await;
                    (symbol, result)
                }
            });

            let results = futures_util::stream::iter(futures)
                .buffer_unordered(25)
                .collect::<Vec<_>>()
                .await;

            for (symbol, data_result) in results {
                if let Ok(data) = data_result {
                    fetched_ff.insert(symbol, data);
                }
            }
        }

        let mut surviving_stocks = Vec::new();
        for stock in stocks {
            let clean_symbol = stock.symbol.trim_end_matches(".JK");
            let ff_resp_opt = cached_ff
                .remove(&stock.symbol)
                .or_else(|| cached_ff.remove(clean_symbol))
                .or_else(|| fetched_ff.remove(&stock.symbol))
                .or_else(|| fetched_ff.remove(clean_symbol));

            let mut sum_1w: f64 = 0.0;
            let mut sum_1m: f64 = 0.0;
            let mut sum_3m: f64 = 0.0;

            if let Some(ff_resp) = ff_resp_opt {
                for point in ff_resp.data {
                    if let Ok(point_date) =
                        chrono::NaiveDate::parse_from_str(&point.date, "%Y-%m-%d")
                    {
                        if point_date <= end_date_ff {
                            let inflow = point.net_foreign_inflow.unwrap_or(0) as f64;
                            if point_date >= cutoff_1w {
                                sum_1w += inflow;
                            }
                            if point_date >= cutoff_1m {
                                sum_1m += inflow;
                            }
                            if point_date >= cutoff_3m {
                                sum_3m += inflow;
                            }
                        }
                    }
                }
            }

            let values = StockForeignFlowValues {
                last_1_week: sum_1w,
                last_1_month: sum_1m,
                last_3_months: sum_3m,
            };

            if evaluate_foreign_flow_rules(&foreign_flow_filter_groups, &values) {
                surviving_stocks.push(stock);
            }
        }

        eprintln!(
            "[Backtest] Foreign flow screening passed: {} stock(s)",
            surviving_stocks.len()
        );

        if surviving_stocks.is_empty() {
            return Err(AppError::bad_request(
                "No stocks matched the foreign flow screening criteria",
            ));
        }

        stocks = surviving_stocks;
    }

    let trade_info_map = build_trade_info_map(&stocks);
    if let Ok(json_str) = serde_json::to_string(&trade_info_map) {
        let key = format!("backtest:trade_query_values:{}", job_id);
        let mut redis_conn = redis.clone();
        let _: Result<(), _> = redis::cmd("SET")
            .arg(&key)
            .arg(&json_str)
            .query_async(&mut redis_conn)
            .await;
    }

    // ── Rate-limited parallel fetch ───────────────────────────────────────────
    // Before dispatching API requests, check Redis first for cached transactions.
    use crate::clients::sectors_client::compute_date_chunks;

    let chunks = compute_date_chunks(start_date, end_date);
    let chunks_per_stock = chunks.len().max(1);

    let (mut cached_stocks_data, stocks_to_fetch) =
        get_cached_stock_transactions(redis, &stocks, &chunks).await;

    eprintln!(
        "[Backtest] Screener returned {} stock(s): {} found in Redis, {} to fetch from API",
        stocks.len(),
        cached_stocks_data.len(),
        stocks_to_fetch.len()
    );

    let mut fetched_stocks_data: HashMap<String, Vec<DailyTransaction>> = HashMap::new();

    if !stocks_to_fetch.is_empty() {
        eprintln!(
            "[Backtest] Fetching daily transactions for {} stock(s) ({} chunk(s) per stock) via rate-limited Sectors client",
            stocks_to_fetch.len(),
            chunks_per_stock
        );

        let futures = stocks_to_fetch.into_iter().map(|stock| {
            let s = sectors.clone();
            let symbol = stock.symbol.clone();
            let start = start_str.clone();
            let end = end_str.clone();
            async move {
                let result = s.daily_transactions(&symbol, &start, &end).await;
                (symbol, result)
            }
        });

        let results = futures_util::stream::iter(futures)
            .buffer_unordered(25)
            .collect::<Vec<_>>()
            .await;

        for (symbol, data_result) in results {
            if let Ok(data) = data_result {
                fetched_stocks_data.insert(symbol, data);
            }
        }
    } else {
        eprintln!("[Backtest] All stocks found in Redis cache; skipping API fetch");
    }

    let mut stock_data_list: Vec<StockData> = Vec::new();
    for stock in &stocks {
        let clean_symbol = stock.symbol.trim_end_matches(".JK");
        let transactions_opt = cached_stocks_data
            .remove(&stock.symbol)
            .or_else(|| cached_stocks_data.remove(clean_symbol))
            .or_else(|| fetched_stocks_data.remove(&stock.symbol))
            .or_else(|| fetched_stocks_data.remove(clean_symbol));

        if let Some(transactions) = transactions_opt {
            let valid = process_transactions(transactions);
            if !valid.is_empty() {
                stock_data_list.push(StockData {
                    code: stock.symbol.clone(),
                    daily_data: valid,
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
        &manual_filter_groups,
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
        if let Ok(insights) = generate_backtest_ai_insights(
            llm,
            &req.name,
            &strategy.name,
            req.year,
            req.backtest_duration_months,
            req.initial_cash,
            &result_record,
        )
        .await
        {
            let _ = repo.update_ai_insights(job_id, &insights).await;
        }
    }

    Ok(())
}

pub async fn generate_backtest_ai_insights(
    llm: &dyn LLMTrait,
    job_name: &str,
    strategy_name: &str,
    year: i32,
    duration_months: i32,
    initial_cash: f64,
    result: &BacktestResultRecord,
) -> Result<Vec<String>, AppError> {
    let prompt = prompt_helper::get_backtest_ai_insights_prompt(
        job_name,
        strategy_name,
        year,
        duration_months,
        initial_cash,
        result,
    );

    let insights: Vec<String> =
        crate::clients::llm_client::generate_structured(llm, &prompt).await?;
    let truncated: Vec<String> = insights.into_iter().take(4).collect();

    Ok(truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLLM;

    #[async_trait::async_trait]
    impl LLMTrait for MockLLM {
        async fn generate(&self, _prompt: &str) -> Result<String, AppError> {
            Ok(r#"["Point 1: Strong performance.", "Point 2: Low volatility.", "Point 3: Efficient hold time.", "Point 4: Maintain current SL."]"#.into())
        }
    }

    #[tokio::test]
    async fn should_generate_backtest_ai_insights_with_mock() {
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

        let insights = generate_backtest_ai_insights(
            &MockLLM,
            "Momentum 2024",
            "Strategy Alpha",
            2024,
            12,
            100_000_000.0,
            &result,
        )
        .await
        .expect("generates insights");

        assert_eq!(insights.len(), 4);
        assert_eq!(insights[0], "Point 1: Strong performance.");
    }

    #[test]
    fn should_build_daily_chunk_keys_correctly() {
        let chunks = vec![
            (
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
            ),
            (
                NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 6, 29).unwrap(),
            ),
        ];

        let keys_jk = build_daily_chunk_keys("BBCA.JK", &chunks);
        assert_eq!(
            keys_jk,
            vec![
                "sectors:daily:BBCA:2025-01-01:2025-03-31",
                "sectors:daily:BBCA:2025-04-01:2025-06-29"
            ]
        );

        let keys_plain = build_daily_chunk_keys("BBCA", &chunks);
        assert_eq!(keys_plain, keys_jk);
    }

    #[tokio::test]
    async fn should_partition_cached_and_uncached_stocks_with_redis() {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://:tradinglab123@127.0.0.1:6379".to_string());
        let client = match redis::Client::open(redis_url) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut conn = match redis::aio::ConnectionManager::new(client).await {
            Ok(c) => c,
            Err(_) => return,
        };

        let chunk = (
            NaiveDate::from_ymd_opt(2099, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2099, 3, 31).unwrap(),
        );
        let chunks = vec![chunk];

        let key_cached = "sectors:daily:TESTCACHED:2099-01-01:2099-03-31";
        let mock_tx = vec![DailyTransaction {
            symbol: "TESTCACHED.JK".to_string(),
            date: "2099-01-05".to_string(),
            close: 1000.0,
            volume: 500,
            market_cap: 10_000_000_000.0,
        }];
        let tx_json = serde_json::to_string(&mock_tx).unwrap();

        let _: Result<(), _> = redis::cmd("SET")
            .arg(key_cached)
            .arg(&tx_json)
            .query_async(&mut conn)
            .await;

        let stocks = vec![
            ScreenerCompany {
                symbol: "TESTCACHED.JK".to_string(),
                company_name: "Test Cached Co".to_string(),
                query_values: None,
            },
            ScreenerCompany {
                symbol: "TESTUNCACHED.JK".to_string(),
                company_name: "Test Uncached Co".to_string(),
                query_values: None,
            },
        ];

        let (cached_map, uncached) = get_cached_stock_transactions(&conn, &stocks, &chunks).await;

        // Cleanup
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(key_cached)
            .query_async(&mut conn)
            .await;

        assert_eq!(cached_map.len(), 1);
        assert!(cached_map.contains_key("TESTCACHED.JK"));
        assert_eq!(cached_map["TESTCACHED.JK"].len(), 1);
        assert_eq!(cached_map["TESTCACHED.JK"][0].close, 1000.0);

        assert_eq!(uncached.len(), 1);
        assert_eq!(uncached[0].symbol, "TESTUNCACHED.JK");
    }

    #[tokio::test]
    async fn should_partition_cached_and_uncached_foreign_flow_with_redis() {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://:tradinglab123@127.0.0.1:6379".to_string());
        let client = match redis::Client::open(redis_url) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut conn = match redis::aio::ConnectionManager::new(client).await {
            Ok(c) => c,
            Err(_) => return,
        };

        let start_str = "2099-01-01";
        let end_str = "2099-01-07";
        let key_cached = "sectors:foreign_flow:TESTCACHEDFF:2099-01-01:2099-01-07";

        let mock_ff = ForeignFlowResponse {
            symbol: "TESTCACHEDFF".to_string(),
            start: start_str.to_string(),
            end: end_str.to_string(),
            data: vec![crate::entities::sectors::ForeignFlowDailyPoint {
                date: "2099-01-05".to_string(),
                net_foreign_inflow: Some(150_000_000),
                foreign_buy_idr: Some(300_000_000),
                foreign_sell_idr: Some(150_000_000),
                foreign_share: Some(0.45),
            }],
        };
        let ff_json = serde_json::to_string(&mock_ff).unwrap();

        let _: Result<(), _> = redis::cmd("SET")
            .arg(key_cached)
            .arg(&ff_json)
            .query_async(&mut conn)
            .await;

        let stocks = vec![
            ScreenerCompany {
                symbol: "TESTCACHEDFF.JK".to_string(),
                company_name: "Test Cached FF Co".to_string(),
                query_values: None,
            },
            ScreenerCompany {
                symbol: "TESTUNCACHEDFF.JK".to_string(),
                company_name: "Test Uncached FF Co".to_string(),
                query_values: None,
            },
        ];

        let (cached_map, uncached) =
            get_cached_foreign_flow(&conn, &stocks, start_str, end_str).await;

        // Cleanup
        let _: Result<(), _> = redis::cmd("DEL")
            .arg(key_cached)
            .arg("sectors:foreign_flow:TESTCACHEDFF.JK:2099-01-01:2099-01-07")
            .query_async(&mut conn)
            .await;

        assert_eq!(cached_map.len(), 1);
        assert!(cached_map.contains_key("TESTCACHEDFF.JK"));
        assert_eq!(cached_map["TESTCACHEDFF.JK"].data.len(), 1);
        assert_eq!(
            cached_map["TESTCACHEDFF.JK"].data[0].net_foreign_inflow,
            Some(150_000_000)
        );

        assert_eq!(uncached.len(), 1);
        assert_eq!(uncached[0].symbol, "TESTUNCACHEDFF.JK");
    }
}
