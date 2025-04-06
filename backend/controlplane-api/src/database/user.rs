use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::{
    model::{
        organization::Organization,
        user::{User, UserId},
    },
    ServiceError,
};

pub async fn get_user(db: &PgPool, user_id: &UserId) -> Result<User, ServiceError> {
    let id = user_id.inner();
    let user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            SELECT * FROM "users" WHERE user_id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

pub async fn get_user_by_email(db: &PgPool, email: &String) -> Result<User, ServiceError> {
    let user = sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            SELECT * FROM "users" WHERE email = $1
        "#,
        email
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

pub async fn create_user(db: &PgPool, user: User) -> Result<UserId, ServiceError> {
    let user_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "users" (user_id, email, email_verified, password_hash, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING user_id
        "#,
        user.user_id.inner(),
        &user.email,
        &user.email_verified,
        &user.password_hash.expose_secret(),
        &user.created_at,
    )
    .map(|row| UserId::new(row.user_id))
    .fetch_one(db)
    .await?;
    Ok(user_id)
}

pub async fn list_organizations(
    db: &PgPool,
    user_id: &UserId,
) -> Result<Vec<Organization>, ServiceError> {
    let id = user_id.inner();
    let orgs = sqlx::query_as!(
        Organization,
        // language=PostgreSQL
        r#"
            SELECT o.* FROM "organization_memberships" m INNER JOIN organizations o ON m.organization_id = o.organization_id WHERE m.user_id = $1
        "#,
        id
    )
    .fetch_all(db)
    .await?;
    Ok(orgs)
}
