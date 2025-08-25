use axum::{
    routing::{delete, get},
    Router,
};

use crate::inbound::http::AppState;

mod handlers;
use handlers::*;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/account/details", get(get_current_user))
        .route("/users/{id}", get(get_user))
        .route("/account/api-keys", get(list_api_keys).post(create_api_key))
        .route("/account/api-keys/{key_id}", delete(delete_api_key))
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        auth::models::TokenClaims,
        user::{
            models::{ApiKey, ApiKeyId, NewApiKey, User, UserId},
            service::MockUserService,
        },
    };
    use crate::inbound::http::{
        routes::users::models::{ApiKeyResponse, CreateApiKeyResponse, GetUserHttpResponse},
        AppState,
    };
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use mockall::predicate::*;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn setup_test_app(
        user_service: MockUserService,
        auth_service: crate::domain::auth::service::MockAuthService,
    ) -> axum::Router {
        let mut app_state = AppState::new_mock();
        app_state.user_service = Arc::new(user_service);
        app_state.auth_service = Arc::new(auth_service);
        crate::inbound::http::routes::users::router().with_state(app_state)
    }

    fn mock_auth(
        user_id: UserId,
        token: &'static str,
    ) -> crate::domain::auth::service::MockAuthService {
        let token_claims = TokenClaims::new_mock(user_id);
        let mut auth_service = crate::domain::auth::service::MockAuthService::new();
        auth_service
            .expect_validate_token()
            .with(eq(token))
            .times(1)
            .returning(move |_| Ok(token_claims.clone()));
        auth_service
    }

    #[tokio::test]
    async fn test_get_current_user_route() {
        let user = User::new_mock();
        let user_id = user.id;
        let token = "test-token";

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_get_user_by_id()
            .with(eq(user.id))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let app = setup_test_app(mock_user_service, mock_auth(user_id, token));

        let request = Request::builder()
            .uri("/account/details")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_as_user: GetUserHttpResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_as_user.user_id, user_id);
    }

    #[tokio::test]
    async fn test_list_api_keys_route() {
        let user = User::new_mock();
        let token = "test-token";

        let mut mock_user_service = MockUserService::new();
        let api_keys_to_return = vec![ApiKey {
            user_id: Some(user.id),
            ..Default::default()
        }];

        mock_user_service
            .expect_list_api_keys()
            .with(eq(user.id))
            .times(1)
            .returning(move |_| Ok(api_keys_to_return.clone()));

        let auth_service = mock_auth(user.id, token);
        let app = setup_test_app(mock_user_service, auth_service);

        let request = Request::builder()
            .uri("/account/api-keys")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_as_keys: Vec<ApiKeyResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_as_keys.len(), 1);
    }

    #[tokio::test]
    async fn test_get_user_by_id_route() {
        let user = User::new_mock();
        let user_id = user.id;
        let token = "test-token";

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_get_user_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(user.clone()));

        let app = setup_test_app(mock_user_service, mock_auth(user_id, token));

        let request = Request::builder()
            .uri(format!("/users/{}", user_id))
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_user_by_id_route_forbidden() {
        let authenticated_user_id = UserId::generate();
        let target_user_id = UserId::generate();
        let token = "test-token";

        let mock_user_service = MockUserService::new();
        let app = setup_test_app(mock_user_service, mock_auth(authenticated_user_id, token));

        let request = Request::builder()
            .uri(format!("/users/{}", target_user_id))
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_create_api_key_route() {
        let user = User::new_mock();
        let token = "test-token";

        let new_api_key = NewApiKey {
            id: ApiKeyId::generate(),
            ..Default::default()
        };
        let new_api_key_for_closure = new_api_key.clone();

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_create_api_key()
            .with(eq(user.id))
            .times(1)
            .returning(move |_| Ok(new_api_key_for_closure.clone()));

        let app = setup_test_app(mock_user_service, mock_auth(user.id, token));

        let request = Request::builder()
            .method("POST")
            .uri("/account/api-keys")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_as_key: CreateApiKeyResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_as_key.id, new_api_key.id);
    }

    #[tokio::test]
    async fn test_delete_api_key_route() {
        let user = User::new_mock();
        let token = "test-token";
        let key_id_to_delete = ApiKeyId::generate();

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_delete_api_key()
            .with(eq(user.id), eq(key_id_to_delete))
            .times(1)
            .returning(|_, _| Ok(()));

        let app = setup_test_app(mock_user_service, mock_auth(user.id, token));

        let request = Request::builder()
            .method("DELETE")
            .uri(format!("/account/api-keys/{}", key_id_to_delete))
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
