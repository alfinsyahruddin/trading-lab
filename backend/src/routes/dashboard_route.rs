use actix_web::{
    delete, get, post,
    web::{Data, Path},
};
use uuid::Uuid;

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        base_response::JsonFromStringTrait,
        dashboard::{DashboardStatsResponse, LeaderboardEntry},
    },
    guards::authenticated_user::AuthenticatedUser,
    services::dashboard_service::DashboardService,
};

#[get("/api/dashboard/stats")]
pub async fn get_stats(
    service: Data<DashboardService>,
    user: AuthenticatedUser,
) -> AppResponse<DashboardStatsResponse> {
    service.get_stats(user.claims.sub).await.json()
}

#[get("/api/dashboard/leaderboard")]
pub async fn get_leaderboard(
    service: Data<DashboardService>,
    user: AuthenticatedUser,
) -> AppResponse<Vec<LeaderboardEntry>> {
    service
        .leaderboard_by_percentage(user.claims.sub)
        .await
        .json()
}

#[get("/api/dashboard/top-stars")]
pub async fn get_top_stars(
    service: Data<DashboardService>,
    user: AuthenticatedUser,
) -> AppResponse<Vec<LeaderboardEntry>> {
    service.leaderboard_by_stars(user.claims.sub).await.json()
}

#[post("/api/dashboard/stars/{backtest_id}")]
pub async fn star_backtest(
    service: Data<DashboardService>,
    user: AuthenticatedUser,
    backtest_id: Path<Uuid>,
) -> AppResponse<String> {
    service
        .star(user.claims.sub, backtest_id.into_inner())
        .await?;
    String::from("Backtest starred").json()
}

#[delete("/api/dashboard/stars/{backtest_id}")]
pub async fn unstar_backtest(
    service: Data<DashboardService>,
    user: AuthenticatedUser,
    backtest_id: Path<Uuid>,
) -> AppResponse<String> {
    service
        .unstar(user.claims.sub, backtest_id.into_inner())
        .await?;
    String::from("Backtest unstarred").json()
}
