use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        trading_strategy::{
            CreateTradingStrategyRequest, DuplicateTradingStrategyRequest, TradingStrategyResponse,
            UpdateTradingStrategyRequest,
        },
    },
    guards::authenticated_user::AuthenticatedUser,
    services::trading_strategy_service::TradingStrategyService,
};

#[get("/api/strategies")]
pub async fn list_strategies(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
) -> AppResponse<Vec<TradingStrategyResponse>> {
    strategy_service.list(user.claims.sub).await.json()
}

#[get("/api/strategies/{id}")]
pub async fn get_strategy(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
) -> AppResponse<TradingStrategyResponse> {
    strategy_service
        .get(id.into_inner(), user.claims.sub)
        .await
        .json()
}

#[post("/api/strategies")]
pub async fn create_strategy(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
    body: Json<CreateTradingStrategyRequest>,
) -> AppResponse<TradingStrategyResponse> {
    strategy_service
        .create(user.claims.sub, body.into_inner())
        .await
        .json()
}

#[patch("/api/strategies/{id}")]
pub async fn update_strategy(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
    body: Json<UpdateTradingStrategyRequest>,
) -> AppResponse<TradingStrategyResponse> {
    strategy_service
        .update(id.into_inner(), user.claims.sub, body.into_inner())
        .await
        .json()
}

#[delete("/api/strategies/{id}")]
pub async fn delete_strategy(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
) -> AppResponse<String> {
    strategy_service
        .delete(id.into_inner(), user.claims.sub)
        .await
        .map(|_| String::from("Trading strategy deleted successfully"))
        .json()
}

#[post("/api/strategies/{id}/duplicate")]
pub async fn duplicate_strategy(
    strategy_service: Data<TradingStrategyService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
    body: Json<DuplicateTradingStrategyRequest>,
) -> AppResponse<TradingStrategyResponse> {
    strategy_service
        .duplicate(id.into_inner(), user.claims.sub, body.into_inner())
        .await
        .json()
}
