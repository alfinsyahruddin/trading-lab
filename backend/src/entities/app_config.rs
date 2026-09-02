use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub bind_address: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub access_token_expiration_seconds: u64,
    pub refresh_token_expiration_seconds: u64,
    pub cors_allowed_origin: String,
    pub sectors_api_key: String,
    pub gemini_api_key: String,
    pub gemini_model: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            app_name: required("APP_NAME")?,
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".into()),
            port: required("PORT")?
                .parse()
                .map_err(|_| "PORT must be an integer")?,
            database_url: required("DATABASE_URL")?,
            redis_url: required("REDIS_URL")?,
            jwt_secret: required("JWT_SECRET")?,
            access_token_expiration_seconds: required("ACCESS_TOKEN_EXPIRATION_SECONDS")?
                .parse()
                .map_err(|_| {
                "ACCESS_TOKEN_EXPIRATION_SECONDS must be an integer"
            })?,
            refresh_token_expiration_seconds: required("REFRESH_TOKEN_EXPIRATION_SECONDS")?
                .parse()
                .map_err(|_| "REFRESH_TOKEN_EXPIRATION_SECONDS must be an integer")?,
            cors_allowed_origin: env::var("CORS_ALLOWED_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            sectors_api_key: required("SECTORS_API_KEY")?,
            gemini_api_key: env::var("GEMINI_API_KEY").unwrap_or_default(),
            gemini_model: env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-3.1-flash-lite".into()),
        })
    }
}

fn required(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("missing required environment variable: {name}"))
}
