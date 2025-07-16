use async_trait::async_trait;
use sqlx::{postgres::PgDatabaseError, PgPool};

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

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, req: &CreateUserRequest) -> Result<User, UserRepositoryError> {
        let password_hash = req
            .password
            .as_ref()
            .map(|p| password_auth::generate_hash(p.as_str()));

        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, name, password_hash, email_verified, login_method)
            VALUES ($1, $2, $3, false, 'email')
            RETURNING user_id as "id!", email as "email!", name as "name!", password_hash as "password_hash?"
            "#,
            req.email,
            req.name.as_deref(),
            password_hash,
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(db_err)) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                if pg_err.code() == "23505" {
                    // unique_violation
                    Err(UserRepositoryError::Duplicate(
                        "email".to_string(),
                        req.email.clone(),
                    ))
                } else {
                    Err(UserRepositoryError::Unknown(anyhow::anyhow!(db_err)))
                }
            }
            Err(e) => Err(UserRepositoryError::Unknown(anyhow::anyhow!(e))),
        }
    }

    async fn get_user_by_id(&self, id: &UserId) -> Result<User, UserRepositoryError> {
        sqlx::query_as!(User, r#"SELECT user_id as "id!", email as "email!", name as "name!", password_hash as "password_hash?" FROM users WHERE user_id = $1"#, id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserRepositoryError::NotFound(id.0.to_string()),
                _ => UserRepositoryError::Unknown(anyhow::anyhow!(e)),
            })
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, UserRepositoryError> {
        sqlx::query_as!(User, r#"SELECT user_id as "id!", email as "email!", name as "name!", password_hash as "password_hash?" FROM users WHERE email = $1"#, email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserRepositoryError::NotFound(email.to_string()),
                _ => UserRepositoryError::Unknown(anyhow::anyhow!(e)),
            })
    }

    async fn delete_user(&self, id: &UserId) -> Result<(), UserRepositoryError> {
        sqlx::query!("DELETE FROM users WHERE user_id = $1", id.0)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e: sqlx::Error| UserRepositoryError::Unknown(anyhow::anyhow!(e)))
    }
}
