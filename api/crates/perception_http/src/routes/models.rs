use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use perception_app::{GetModelUseCase, ListModelsUseCase, UseCaseError};
use perception_domain::ModelId;

use crate::{
    dto::{
        error::ErrorResponse,
        model::{ListModelsResponse, ModelResponse},
    },
    mappers,
    state::ModelHttpState,
};

pub fn routes(state: ModelHttpState) -> Router {
    Router::new()
        .route("/models", get(list_models))
        .route("/models/{model_id}", get(get_model))
        .with_state(state)
}

async fn list_models(
    State(state): State<ModelHttpState>,
) -> Result<Json<ListModelsResponse>, ModelRouteError> {
    let models = ListModelsUseCase::new(state.model_repository())
        .execute()
        .await?;

    Ok(Json(ListModelsResponse {
        models: models
            .into_iter()
            .map(mappers::model::model_response)
            .collect(),
    }))
}

async fn get_model(
    State(state): State<ModelHttpState>,
    Path(model_id): Path<String>,
) -> Result<Json<ModelResponse>, ModelRouteError> {
    let model_id =
        ModelId::parse(model_id).map_err(|_| UseCaseError::Validation("invalid model id"))?;
    let model = GetModelUseCase::new(state.model_repository())
        .execute(model_id)
        .await?;

    Ok(Json(mappers::model::model_response(model)))
}

struct ModelRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for ModelRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for ModelRouteError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self.error {
            UseCaseError::Validation(message) => (
                StatusCode::BAD_REQUEST,
                "validation_failed",
                message.to_owned(),
            ),
            UseCaseError::NotFound(message) => {
                (StatusCode::NOT_FOUND, "not_found", message.to_owned())
            }
            UseCaseError::Repository(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "repository_failed",
                message.to_owned(),
            ),
        };

        (status, Json(ErrorResponse::new(code, message))).into_response()
    }
}
