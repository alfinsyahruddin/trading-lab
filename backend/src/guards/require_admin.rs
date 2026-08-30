use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};

use crate::{
    entities::app_error::AppError,
    guards::authenticated_user::{authenticate_request, AuthenticatedUser},
};

#[derive(Debug)]
pub struct RequireAdmin {
    pub user: AuthenticatedUser,
}

impl FromRequest for RequireAdmin {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = request.clone();
        Box::pin(async move {
            let user = authenticate_request(&request).await?;
            if !user.claims.role.is_admin() {
                return Err(AppError::Forbidden(
                    "Administrator access is required".into(),
                ));
            }
            Ok(Self { user })
        })
    }
}
