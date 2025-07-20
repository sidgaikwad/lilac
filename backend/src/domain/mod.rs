use secrecy::{ExposeSecret, SecretString};

pub mod auth;
pub mod cluster;
pub mod credentials;
pub mod dataset;
pub mod project;
pub mod user;
pub mod workspace;

pub fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}
