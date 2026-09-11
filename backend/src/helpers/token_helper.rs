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

    fn dummy_config(secret: &str) -> AppConfig {
        AppConfig {
            app_name: "test".into(),
            bind_address: "127.0.0.1".into(),
            port: 8000,
            database_url: "postgres://unused".into(),
            redis_url: "redis://unused".into(),
            jwt_secret: secret.into(),
            access_token_expiration_seconds: 3600,
            refresh_token_expiration_seconds: 86400,
            cors_allowed_origin: "http://localhost:3000".into(),
            sectors_api_key: "test_key".into(),
            gemini_api_key: "test_gemini".into(),
            gemini_model: "gemini-3.1-flash-lite".into(),
        }
    }

    #[test]
    fn create_token_pair_should_encode_and_extract_claims_correctly() {
        let now = Utc::now();
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let user = UserRecord {
            id: user_id,
            name: "Admin User".into(),
            email: "admin@example.com".into(),
            password_hash: "not-used".into(),
            role: UserRole::Admin,
            created_at: now,
            updated_at: now,
        };
        let config = dummy_config("test-secret-key-12345");

        let tokens = create_token_pair(&user, session_id, &config).expect("tokens encode");

        // Test access token with Bearer prefix
        let bearer_raw = format!("  Bearer {}  ", tokens.access_token);
        let access_claims = decode_token(&bearer_raw, &config).expect("access token decodes");
        assert_eq!(access_claims.sub, user_id);
        assert_eq!(access_claims.sid, session_id);
        assert_eq!(access_claims.role, UserRole::Admin);
        assert_eq!(access_claims.token_type, TokenType::Access);
        assert!(access_claims.exp > access_claims.iat);

        // Test refresh token without Bearer prefix
        let refresh_claims =
            decode_token(&tokens.refresh_token, &config).expect("refresh token decodes");
        assert_eq!(refresh_claims.sub, user_id);
        assert_eq!(refresh_claims.sid, session_id);
        assert_eq!(refresh_claims.role, UserRole::Admin);
        assert_eq!(refresh_claims.token_type, TokenType::Refresh);
        assert!(refresh_claims.exp > access_claims.exp);
    }

    #[test]
    fn decode_token_should_reject_invalid_signature() {
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

        let signer_config = dummy_config("signer-secret-key-1");
        let verifier_config = dummy_config("different-secret-key-2");

        let tokens = create_token_pair(&user, Uuid::new_v4(), &signer_config).expect("tokens");
        let result = decode_token(&tokens.access_token, &verifier_config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::entities::app_error::AppError::Unauthorized(_)
        ));
    }

    #[test]
    fn decode_token_should_reject_malformed_tokens() {
        let config = dummy_config("test-secret");

        let malformed_samples = [
            "",
            "   ",
            "not-a-jwt",
            "header.payload",
            "header.payload.signature.extra",
            "Bearer ",
            "invalid.base64.token",
        ];

        for sample in malformed_samples {
            let result = decode_token(sample, &config);
            assert!(
                result.is_err(),
                "Expected failure for malformed token: '{sample}'"
            );
            assert!(matches!(
                result.unwrap_err(),
                crate::entities::app_error::AppError::Unauthorized(_)
            ));
        }
    }

    #[test]
    fn decode_token_should_reject_expired_tokens() {
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

        let config = dummy_config("test-secret");
        let past_timestamp = Utc::now().timestamp() - 3600; // 1 hour ago
        let expired_claims = crate::entities::user::Claims {
            sub: Uuid::new_v4(),
            role: UserRole::Member,
            token_type: TokenType::Access,
            sid: Uuid::new_v4(),
            iat: past_timestamp - 300,
            exp: past_timestamp,
        };

        let expired_jwt = encode(
            &Header::new(Algorithm::HS256),
            &expired_claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .expect("encoded expired token");

        let result = decode_token(&expired_jwt, &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::entities::app_error::AppError::Unauthorized(_)
        ));
    }
}
