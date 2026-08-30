use std::sync::Arc;

use validator::Validate;

use crate::{
    entities::{
        app_config::AppConfig,
        app_error::AppError,
        user::{LoginRequest, LoginResponse, RefreshTokenRequest},
    },
    enums::token_type::TokenType,
    helpers::{hash_helper::verify_password, token_helper},
    repositories::user_repository::UserRepository,
    services::session_service::SessionService,
};

#[derive(Clone)]
pub struct AuthService {
    users: Arc<UserRepository>,
    sessions: Arc<SessionService>,
    config: AppConfig,
}

impl AuthService {
    pub fn new(
        users: Arc<UserRepository>,
        sessions: Arc<SessionService>,
        config: AppConfig,
    ) -> Self {
        Self {
            users,
            sessions,
            config,
        }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        request
            .validate()
            .map_err(|_| AppError::bad_request("Invalid email or password"))?;
        let email = normalize_email(&request.email);
        let user = self
            .users
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::unauthorized("Invalid email or password"))?;

        if !verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::unauthorized("Invalid email or password"));
        }

        let session_id = self.sessions.create_session(user.id).await?;
        let tokens = token_helper::create_token_pair(&user, session_id, &self.config)?;
        Ok(LoginResponse {
            user: user.into(),
            tokens,
        })
    }

    pub async fn refresh(&self, request: RefreshTokenRequest) -> Result<LoginResponse, AppError> {
        let claims = token_helper::decode_token(&request.refresh_token, &self.config)?;
        if claims.token_type != TokenType::Refresh {
            return Err(AppError::unauthorized("Refresh token required"));
        }
        if !self
            .sessions
            .validate_session(claims.sid, claims.sub)
            .await?
        {
            return Err(AppError::unauthorized("Session is no longer active"));
        }

        let user = self.users.find_by_id(claims.sub).await?;
        self.sessions.revoke_session(claims.sid).await?;
        let session_id = self.sessions.create_session(user.id).await?;
        let tokens = token_helper::create_token_pair(&user, session_id, &self.config)?;
        Ok(LoginResponse {
            user: user.into(),
            tokens,
        })
    }

    pub async fn logout(&self, session_id: uuid::Uuid) -> Result<(), AppError> {
        self.sessions.revoke_session(session_id).await
    }
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}
