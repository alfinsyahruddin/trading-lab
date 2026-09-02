use actix_web::{
    get, patch,
    web::{Data, Json},
};

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        app_settings::{AppSettingsResponse, UpdateAppSettingsRequest},
    },
    guards::{authenticated_user::AuthenticatedUser, require_admin::RequireAdmin},
    services::settings_service::SettingsService,
};

#[get("/api/settings")]
pub async fn get_settings(
    service: Data<SettingsService>,
    _user: AuthenticatedUser,
) -> AppResponse<AppSettingsResponse> {
    service.get_settings().await.json()
}

#[patch("/api/settings")]
pub async fn update_settings(
    service: Data<SettingsService>,
    _user: AuthenticatedUser,
    _admin: RequireAdmin,
    body: Json<UpdateAppSettingsRequest>,
) -> AppResponse<AppSettingsResponse> {
    service.update_settings(body.into_inner()).await.json()
}
