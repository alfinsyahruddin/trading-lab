pub mod backtest_route;
pub mod dashboard_route;
pub mod settings_route;
pub mod trading_strategy_route;
pub mod user_route;

use actix_web::{get, web, HttpResponse};

use crate::entities::{
    app_response::AppResponse,
    base_response::{BaseResponse, JsonFromStringTrait},
};

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(index)
        .service(health)
        .service(user_route::get_captcha)
        .service(user_route::register)
        .service(user_route::login)
        .service(user_route::refresh)
        .service(user_route::logout)
        .service(user_route::get_current_user)
        .service(user_route::update_current_user_profile)
        .service(user_route::change_current_user_password)
        .service(user_route::create_user)
        .service(user_route::list_users)
        .service(user_route::get_user)
        .service(user_route::update_user)
        .service(user_route::delete_user)
        .service(trading_strategy_route::list_strategies)
        .service(trading_strategy_route::get_strategy)
        .service(trading_strategy_route::create_strategy)
        .service(trading_strategy_route::update_strategy)
        .service(trading_strategy_route::delete_strategy)
        .service(trading_strategy_route::duplicate_strategy)
        .service(trading_strategy_route::get_ai_suggestions)
        .service(backtest_route::list_backtests)
        .service(backtest_route::get_backtest)
        .service(backtest_route::create_backtest)
        .service(backtest_route::rerun_backtest)
        .service(backtest_route::update_backtest)
        .service(backtest_route::delete_backtest)
        .service(dashboard_route::get_stats)
        .service(dashboard_route::get_leaderboard)
        .service(dashboard_route::get_top_stars)
        .service(dashboard_route::star_backtest)
        .service(dashboard_route::unstar_backtest)
        .service(settings_route::get_settings)
        .service(settings_route::update_settings);
}

#[get("/")]
async fn index() -> AppResponse<String> {
    String::from("Trading Lab API").json_data()
}

#[get("/health")]
async fn health() -> AppResponse<String> {
    String::from("ok").json_data()
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(BaseResponse::<()>::error(404, "Not found"))
}
