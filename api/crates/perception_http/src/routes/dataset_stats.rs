use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use perception_app::{DatasetStatsUseCase, UseCaseError};
use perception_domain::DatasetId;

use crate::{
    dto::{dataset_stats::DatasetStatsResponse, error::ErrorResponse},
    mappers,
    state::DatasetStatsHttpState,
};

pub fn routes(state: DatasetStatsHttpState) -> Router {
    Router::new()
        .route("/datasets/{dataset_id}/stats", get(dataset_stats))
        .with_state(state)
}

async fn dataset_stats(
    State(state): State<DatasetStatsHttpState>,
    Path(dataset_id): Path<String>,
) -> Result<Json<DatasetStatsResponse>, DatasetStatsRouteError> {
    let dataset_id =
        DatasetId::parse(dataset_id).map_err(|_| UseCaseError::Validation("invalid dataset id"))?;
    let use_case = DatasetStatsUseCase::new(
        state.dataset_repository(),
        state.sample_repository(),
        state.annotation_repository(),
    );
    let stats = use_case.execute(dataset_id).await?;

    Ok(Json(mappers::dataset_stats::dataset_stats_response(stats)))
}

struct DatasetStatsRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for DatasetStatsRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for DatasetStatsRouteError {
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
