use std::sync::Arc;

use actix_web::web::{self, Data};
use reqwest::Client;

use crate::{
    clients::{
        llm_client::{GeminiLLM, LLMTrait},
        sectors_client::SectorsClient,
    },
    entities::{app_config::AppConfig, app_error::AppError},
    repositories::{
        backtest_repository::BacktestRepository, dashboard_repository::DashboardRepository,
        settings_repository::SettingsRepository,
        trading_strategy_repository::TradingStrategyRepository, user_repository::UserRepository,
    },
    services::{
        auth_service::AuthService, backtest_service::BacktestService,
        dashboard_service::DashboardService, session_service::SessionService,
        settings_service::SettingsService, trading_strategy_service::TradingStrategyService,
        user_service::UserService,
    },
    setup::{setup_db::setup_db, setup_http_client::setup_http_client, setup_redis::setup_redis},
};

#[derive(Clone)]
pub struct AppDependencies {
    config: Data<AppConfig>,
    auth_service: Data<AuthService>,
    user_service: Data<UserService>,
    session_service: Data<SessionService>,
    trading_strategy_service: Data<TradingStrategyService>,
    backtest_service: Data<BacktestService>,
    dashboard_service: Data<DashboardService>,
    settings_service: Data<SettingsService>,
    http_client: Data<Client>,
}

impl AppDependencies {
    pub async fn build(config: AppConfig) -> Result<Self, AppError> {
        let database = setup_db(&config).await?;
        let redis = setup_redis(&config).await?;
        let http_client = setup_http_client()?;

        let sectors_client = Arc::new(SectorsClient::new(
            http_client.clone(),
            config.sectors_api_key.clone(),
            redis.clone(),
        ));

        let llm: Arc<dyn LLMTrait> = Arc::new(GeminiLLM::new(
            http_client.clone(),
            config.gemini_api_key.clone(),
            config.gemini_model.clone(),
        ));

        let users = Arc::new(UserRepository::new(database.clone()));
        let strategies = Arc::new(TradingStrategyRepository::new(database.clone()));
        let backtests = Arc::new(BacktestRepository::new(database.clone()));
        let dashboard = Arc::new(DashboardRepository::new(database.clone()));
        let settings_repo = Arc::new(SettingsRepository::new(database.clone()));

        let sessions = Arc::new(SessionService::new(
            redis.clone(),
            config.refresh_token_expiration_seconds,
        ));

        let settings_service = Arc::new(SettingsService::new(settings_repo, redis.clone()));

        let auth_service =
            AuthService::new(Arc::clone(&users), Arc::clone(&sessions), config.clone());
        let user_service = UserService::new(users.clone(), Arc::clone(&sessions), redis.clone());
        let trading_strategy_service = TradingStrategyService::new(
            Arc::clone(&strategies),
            Arc::clone(&llm),
            Arc::clone(&settings_service),
        );
        let backtest_service = BacktestService::new(
            backtests,
            strategies,
            sectors_client,
            Arc::clone(&llm),
            Arc::clone(&settings_service),
            redis.clone(),
        );
        let dashboard_service = DashboardService::new(dashboard, Arc::clone(&users));

        Ok(Self {
            config: Data::new(config),
            auth_service: Data::new(auth_service),
            user_service: Data::new(user_service),
            session_service: Data::from(sessions),
            trading_strategy_service: Data::new(trading_strategy_service),
            backtest_service: Data::new(backtest_service),
            dashboard_service: Data::new(dashboard_service),
            settings_service: Data::from(settings_service),
            http_client: Data::new(http_client),
        })
    }

    pub fn configure(&self, config: &mut web::ServiceConfig) {
        config
            .app_data(self.config.clone())
            .app_data(self.auth_service.clone())
            .app_data(self.user_service.clone())
            .app_data(self.session_service.clone())
            .app_data(self.trading_strategy_service.clone())
            .app_data(self.backtest_service.clone())
            .app_data(self.dashboard_service.clone())
            .app_data(self.settings_service.clone())
            .app_data(self.http_client.clone());
    }
}
