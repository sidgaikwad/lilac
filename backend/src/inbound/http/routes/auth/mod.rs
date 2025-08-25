use crate::inbound::http::AppState;
use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
use handlers::*;
mod models;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_with_username))
        .route("/auth/signup", post(sign_up))
        .route("/auth/logout", get(logout))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::LilacConfig,
        domain::{
            auth::{models::Token, service::MockAuthService},
            user::{
                models::{User, UserId},
                service::MockUserService,
            },
        },
        inbound::http::{
            routes::auth::models::{LoginHttpRequest, SignUpHttpRequest, SignUpHttpResponse},
            AppState,
        },
    };
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use secrecy::SecretString;
    use std::sync::Arc;
    use tower::ServiceExt;
    use tower_sessions::{MemoryStore, SessionManagerLayer};

    fn setup_auth_router_test_app(
        config: LilacConfig,
        auth_service: MockAuthService,
        user_service: MockUserService,
    ) -> axum::Router {
        let mut app_state = AppState::new_mock();
        app_state.config = Arc::new(config);
        app_state.auth_service = Arc::new(auth_service);
        app_state.user_service = Arc::new(user_service);

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store);

        router().with_state(app_state).layer(session_layer)
    }

    #[tokio::test]
    async fn test_login_route_success() {
        let request_body = LoginHttpRequest {
            username: "testuser".to_string(),
            password: SecretString::from("password123".to_string()),
        };
        let token_to_return = Token {
            access_token: "mock-jwt-token".to_string(),
            token_type: "Bearer".to_string(),
        };

        let mut mock_auth_service = MockAuthService::new();
        mock_auth_service
            .expect_login_with_username()
            .times(1)
            .returning(move |_, _| Ok(token_to_return.clone()));

        let app =
            setup_auth_router_test_app(Default::default(), mock_auth_service, Default::default());

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_body: Token = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_body.access_token, "mock-jwt-token");
    }

    #[tokio::test]
    async fn test_signup_route_success() {
        let request_body = SignUpHttpRequest {
            username: "newuser".to_string(),
            first_name: None,
            last_name: None,
            password: "password123".into(),
        };
        let user_id_to_return = UserId::generate();

        let mut mock_user_service = MockUserService::new();
        mock_user_service
            .expect_create_user()
            .times(1)
            .returning(move |_| {
                Ok(User {
                    id: user_id_to_return,
                    ..Default::default()
                })
            });

        let config = LilacConfig {
            disable_sign_up: false,
            ..Default::default()
        };
        let app = setup_auth_router_test_app(config, Default::default(), mock_user_service);

        let request = Request::builder()
            .method("POST")
            .uri("/auth/signup")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_body: SignUpHttpResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_body.user_id, user_id_to_return);
    }

    #[tokio::test]
    async fn test_logout_route() {
        let app =
            setup_auth_router_test_app(Default::default(), Default::default(), Default::default());

        let request = Request::builder()
            .uri("/auth/logout")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.headers().get("location").unwrap(), "/");
    }
}
