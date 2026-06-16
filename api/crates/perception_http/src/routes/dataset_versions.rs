use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use perception_app::{CreateDatasetVersionCommand, CreateDatasetVersionUseCase, UseCaseError};
use perception_domain::DatasetId;

use crate::{
    dto::{
        dataset_version::{CreateDatasetVersionRequest, DatasetVersionResponse},
        error::ErrorResponse,
    },
    mappers,
    state::DatasetVersionHttpState,
};

pub fn routes(state: DatasetVersionHttpState) -> Router {
    Router::new()
        .route(
            "/datasets/{dataset_id}/versions",
            post(create_dataset_version),
        )
        .with_state(state)
}

async fn create_dataset_version(
    State(state): State<DatasetVersionHttpState>,
    Path(dataset_id): Path<String>,
    Json(request): Json<CreateDatasetVersionRequest>,
) -> Result<(StatusCode, Json<DatasetVersionResponse>), DatasetVersionRouteError> {
    let dataset_id =
        DatasetId::parse(dataset_id).map_err(|_| UseCaseError::Validation("invalid dataset id"))?;
    let use_case = CreateDatasetVersionUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.annotation_repository(),
        state.dataset_version_repository(),
    );
    let version = use_case
        .execute(CreateDatasetVersionCommand {
            dataset_id,
            version_name: request.version_name,
            split_config: request.split_config,
            created_by: request.created_by,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::dataset_version::dataset_version_response(version)),
    ))
}

struct DatasetVersionRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for DatasetVersionRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for DatasetVersionRouteError {
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
