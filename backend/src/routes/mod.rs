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
        .service(user_route::register)
        .service(user_route::login)
        .service(user_route::refresh)
        .service(user_route::logout)
        .service(user_route::create_user)
        .service(user_route::list_users)
        .service(user_route::get_user)
        .service(user_route::update_user)
        .service(user_route::delete_user);
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
