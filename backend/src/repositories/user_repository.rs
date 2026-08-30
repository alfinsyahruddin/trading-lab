use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::{
    entities::{app_error::AppError, user::UserRecord},
    enums::user_role::UserRole,
};

#[derive(Clone)]
pub struct UserRepository {
    db: PgPool,
}

impl UserRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        name: &str,
        email: &str,
        password_hash: &str,
        role: UserRole,
    ) -> Result<UserRecord, AppError> {
        sqlx::query_as::<_, UserRecord>(
            "INSERT INTO users (name, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(role)
        .fetch_one(&self.db)
        .await
        .map_err(map_database_error)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>, AppError> {
        sqlx::query_as::<_, UserRecord>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<UserRecord, AppError> {
        sqlx::query_as::<_, UserRecord>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found"))
    }

    pub async fn list(&self) -> Result<Vec<UserRecord>, AppError> {
        sqlx::query_as::<_, UserRecord>(
            "SELECT id, name, email, password_hash, role, created_at, updated_at FROM users ORDER BY created_at ASC",
        )
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        email: Option<&str>,
        password_hash: Option<&str>,
        role: Option<UserRole>,
    ) -> Result<UserRecord, AppError> {
        sqlx::query_as::<_, UserRecord>(
            "UPDATE users SET name = COALESCE($2, name), email = COALESCE($3, email), password_hash = COALESCE($4, password_hash), role = COALESCE($5, role), updated_at = CURRENT_TIMESTAMP WHERE id = $1 RETURNING id, name, email, password_hash, role, created_at, updated_at",
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(role)
        .fetch_optional(&self.db)
        .await
        .map_err(map_database_error)?
        .ok_or_else(|| AppError::not_found("User not found"))
    }

    pub async fn delete(&self, id: Uuid) -> Result<PgQueryResult, AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_admins(&self) -> Result<i64, AppError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role = 'ADMIN'")
            .fetch_one(&self.db)
            .await
            .map_err(AppError::from)
    }
}

fn map_database_error(error: sqlx::Error) -> AppError {
    match &error {
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("23505") =>
        {
            AppError::conflict("Email is already registered")
        }
        _ => AppError::from(error),
    }
}
