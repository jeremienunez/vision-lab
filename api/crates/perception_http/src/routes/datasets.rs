use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use perception_app::{CreateDatasetUseCase, ListDatasetsUseCase, UseCaseError};

use crate::{
    dto::{
        dataset::{CreateDatasetRequest, DatasetResponse, ListDatasetsResponse},
        error::ErrorResponse,
    },
    mappers,
    state::HttpState,
};

pub fn routes(state: HttpState) -> Router {
    Router::new()
        .route("/datasets", post(create_dataset).get(list_datasets))
        .with_state(state)
}

async fn create_dataset(
    State(state): State<HttpState>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<(StatusCode, Json<DatasetResponse>), DatasetRouteError> {
    let command = mappers::dataset::create_dataset_command(request)?;
    let use_case = CreateDatasetUseCase::new(state.dataset_repository());
    let dataset = use_case.execute(command).await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::dataset::dataset_response(dataset)),
    ))
}

async fn list_datasets(
    State(state): State<HttpState>,
) -> Result<Json<ListDatasetsResponse>, DatasetRouteError> {
    let use_case = ListDatasetsUseCase::new(state.dataset_repository());
    let datasets = use_case.execute().await?;

    Ok(Json(ListDatasetsResponse {
        datasets: datasets
            .into_iter()
            .map(mappers::dataset::dataset_response)
            .collect(),
    }))
}

struct DatasetRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for DatasetRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for DatasetRouteError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self.error {
            UseCaseError::Validation(message) => {
                (StatusCode::BAD_REQUEST, "validation_failed", message.to_owned())
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
