use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        user::{
            AdminCreateUserRequest, AdminUpdateUserRequest, LoginRequest, LoginResponse,
            RefreshTokenRequest, UserResponse,
        },
    },
    guards::{authenticated_user::AuthenticatedUser, require_admin::RequireAdmin},
    services::{auth_service::AuthService, user_service::UserService},
};

#[post("/api/users/register")]
pub async fn register(
    user_service: Data<UserService>,
    body: Json<crate::entities::user::RegisterRequest>,
) -> AppResponse<UserResponse> {
    user_service.register(body.into_inner()).await.json()
}

#[post("/api/users/login")]
pub async fn login(
    auth_service: Data<AuthService>,
    body: Json<LoginRequest>,
) -> AppResponse<LoginResponse> {
    auth_service.login(body.into_inner()).await.json()
}

#[post("/api/users/refresh")]
pub async fn refresh(
    auth_service: Data<AuthService>,
    body: Json<RefreshTokenRequest>,
) -> AppResponse<LoginResponse> {
    auth_service.refresh(body.into_inner()).await.json()
}

#[post("/api/users/logout")]
pub async fn logout(
    auth_service: Data<AuthService>,
    user: AuthenticatedUser,
) -> AppResponse<String> {
    auth_service
        .logout(user.claims.sid)
        .await
        .map(|_| String::from("Logout successful"))
        .json()
}

#[post("/api/users")]
pub async fn create_user(
    user_service: Data<UserService>,
    _admin: RequireAdmin,
    body: Json<AdminCreateUserRequest>,
) -> AppResponse<UserResponse> {
    user_service.create_by_admin(body.into_inner()).await.json()
}

#[get("/api/users")]
pub async fn list_users(
    user_service: Data<UserService>,
    _admin: RequireAdmin,
) -> AppResponse<Vec<UserResponse>> {
    user_service.list().await.json()
}

#[get("/api/users/{id}")]
pub async fn get_user(
    user_service: Data<UserService>,
    _admin: RequireAdmin,
    id: Path<Uuid>,
) -> AppResponse<UserResponse> {
    user_service.get(id.into_inner()).await.json()
}

#[patch("/api/users/{id}")]
pub async fn update_user(
    user_service: Data<UserService>,
    _admin: RequireAdmin,
    id: Path<Uuid>,
    body: Json<AdminUpdateUserRequest>,
) -> AppResponse<UserResponse> {
    user_service
        .update(id.into_inner(), body.into_inner())
        .await
        .json()
}

#[delete("/api/users/{id}")]
pub async fn delete_user(
    user_service: Data<UserService>,
    admin: RequireAdmin,
    id: Path<Uuid>,
) -> AppResponse<String> {
    user_service
        .delete(admin.user.claims.sub, id.into_inner())
        .await
        .map(|_| String::from("User deleted"))
        .json()
}
