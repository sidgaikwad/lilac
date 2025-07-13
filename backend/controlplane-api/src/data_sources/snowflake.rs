use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde_json::{json, Value};

use crate::{model::dataset::SnowflakeConnector, ServiceError};

pub async fn check_snowflake_access(
    snowflake_connector: &SnowflakeConnector,
) -> Result<(), ServiceError> {
    let http = Client::builder()
        .gzip(true)
        .use_rustls_tls()
        .build()
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to construct HTTP client");
            ServiceError::InternalError(e.to_string())
        })?;
    login(&http, snowflake_connector).await?;
    Ok(())
}

async fn login(
    http: &Client,
    snowflake_connector: &SnowflakeConnector,
) -> Result<String, ServiceError> {
    let url = format!(
        "https://{account}.snowflakecomputing.com/session/v1/login-request",
        account = snowflake_connector.account(),
    );

    let mut queries = vec![];
    if let Some(warehouse) = &snowflake_connector.warehouse() {
        queries.push(("warehouse", warehouse));
    }
    if let Some(database) = &snowflake_connector.database() {
        queries.push(("databaseName", database));
    }
    if let Some(schema) = &snowflake_connector.schema() {
        queries.push(("schemaName", schema));
    }
    if let Some(role) = &snowflake_connector.role() {
        queries.push(("roleName", role));
    }

    let login_data = login_request_data(
        snowflake_connector.username(),
        snowflake_connector.password(),
        snowflake_connector.account(),
    );
    let response = http
        .post(url)
        .query(&queries)
        .json(&json!({
            "data": login_data
        }))
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to login to snowflake");
            ServiceError::InternalError(e.to_string())
        })?;
    let status = response.status();
    let body = response.text().await.map_err(|e| {
        tracing::error!(error = ?e, "failed to decode login response");
        ServiceError::InternalError(e.to_string())
    })?;
    if !status.is_success() {
        return Err(ServiceError::Unauthorized {
            reason: "failed to authenticate with Snowflake".into(),
        });
    }

    let response: Response =
        serde_json::from_str(&body).map_err(|e| ServiceError::InternalError(e.to_string()))?;
    if !response.success {
        return Err(ServiceError::Unauthorized {
            reason: "failed to authenticate with Snowflake".into(),
        });
    }

    Ok(response.data.token)
}

fn login_request_data(username: &str, password: &SecretString, account: &str) -> Value {
    json!({
        "LOGIN_NAME": username,
        "PASSWORD": password.expose_secret(),
        "ACCOUNT_NAME": account,
    })
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(serde:: Deserialize)]
struct Response {
    data: LoginResponse,
    message: Option<String>,
    success: bool,
}
