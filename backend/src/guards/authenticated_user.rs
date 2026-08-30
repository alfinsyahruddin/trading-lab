use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, http::header::AUTHORIZATION, web::Data, FromRequest, HttpRequest};

use crate::{
    entities::{app_config::AppConfig, app_error::AppError, user::Claims},
    enums::token_type::TokenType,
    helpers::token_helper::decode_token,
    services::session_service::SessionService,
};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = request.clone();
        Box::pin(async move { authenticate_request(&request).await })
    }
}

pub async fn authenticate_request(request: &HttpRequest) -> Result<AuthenticatedUser, AppError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("Authorization token is required"))?;
    let config = request
        .app_data::<Data<AppConfig>>()
        .ok_or(AppError::Internal)?;
    let sessions = request
        .app_data::<Data<SessionService>>()
        .ok_or(AppError::Internal)?;
    let claims = decode_token(token, config.get_ref())?;
    if claims.token_type != TokenType::Access {
        return Err(AppError::unauthorized("Access token required"));
    }
    if !sessions.validate_session(claims.sid, claims.sub).await? {
        return Err(AppError::unauthorized("Session is no longer active"));
    }
    Ok(AuthenticatedUser { claims })
}
