use crate::{auth::error::AuthError, AppState};
use common::model::user::{AuthProvider, User, UserId};
use std::str::FromStr;

pub async fn get_or_create_sso_user(
    app_state: &AppState,
    email: String,
    provider: String,
    sso_id: String,
) -> Result<User, AuthError> {
    match app_state.db.get_user_by_email(&email).await {
        Ok(user) => {
            if user.login_method != Some(AuthProvider::from_str(&provider).unwrap()) {
                return Err(AuthError::DuplicateUser);
            }
            Ok(user)
        }
        Err(_) => {
            let new_user = User::create_sso_user(
                email.clone(),
                AuthProvider::from_str(&provider).unwrap(),
                sso_id,
            );
            app_state.db.create_user(new_user).await?;
            Ok(app_state.db.get_user_by_email(&email).await?)
        }
    }
}

pub fn generate_jwt(user_id: UserId) -> Result<String, AuthError> {
    let claims = crate::auth::claims::Claims::create(user_id);
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &crate::auth::keys::KEYS.get().unwrap().encoding,
    )
    .map_err(|_| AuthError::TokenCreation)
}
