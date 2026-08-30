use std::sync::Arc;

use actix_web::web::{self, Data};
use reqwest::Client;

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    repositories::{
        trading_strategy_repository::TradingStrategyRepository, user_repository::UserRepository,
    },
    services::{
        auth_service::AuthService, session_service::SessionService,
        trading_strategy_service::TradingStrategyService, user_service::UserService,
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
    http_client: Data<Client>,
}

impl AppDependencies {
    pub async fn build(config: AppConfig) -> Result<Self, AppError> {
        let database = setup_db(&config).await?;
        let redis = setup_redis(&config).await?;
        let http_client = setup_http_client()?;
        let users = Arc::new(UserRepository::new(database.clone()));
        let strategies = Arc::new(TradingStrategyRepository::new(database));
        let sessions = Arc::new(SessionService::new(
            redis,
            config.refresh_token_expiration_seconds,
        ));
        let auth_service =
            AuthService::new(Arc::clone(&users), Arc::clone(&sessions), config.clone());
        let user_service = UserService::new(users, Arc::clone(&sessions));
        let trading_strategy_service = TradingStrategyService::new(strategies);

        Ok(Self {
            config: Data::new(config),
            auth_service: Data::new(auth_service),
            user_service: Data::new(user_service),
            session_service: Data::from(sessions),
            trading_strategy_service: Data::new(trading_strategy_service),
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
            .app_data(self.http_client.clone());
    }
}
