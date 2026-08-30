use std::sync::Arc;

use uuid::Uuid;
use validator::Validate;

use crate::{
    entities::{
        app_error::AppError,
        user::{AdminCreateUserRequest, AdminUpdateUserRequest, RegisterRequest, UserResponse},
    },
    enums::user_role::UserRole,
    helpers::hash_helper::hash_password,
    repositories::user_repository::UserRepository,
    services::{auth_service::normalize_email, session_service::SessionService},
};

#[derive(Clone)]
pub struct UserService {
    users: Arc<UserRepository>,
    sessions: Arc<SessionService>,
}

impl UserService {
    pub fn new(users: Arc<UserRepository>, sessions: Arc<SessionService>) -> Self {
        Self { users, sessions }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<UserResponse, AppError> {
        request
            .validate()
            .map_err(|_| AppError::bad_request("Invalid registration request"))?;
        let name = normalize_name(request.name)?;
        let email = normalize_email(&request.email);
        let password_hash = hash_password(&request.password)?;
        self.users
            .create(&name, &email, &password_hash, UserRole::Member)
            .await
            .map(Into::into)
    }

    pub async fn create_by_admin(
        &self,
        request: AdminCreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        request
            .validate()
            .map_err(|_| AppError::bad_request("Invalid user request"))?;
        let name = normalize_name(request.name)?;
        let email = normalize_email(&request.email);
        let password_hash = hash_password(&request.password)?;
        self.users
            .create(&name, &email, &password_hash, request.role)
            .await
            .map(Into::into)
    }

    pub async fn list(&self) -> Result<Vec<UserResponse>, AppError> {
        self.users
            .list()
            .await
            .map(|users| users.into_iter().map(Into::into).collect())
    }

    pub async fn get(&self, id: Uuid) -> Result<UserResponse, AppError> {
        self.users.find_by_id(id).await.map(Into::into)
    }

    pub async fn update(
        &self,
        id: Uuid,
        request: AdminUpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        if request.is_empty() {
            return Err(AppError::bad_request(
                "Provide at least one field to update",
            ));
        }
        request
            .validate()
            .map_err(|_| AppError::bad_request("Invalid user request"))?;
        let target = self.users.find_by_id(id).await?;
        let role_changed = request.role.is_some_and(|role| role != target.role);
        if target.role.is_admin() && request.role.is_some_and(|role| !role.is_admin()) {
            self.ensure_not_last_admin().await?;
        }

        let name = request.name.map(normalize_name).transpose()?;
        let email = request.email.map(|email| normalize_email(&email));
        let password_hash = request.password.as_deref().map(hash_password).transpose()?;
        let password_changed = password_hash.is_some();
        let user = self
            .users
            .update(
                id,
                name.as_deref(),
                email.as_deref(),
                password_hash.as_deref(),
                request.role,
            )
            .await?;

        if password_changed || role_changed {
            self.sessions.revoke_all_user_sessions(id).await?;
        }
        Ok(user.into())
    }

    pub async fn delete(&self, actor_id: Uuid, target_id: Uuid) -> Result<(), AppError> {
        if actor_id == target_id {
            return Err(AppError::bad_request(
                "Administrators cannot delete their own account",
            ));
        }
        let target = self.users.find_by_id(target_id).await?;
        if target.role.is_admin() {
            self.ensure_not_last_admin().await?;
        }
        self.users.delete(target_id).await?;
        self.sessions.revoke_all_user_sessions(target_id).await
    }

    async fn ensure_not_last_admin(&self) -> Result<(), AppError> {
        if self.users.count_admins().await? <= 1 {
            return Err(AppError::conflict(
                "The last administrator cannot be removed or demoted",
            ));
        }
        Ok(())
    }
}

fn normalize_name(name: String) -> Result<String, AppError> {
    let name = name.trim().to_owned();
    if name.is_empty() {
        return Err(AppError::bad_request("Name is required"));
    }
    Ok(name)
}
