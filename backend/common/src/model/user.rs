use std::fmt::Display;

use password_auth::generate_hash;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl UserId {
    pub fn new(user_id: Uuid) -> Self {
        Self(user_id)
    }

    pub fn generate() -> Self {
        let id = Uuid::new_v4();
        Self(id)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for UserId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("UserId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct User {
    pub user_id: UserId,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: Option<SecretString>,
    pub oidc_provider: Option<String>,
    pub oidc_provider_id: Option<String>,
}

impl User {
    pub fn new(
        user_id: UserId,
        email: String,
        email_verified: bool,
        password_hash: Option<String>,
        oidc_provider: Option<String>,
        oidc_provider_id: Option<String>,
    ) -> Self {
        Self {
            user_id,
            email,
            email_verified,
            password_hash: password_hash.map(SecretString::from),
            oidc_provider,
            oidc_provider_id,
        }
    }

    pub fn create_password_user(email: String, password: SecretString) -> Self {
        Self {
            user_id: UserId::generate(),
            email,
            email_verified: false,
            password_hash: Some(SecretString::from(generate_hash(password.expose_secret()))),
            oidc_provider: None,
            oidc_provider_id: None,
        }
    }

    pub fn create_oidc_user(email: String, oidc_provider: String, oidc_provider_id: String) -> Self {
        Self {
            user_id: UserId::generate(),
            email,
            email_verified: true,
            password_hash: None,
            oidc_provider: Some(oidc_provider),
            oidc_provider_id: Some(oidc_provider_id),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type, strum::Display)]
#[sqlx(type_name = "auth_provider")]
#[sqlx(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum AuthProvider {
    Email,
    Google,
    Github,
    Gitlab,
    Ldap,
    Oidc,
}
