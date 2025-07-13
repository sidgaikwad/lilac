use secrecy::ExposeSecret;

use crate::{
    database::DatabaseError,
    model::user::{AuthProvider, User, UserId},
};

use super::Database;

#[derive(sqlx::FromRow)]
struct UserRecord {
    user_id: uuid::Uuid,
    email: String,
    email_verified: bool,
    password_hash: Option<String>,
    login_method: Option<AuthProvider>,
    sso_provider_id: Option<String>,
}

impl From<UserRecord> for User {
    fn from(user_record: UserRecord) -> Self {
        User::new(
            UserId::new(user_record.user_id),
            user_record.email,
            user_record.email_verified,
            user_record.password_hash,
            user_record.login_method,
            user_record.sso_provider_id,
        )
    }
}

impl Database {
    pub async fn get_user(&self, user_id: &UserId) -> Result<User, DatabaseError> {
        let id = user_id.inner();
        let user_record = sqlx::query_as!(
            UserRecord,
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash, login_method as "login_method: _", sso_provider_id FROM "users" WHERE user_id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_record.into())
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<User, DatabaseError> {
        let user_record = sqlx::query_as!(
            UserRecord,
            // language=PostgreSQL
            r#"
            SELECT user_id, email, email_verified, password_hash, login_method as "login_method: _", sso_provider_id FROM "users" WHERE email = $1
        "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_record.into())
    }

    pub async fn create_user(&self, user: User) -> Result<UserId, DatabaseError> {
        let user_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "users" (user_id, email, email_verified, password_hash, login_method, sso_provider_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING user_id
        "#,
        user.user_id.inner(),
        &user.email,
        &user.email_verified,
        user.password_hash.as_ref().map(|s| s.expose_secret()),
        user.login_method as _,
        user.sso_provider_id,
    )
    .map(|row| UserId::new(row.user_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(user_id)
    }
    pub async fn delete_user(&self, user_id: &UserId) -> Result<(), DatabaseError> {
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
