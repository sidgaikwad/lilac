use secrecy::ExposeSecret;

use crate::{
    model::user::{User, UserId},
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_user(&self, user_id: &UserId) -> Result<User, ServiceError> {
        let id = user_id.inner();
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash FROM "users" WHERE user_id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<User, ServiceError> {
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash FROM "users" WHERE email = $1
        "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: User) -> Result<UserId, ServiceError> {
        let user_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "users" (user_id, email, email_verified, password_hash) VALUES ($1, $2, $3, $4) RETURNING user_id
        "#,
        user.user_id.inner(),
        &user.email,
        &user.email_verified,
        &user.password_hash.expose_secret(),
    )
    .map(|row| UserId::new(row.user_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(user_id)
    }
    pub async fn delete_user(&self, user_id: &UserId) -> Result<(), ServiceError> {
        let id = user_id.inner();
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM "users" WHERE user_id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_oidc_user(&self, email: &String) -> Result<User, ServiceError> {
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            INSERT INTO "users" (email, email_verified, password_hash) VALUES ($1, true, 'oidc_user') RETURNING user_id, email, email_verified, password_hash
        "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
