use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};

use crate::dto::error::ErrorResponse;

const API_KEY_HEADER: &str = "x-api-key";
const API_KEY_ENV: &str = "PERCEPTIONLAB_API_KEY";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiKeyAuthConfig {
    required_key: Option<String>,
}

impl ApiKeyAuthConfig {
    pub fn disabled() -> Self {
        Self { required_key: None }
    }

    pub fn required(key: impl Into<String>) -> Self {
        let key = key.into().trim().to_owned();

        if key.is_empty() {
            Self::disabled()
        } else {
            Self {
                required_key: Some(key),
            }
        }
    }

    pub fn from_env() -> Self {
        std::env::var(API_KEY_ENV)
            .map(Self::required)
            .unwrap_or_else(|_| Self::disabled())
    }

    pub fn is_enabled(&self) -> bool {
        self.required_key.is_some()
    }

    fn required_key(&self) -> Option<&str> {
        self.required_key.as_deref()
    }
}

pub fn with_optional_api_key_auth(router: Router, config: ApiKeyAuthConfig) -> Router {
    if !config.is_enabled() {
        return router;
    }

    router.layer(middleware::from_fn_with_state(config, require_api_key))
}

async fn require_api_key(
    State(config): State<ApiKeyAuthConfig>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }

    let Some(required_key) = config.required_key() else {
        return next.run(request).await;
    };

    match request
        .headers()
        .get(API_KEY_HEADER)
        .and_then(|value| value.to_str().ok())
    {
        Some(provided_key) if provided_key == required_key => next.run(request).await,
        Some(_) => auth_error(StatusCode::FORBIDDEN, "api_key_invalid", "invalid api key"),
        None => auth_error(
            StatusCode::UNAUTHORIZED,
            "api_key_missing",
            "missing api key",
        ),
    }
}

fn auth_error(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    (status, Json(ErrorResponse::new(code, message))).into_response()
}
