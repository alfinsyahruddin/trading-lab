use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use uuid::Uuid;

use crate::entities::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|_| AppError::Internal)?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::Internal)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AppError::Internal)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn hash_password_should_verify_its_source_password() {
        let hash = hash_password("correct-password").expect("password hashes");
        assert!(verify_password("correct-password", &hash).expect("hash is valid"));
    }

    #[test]
    fn verify_password_should_reject_another_password() {
        let hash = hash_password("correct-password").expect("password hashes");
        assert!(!verify_password("incorrect-password", &hash).expect("hash is valid"));
    }
}
