use async_trait::async_trait;
use chrono::{DateTime, Utc};
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::domain::user::{
    models::{ApiKey, ApiKeyId, CreateUserRequest, User, UserId},
    ports::{ApiKeyRepository, ApiKeyRepositoryError, UserRepository, UserRepositoryError},
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

#[derive(sqlx::FromRow)]
struct ApiKeyRecord {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    prefix: String,
    key_hash: String,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
}

impl From<ApiKeyRecord> for ApiKey {
    fn from(record: ApiKeyRecord) -> Self {
        Self {
            id: record.id.into(),
            user_id: record.user_id.into(),
            prefix: record.prefix,
            key_hash: record.key_hash,
            created_at: record.created_at,
            last_used_at: record.last_used_at,
            expires_at: record.expires_at,
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

#[async_trait]
impl ApiKeyRepository for PostgresUserRepository {
    async fn create_api_key(&self, key: &ApiKey) -> Result<(), ApiKeyRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO api_keys (id, user_id, prefix, key_hash, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            key.id.inner(),
            key.user_id.inner(),
            key.prefix,
            key.key_hash,
            key.created_at,
            key.expires_at,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|err| ApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))
    }

    async fn find_user_by_api_key_hash(
        &self,
        key_hash: &str,
    ) -> Result<User, ApiKeyRepositoryError> {
        let user_record = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT u.user_id, u.email, u.name, u.password_hash, u.created_at, u.updated_at
            FROM users u
            JOIN api_keys ak ON u.user_id = ak.user_id
            WHERE ak.key_hash = $1 AND (ak.expires_at IS NULL OR ak.expires_at > now())
            "#,
            key_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ApiKeyRepositoryError::NotFound,
            _ => ApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)),
        })?;

        Ok(user_record.into())
    }

    async fn list_api_keys_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<ApiKey>, ApiKeyRepositoryError> {
        let records = sqlx::query_as!(
            ApiKeyRecord,
            r#"
            SELECT id, user_id, prefix, key_hash, created_at, last_used_at, expires_at
            FROM api_keys
            WHERE user_id = $1
            "#,
            user_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))?;

        Ok(records.into_iter().map(ApiKey::from).collect())
    }

    async fn delete_api_key(&self, id: &ApiKeyId) -> Result<(), ApiKeyRepositoryError> {
        let result = sqlx::query!("DELETE FROM api_keys WHERE id = $1", id.inner())
            .execute(&self.pool)
            .await
            .map_err(|err| ApiKeyRepositoryError::Unknown(anyhow::anyhow!(err)))?;

        if result.rows_affected() == 0 {
            return Err(ApiKeyRepositoryError::NotFound);
        }

        Ok(())
    }
}
