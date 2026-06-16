use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use perception_app::{RegisterModelCommand, RegisterModelUseCase, UseCaseError};
use perception_domain::TrainingJobId;

use crate::{
    dto::{
        error::ErrorResponse,
        model::{ModelResponse, RegisterModelRequest},
    },
    mappers,
    state::ModelRegistrationHttpState,
};

pub fn routes(state: ModelRegistrationHttpState) -> Router {
    Router::new()
        .route("/models", post(register_model))
        .with_state(state)
}

async fn register_model(
    axum::extract::State(state): axum::extract::State<ModelRegistrationHttpState>,
    Json(request): Json<RegisterModelRequest>,
) -> Result<(StatusCode, Json<ModelResponse>), ModelRegistrationRouteError> {
    let training_job_id = TrainingJobId::parse(request.training_job_id)
        .map_err(|_| UseCaseError::Validation("invalid training job id"))?;
    let model =
        RegisterModelUseCase::new(state.training_job_repository(), state.model_repository())
            .execute(RegisterModelCommand {
                training_job_id,
                name: request.name,
                version: request.version,
                artifact_uri: request.artifact_uri,
                metrics_summary: request.metrics_summary,
            })
            .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::model::model_response(model)),
    ))
}

struct ModelRegistrationRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for ModelRegistrationRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for ModelRegistrationRouteError {
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
