use actix_web::{
    error::InternalError, http::header, middleware::DefaultHeaders, web, ResponseError,
};

use crate::entities::app_error::AppError;

pub fn cors_headers(allowed_origin: &str) -> DefaultHeaders {
    DefaultHeaders::new()
        .add((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            allowed_origin.to_owned(),
        ))
        .add((
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "POST, GET, PATCH, DELETE, OPTIONS",
        ))
        .add((
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            "Authorization, Content-Type",
        ))
        .add((header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
}

pub fn json_config() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|_, _| {
        InternalError::from_response(
            "invalid JSON body",
            AppError::bad_request("Invalid JSON request body").error_response(),
        )
        .into()
    })
}
