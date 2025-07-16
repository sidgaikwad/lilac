use async_trait::async_trait;
use chrono::{DateTime, Utc};
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::domain::user::{
    models::{CreateUserRequest, User, UserId},
    ports::{UserRepository, UserRepositoryError},
};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRecord {
    user_id: uuid::Uuid,
    email: String,
    name: Option<String>,
    password_hash: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserRecord> for User {
    fn from(record: UserRecord) -> Self {
        Self {
            id: record.user_id.into(),
            name: record.name.unwrap_or_default(),
            email: record.email,
            password_hash: record.password_hash,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserRepositoryError> {
        let password_hash = password_auth::generate_hash(req.password.expose_secret());

        let record = sqlx::query_as!(
            UserRecord,
            r#"
            INSERT INTO users (email, name, password_hash, email_verified, login_method)
            VALUES ($1, $2, $3, false, 'email')
            RETURNING user_id, email, name, password_hash, created_at, updated_at
            "#,
            req.email,
            req.name.as_ref(),
            password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                UserRepositoryError::Duplicate {
                    field: "email".to_string(),
                    value: req.email.clone(),
                }
            }
            _ => UserRepositoryError::Unknown(anyhow::anyhow!(err)),
        })?;

        Ok(record.into())
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError> {
        let record = sqlx::query_as!(UserRecord, r#"SELECT user_id, email, name, password_hash, created_at, updated_at FROM users WHERE user_id = $1"#, id.inner())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserRepositoryError::NotFound(id.to_string()),
                _ => UserRepositoryError::Unknown(anyhow::anyhow!(e)),
            })?;
        Ok(record.into())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, UserRepositoryError> {
        let record = sqlx::query_as!(UserRecord, r#"SELECT user_id, email, name, password_hash, created_at, updated_at FROM users WHERE email = $1"#, email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserRepositoryError::NotFound(email.to_string()),
                _ => UserRepositoryError::Unknown(anyhow::anyhow!(e)),
            })?;
        Ok(record.into())
    }

    async fn delete_user(&self, id: &UserId) -> Result<(), UserRepositoryError> {
        sqlx::query!("DELETE FROM users WHERE user_id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| UserRepositoryError::Unknown(anyhow::anyhow!(e)))
    }
}
