use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDate;
use futures_util::future::join_all;
use redis::aio::ConnectionManager;
use uuid::Uuid;
use validator::Validate;

pub use crate::entities::backtest::{Position, StockData};
pub use crate::services::backtest::engine::simulate_backtest;
pub use crate::services::backtest::rules::{build_where_query, evaluate_manual_rules};

use crate::{
    clients::{llm_client::LLMTrait, sectors_client::SectorsClientTrait},
    entities::{
        app_error::AppError,
        backtest::{
            BacktestJobResponse, BacktestOwnerResponse, BacktestResultRecord,
            BacktestResultResponse, CreateBacktestJobRequest, MostTradedResponse, TopEntryResponse,
            TradeHistoryResponse, UpdateBacktestJobRequest,
        },
        sectors::ScreenerCompany,
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
            let (where_query, _) = build_where_query(&strategy.rules.0, year);
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
        let redis = self.redis.clone();

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                &repo,
                &strategy_repo,
                sectors.as_ref(),
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
        };

        tokio::spawn(async move {
            if let Err(e) = run_backtest(
                &repo,
                &strategy_repo,
                sectors.as_ref(),
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

#[allow(clippy::too_many_arguments)]
async fn run_backtest(
    repo: &BacktestRepository,
    strategy_repo: &TradingStrategyRepository,
    sectors: &dyn SectorsClientTrait,
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
        let symbol = &stock.symbol;
        let start = &start_str;
        let end = &end_str;
        fetch_futures.push(async move {
            let data = sectors.daily_transactions(symbol, start, end).await;
            (symbol.clone(), data)
        });
    }

    let results = join_all(fetch_futures).await;

    let mut stock_data_list = Vec::new();
    for (symbol, data_result) in results {
        if let Ok(data) = data_result {
            let mut valid_data = Vec::new();
            for d in data {
                let Some(date) = NaiveDate::parse_from_str(&d.date, "%Y-%m-%d").ok() else {
                    continue;
                };
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
    let prompt = prompt_helper::get_backtest_ai_summary_prompt(
        job_name,
        strategy_name,
        year,
        duration_months,
        initial_cash,
        result,
    );

    let summary: Vec<String> =
        crate::clients::llm_client::generate_structured(llm, &prompt).await?;
    let truncated: Vec<String> = summary.into_iter().take(5).collect();

    Ok(truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

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
