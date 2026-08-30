use reqwest::Client;

use crate::entities::app_error::AppError;

pub fn setup_http_client() -> Result<Client, AppError> {
    Client::builder().build().map_err(|error| {
        eprintln!("HTTP client setup error: {error}");
        AppError::Internal
    })
}
