use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::enums::{token_type::TokenType, user_role::UserRole};

#[derive(Clone, Debug, FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserRecord> for UserResponse {
    fn from(user: UserRecord) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email, length(max = 320))]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    pub captcha_id: Uuid,
    #[validate(length(min = 1, max = 32))]
    pub captcha_code: String,
}

#[derive(Debug, Serialize)]
pub struct CaptchaResponse {
    pub id: Uuid,
    pub image: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email, length(max = 320))]
    pub email: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdminCreateUserRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email, length(max = 320))]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdminUpdateUserRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(email, length(max = 320))]
    pub email: Option<String>,
    #[validate(length(min = 8, max = 128))]
    pub password: Option<String>,
    pub role: Option<UserRole>,
}

impl AdminUpdateUserRequest {
    pub const fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.email.is_none()
            && self.password.is_none()
            && self.role.is_none()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email, length(max = 320))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, max = 128))]
    pub current_password: String,
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub tokens: TokenResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub token_type: TokenType,
    pub sid: Uuid,
    pub iat: i64,
    pub exp: i64,
}
