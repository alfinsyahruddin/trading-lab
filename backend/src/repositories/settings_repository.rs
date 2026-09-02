use sqlx::PgPool;

use crate::entities::app_error::AppError;

#[derive(Clone)]
pub struct SettingsRepository {
    db: PgPool,
}

impl SettingsRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_ai_enabled(&self) -> Result<bool, AppError> {
        let row: Option<(serde_json::Value,)> =
            sqlx::query_as("SELECT value FROM app_settings WHERE key = 'ai_enabled'")
                .fetch_optional(&self.db)
                .await
                .map_err(AppError::from)?;

        let enabled = match row {
            Some((val,)) => val.as_bool().unwrap_or(false),
            None => false,
        };

        Ok(enabled)
    }

    pub async fn set_ai_enabled(&self, enabled: bool) -> Result<(), AppError> {
        let json_val = serde_json::Value::Bool(enabled);
        sqlx::query(
            r#"
            INSERT INTO app_settings (key, value, updated_at)
            VALUES ('ai_enabled', $1, CURRENT_TIMESTAMP)
            ON CONFLICT (key) DO UPDATE
            SET value = EXCLUDED.value, updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(json_val)
        .execute(&self.db)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
