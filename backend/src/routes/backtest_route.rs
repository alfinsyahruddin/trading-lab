use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        backtest::{BacktestJobResponse, CreateBacktestJobRequest, UpdateBacktestJobRequest},
        base_response::JsonFromStringTrait,
    },
    guards::authenticated_user::AuthenticatedUser,
    services::backtest_service::BacktestService,
};

#[get("/api/backtests")]
pub async fn list_backtests(
    service: Data<BacktestService>,
    user: AuthenticatedUser,
) -> AppResponse<Vec<BacktestJobResponse>> {
    service.list(user.claims.sub).await.json()
}

#[get("/api/backtests/{id}")]
pub async fn get_backtest(
    service: Data<BacktestService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
) -> AppResponse<BacktestJobResponse> {
    service.get(id.into_inner(), user.claims.sub).await.json()
}

#[post("/api/backtests")]
pub async fn create_backtest(
    service: Data<BacktestService>,
    user: AuthenticatedUser,
    body: Json<CreateBacktestJobRequest>,
) -> AppResponse<BacktestJobResponse> {
    service
        .create(user.claims.sub, body.into_inner())
        .await
        .json()
}

#[patch("/api/backtests/{id}")]
pub async fn update_backtest(
    service: Data<BacktestService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
    body: Json<UpdateBacktestJobRequest>,
) -> AppResponse<BacktestJobResponse> {
    service
        .update(id.into_inner(), user.claims.sub, body.into_inner())
        .await
        .json()
}

#[delete("/api/backtests/{id}")]
pub async fn delete_backtest(
    service: Data<BacktestService>,
    user: AuthenticatedUser,
    id: Path<Uuid>,
) -> AppResponse<String> {
    service.delete(id.into_inner(), user.claims.sub).await?;
    String::from("Backtest deleted successfully").json()
}
