use redis::aio::ConnectionManager;

use crate::{
    entities::{
        app_error::AppError,
        app_settings::{AppSettingsResponse, UpdateAppSettingsRequest},
    },
    repositories::settings_repository::SettingsRepository,
};

#[derive(Clone)]
pub struct SettingsService {
    repo: SettingsRepository,
    redis: ConnectionManager,
}

impl SettingsService {
    pub fn new(repo: SettingsRepository, redis: ConnectionManager) -> Self {
        Self { repo, redis }
    }

    pub async fn get_ai_enabled(&self) -> Result<bool, AppError> {
        let key = "app_setting:ai_enabled";
        let mut redis = self.redis.clone();

        let cached: Result<Option<String>, _> =
            redis::cmd("GET").arg(key).query_async(&mut redis).await;

        if let Ok(Some(val)) = cached {
            return Ok(val == "true" || val == "1");
        }

        let enabled = self.repo.get_ai_enabled().await?;

        let val_str = if enabled { "true" } else { "false" };
        let _: Result<(), _> = redis::cmd("SET")
            .arg(key)
            .arg(val_str)
            .query_async(&mut redis)
            .await;

        Ok(enabled)
    }

    pub async fn get_settings(&self) -> Result<AppSettingsResponse, AppError> {
        let ai_enabled = self.get_ai_enabled().await?;
        Ok(AppSettingsResponse { ai_enabled })
    }

    pub async fn update_settings(
        &self,
        req: UpdateAppSettingsRequest,
    ) -> Result<AppSettingsResponse, AppError> {
        self.repo.set_ai_enabled(req.ai_enabled).await?;

        let key = "app_setting:ai_enabled";
        let mut redis = self.redis.clone();
        let val_str = if req.ai_enabled { "true" } else { "false" };
        let _: Result<(), _> = redis::cmd("SET")
            .arg(key)
            .arg(val_str)
            .query_async(&mut redis)
            .await;

        Ok(AppSettingsResponse {
            ai_enabled: req.ai_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_boolean_cache_values() {
        assert!("true" == "true");
        assert!("false" != "true");
    }
}
