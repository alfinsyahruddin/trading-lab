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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use validator::Validate;

    fn sample_register_request() -> RegisterRequest {
        RegisterRequest {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            captcha_id: Uuid::new_v4(),
            captcha_code: "ABCD".to_string(),
        }
    }

    #[test]
    fn register_request_valid_should_pass() {
        let req = sample_register_request();
        assert!(req.validate().is_ok());
    }

    #[test]
    fn register_request_invalid_email_should_fail() {
        let invalid_emails = [
            "not-an-email",
            "plainaddress",
            "@missingusername.com",
            "username@.com",
            "username@domain..com",
        ];

        for email in invalid_emails {
            let mut req = sample_register_request();
            req.email = email.to_string();
            assert!(
                req.validate().is_err(),
                "Expected validation error for email: {email}"
            );
        }
    }

    #[test]
    fn register_request_password_length_boundaries() {
        let mut req = sample_register_request();

        // 7 characters: too short (< 8)
        req.password = "1234567".to_string();
        assert!(req.validate().is_err());

        // 8 characters: minimum valid
        req.password = "12345678".to_string();
        assert!(req.validate().is_ok());

        // 128 characters: maximum valid
        req.password = "a".repeat(128);
        assert!(req.validate().is_ok());

        // 129 characters: too long (> 128)
        req.password = "a".repeat(129);
        assert!(req.validate().is_err());
    }

    #[test]
    fn register_request_name_length_boundaries_and_blank() {
        let mut req = sample_register_request();

        // Blank name (0 chars)
        req.name = String::new();
        assert!(req.validate().is_err());

        // 1 character: valid min
        req.name = "A".to_string();
        assert!(req.validate().is_ok());

        // 255 characters: valid max
        req.name = "A".repeat(255);
        assert!(req.validate().is_ok());

        // 256 characters: too long
        req.name = "A".repeat(256);
        assert!(req.validate().is_err());
    }

    #[test]
    fn register_request_captcha_code_boundaries() {
        let mut req = sample_register_request();

        // Blank captcha code
        req.captcha_code = String::new();
        assert!(req.validate().is_err());

        // 32 chars: valid max
        req.captcha_code = "X".repeat(32);
        assert!(req.validate().is_ok());

        // 33 chars: too long
        req.captcha_code = "X".repeat(33);
        assert!(req.validate().is_err());
    }

    #[test]
    fn login_request_validation() {
        let valid = LoginRequest {
            email: "trader@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = LoginRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        // Empty password
        let empty_password = LoginRequest {
            email: "trader@example.com".to_string(),
            password: String::new(),
        };
        assert!(empty_password.validate().is_err());

        // Password boundary (128 ok, 129 too long)
        let max_password = LoginRequest {
            email: "trader@example.com".to_string(),
            password: "x".repeat(128),
        };
        assert!(max_password.validate().is_ok());

        let too_long_password = LoginRequest {
            email: "trader@example.com".to_string(),
            password: "x".repeat(129),
        };
        assert!(too_long_password.validate().is_err());
    }

    #[test]
    fn update_profile_request_validation() {
        let valid = UpdateProfileRequest {
            name: "Updated Name".to_string(),
            email: "updated@example.com".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Blank name
        let blank_name = UpdateProfileRequest {
            name: String::new(),
            email: "updated@example.com".to_string(),
        };
        assert!(blank_name.validate().is_err());

        // Name too long (> 255)
        let long_name = UpdateProfileRequest {
            name: "a".repeat(256),
            email: "updated@example.com".to_string(),
        };
        assert!(long_name.validate().is_err());

        // Invalid email
        let invalid_email = UpdateProfileRequest {
            name: "Valid Name".to_string(),
            email: "not-an-email".to_string(),
        };
        assert!(invalid_email.validate().is_err());
    }

    #[test]
    fn change_password_request_validation() {
        let valid = ChangePasswordRequest {
            current_password: "oldPassword123".to_string(),
            new_password: "newPassword456".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty current password
        let empty_current = ChangePasswordRequest {
            current_password: String::new(),
            new_password: "newPassword456".to_string(),
        };
        assert!(empty_current.validate().is_err());

        // Short new password (< 8)
        let short_new = ChangePasswordRequest {
            current_password: "oldPassword123".to_string(),
            new_password: "short".to_string(),
        };
        assert!(short_new.validate().is_err());

        // New password max length boundary (128 ok, 129 too long)
        let max_new = ChangePasswordRequest {
            current_password: "oldPassword123".to_string(),
            new_password: "x".repeat(128),
        };
        assert!(max_new.validate().is_ok());

        let too_long_new = ChangePasswordRequest {
            current_password: "oldPassword123".to_string(),
            new_password: "x".repeat(129),
        };
        assert!(too_long_new.validate().is_err());
    }

    #[test]
    fn admin_create_user_request_validation() {
        let valid = AdminCreateUserRequest {
            name: "Admin User".to_string(),
            email: "admin@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            role: UserRole::Admin,
        };
        assert!(valid.validate().is_ok());

        let invalid = AdminCreateUserRequest {
            name: String::new(),
            email: "bad-email".to_string(),
            password: "short".to_string(),
            role: UserRole::Member,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn admin_update_user_request_validation_and_is_empty() {
        let empty = AdminUpdateUserRequest {
            name: None,
            email: None,
            password: None,
            role: None,
        };
        assert!(empty.is_empty());
        assert!(empty.validate().is_ok());

        let partial = AdminUpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
            password: None,
            role: None,
        };
        assert!(!partial.is_empty());
        assert!(partial.validate().is_ok());

        let invalid = AdminUpdateUserRequest {
            name: Some(String::new()),
            email: Some("invalid-email".to_string()),
            password: Some("short".to_string()),
            role: None,
        };
        assert!(!invalid.is_empty());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn user_record_to_response_conversion() {
        let now = Utc::now();
        let user_id = Uuid::new_v4();
        let record = UserRecord {
            id: user_id,
            name: "Jane Trader".to_string(),
            email: "jane@tradinglab.id".to_string(),
            password_hash: "argon2id$hashedpassword".to_string(),
            role: UserRole::Member,
            created_at: now,
            updated_at: now,
        };

        let resp = UserResponse::from(record);
        assert_eq!(resp.id, user_id);
        assert_eq!(resp.name, "Jane Trader");
        assert_eq!(resp.email, "jane@tradinglab.id");
        assert_eq!(resp.role, UserRole::Member);
        assert_eq!(resp.created_at, now);
        assert_eq!(resp.updated_at, now);
    }
}
