use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    entities::{
        app_config::AppConfig,
        app_error::AppError,
        user::{Claims, TokenResponse, UserRecord},
    },
    enums::token_type::TokenType,
};

pub fn create_token_pair(
    user: &UserRecord,
    session_id: Uuid,
    config: &AppConfig,
) -> Result<TokenResponse, AppError> {
    Ok(TokenResponse {
        access_token: create_token(user, session_id, TokenType::Access, config)?,
        refresh_token: create_token(user, session_id, TokenType::Refresh, config)?,
    })
}

pub fn decode_token(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
    let bearer_token = token.trim().strip_prefix("Bearer ").unwrap_or(token.trim());
    decode::<Claims>(
        bearer_token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|token| token.claims)
    .map_err(|_| AppError::unauthorized("Invalid or expired token"))
}

fn create_token(
    user: &UserRecord,
    session_id: Uuid,
    token_type: TokenType,
    config: &AppConfig,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let expiration = match token_type {
        TokenType::Access => config.access_token_expiration_seconds,
        TokenType::Refresh => config.refresh_token_expiration_seconds,
    };
    let expiration = i64::try_from(expiration).map_err(|_| AppError::Internal)?;
    let claims = Claims {
        sub: user.id,
        role: user.role,
        token_type,
        sid: session_id,
        iat: now,
        exp: now.checked_add(expiration).ok_or(AppError::Internal)?,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(AppError::from)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::{create_token_pair, decode_token};
    use crate::{
        entities::{app_config::AppConfig, user::UserRecord},
        enums::{token_type::TokenType, user_role::UserRole},
    };

    #[test]
    fn create_token_pair_should_encode_access_claims() {
        let now = Utc::now();
        let user = UserRecord {
            id: Uuid::new_v4(),
            name: "Test User".into(),
            email: "test@example.com".into(),
            password_hash: "not-used".into(),
            role: UserRole::Member,
            created_at: now,
            updated_at: now,
        };
        let config = AppConfig {
            app_name: "test".into(),
            bind_address: "127.0.0.1".into(),
            port: 8000,
            database_url: "postgres://unused".into(),
            redis_url: "redis://unused".into(),
            jwt_secret: "test-secret".into(),
            access_token_expiration_seconds: 60,
            refresh_token_expiration_seconds: 60,
            cors_allowed_origin: "http://localhost:3000".into(),
            sectors_api_key: "test_key".into(),
        };

        let tokens = create_token_pair(&user, Uuid::new_v4(), &config).expect("tokens encode");
        let claims = decode_token(&tokens.access_token, &config).expect("access token decodes");
        assert_eq!(claims.token_type, TokenType::Access);
    }
}
