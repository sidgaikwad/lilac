use secrecy::{ExposeSecret, SecretString};

use crate::{
    model::user::{User, UserId},
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_user(&self, user_id: &UserId) -> Result<User, ServiceError> {
        let id = user_id.inner();
        let user_record = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash, oidc_provider, oidc_provider_id FROM "users" WHERE user_id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            user_id: UserId::new(user_record.user_id),
            email: user_record.email,
            email_verified: user_record.email_verified,
            password_hash: user_record.password_hash.map(SecretString::from),
            oidc_provider: user_record.oidc_provider,
            oidc_provider_id: user_record.oidc_provider_id,
        })
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<User, ServiceError> {
        let user_record = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash, oidc_provider, oidc_provider_id FROM "users" WHERE email = $1
        "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            user_id: UserId::new(user_record.user_id),
            email: user_record.email,
            email_verified: user_record.email_verified,
            password_hash: user_record.password_hash.map(SecretString::from),
            oidc_provider: user_record.oidc_provider,
            oidc_provider_id: user_record.oidc_provider_id,
        })
    }

    pub async fn create_user(&self, user: User) -> Result<UserId, ServiceError> {
        let user_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "users" (user_id, email, email_verified, password_hash, oidc_provider, oidc_provider_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING user_id
        "#,
        user.user_id.inner(),
        &user.email,
        &user.email_verified,
        user.password_hash.as_ref().map(|s| s.expose_secret()),
        user.oidc_provider,
        user.oidc_provider_id,
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
}
