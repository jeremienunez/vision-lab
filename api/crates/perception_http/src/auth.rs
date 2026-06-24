use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use crate::dto::error::ErrorResponse;

const API_KEY_HEADER: &str = "x-api-key";
const API_KEY_ENV: &str = "PERCEPTIONLAB_API_KEY";
const PUBLIC_HEALTH_PATH: &str = "/health";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiKeyAuthConfig {
    expected_api_key: Option<String>,
}

impl ApiKeyAuthConfig {
    pub fn disabled() -> Self {
        Self {
            expected_api_key: None,
        }
    }

    pub fn required(expected_api_key: impl Into<String>) -> Self {
        let expected_api_key = expected_api_key.into().trim().to_owned();

        if expected_api_key.is_empty() {
            Self::disabled()
        } else {
            Self {
                expected_api_key: Some(expected_api_key),
            }
        }
    }

    pub fn from_env() -> Self {
        std::env::var(API_KEY_ENV)
            .map(Self::required)
            .unwrap_or_else(|_| Self::disabled())
    }

    pub fn is_enabled(&self) -> bool {
        self.expected_api_key.is_some()
    }

    fn expected_api_key(&self) -> Option<&str> {
        self.expected_api_key.as_deref()
    }
}

impl From<Option<String>> for ApiKeyAuthConfig {
    fn from(api_key: Option<String>) -> Self {
        api_key
            .map(ApiKeyAuthConfig::required)
            .unwrap_or_else(ApiKeyAuthConfig::disabled)
    }
}

pub fn with_optional_api_key_auth(router: Router, config: impl Into<ApiKeyAuthConfig>) -> Router {
    let config = config.into();

    if !config.is_enabled() {
        return router;
    }

    router.layer(middleware::from_fn_with_state(config, require_api_key))
}

async fn require_api_key(
    State(config): State<ApiKeyAuthConfig>,
    request: Request,
    next: Next,
) -> Response {
    if request.uri().path() == PUBLIC_HEALTH_PATH {
        return next.run(request).await;
    }

    let Some(expected_api_key) = config.expected_api_key() else {
        return next.run(request).await;
    };

    let Some(header_value) = request.headers().get(API_KEY_HEADER) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                "missing_api_key",
                "Missing x-api-key header",
            )),
        )
            .into_response();
    };

    let Ok(candidate_api_key) = header_value.to_str() else {
        return invalid_api_key_response();
    };

    if candidate_api_key != expected_api_key {
        return invalid_api_key_response();
    }

    next.run(request).await
}

fn invalid_api_key_response() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(ErrorResponse::new(
            "invalid_api_key",
            "Invalid x-api-key header",
        )),
    )
        .into_response()
}
