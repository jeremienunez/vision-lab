use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use perception_app::{
    CreateTrainingJobCommand, CreateTrainingJobUseCase, ListTrainingClassMetricsUseCase,
    ListTrainingMetricsUseCase, UseCaseError,
};
use perception_domain::{DatasetVersionId, TrainingJobId};

use crate::{
    dto::{
        error::ErrorResponse,
        training_job::{CreateTrainingJobRequest, TrainingJobResponse},
        training_metric::{ListTrainingClassMetricsResponse, ListTrainingMetricsResponse},
    },
    mappers,
    state::TrainingJobHttpState,
};

pub fn routes(state: TrainingJobHttpState) -> Router {
    Router::new()
        .route("/training-jobs", post(create_training_job))
        .route(
            "/training-jobs/{training_job_id}/metrics",
            get(list_training_metrics),
        )
        .route(
            "/training-jobs/{training_job_id}/metrics/by-class",
            get(list_training_class_metrics),
        )
        .with_state(state)
}

async fn create_training_job(
    State(state): State<TrainingJobHttpState>,
    Json(request): Json<CreateTrainingJobRequest>,
) -> Result<(StatusCode, Json<TrainingJobResponse>), TrainingJobRouteError> {
    let dataset_version_id = DatasetVersionId::parse(request.dataset_version_id)
        .map_err(|_| UseCaseError::Validation("invalid dataset version id"))?;
    let use_case = CreateTrainingJobUseCase::new_with_queue(
        state.dataset_version_repository(),
        state.training_job_repository(),
        state.training_job_queue(),
    );
    let job = use_case
        .execute(CreateTrainingJobCommand {
            dataset_version_id,
            model_family: request.model_family,
            base_model: request.base_model,
            epochs: request.hyperparameters.epochs,
            batch_size: request.hyperparameters.batch_size,
            image_size: request.hyperparameters.image_size,
            learning_rate: request.hyperparameters.learning_rate,
        })
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(mappers::training_job::training_job_response(job)),
    ))
}

async fn list_training_metrics(
    State(state): State<TrainingJobHttpState>,
    Path(training_job_id): Path<String>,
) -> Result<Json<ListTrainingMetricsResponse>, TrainingJobRouteError> {
    let training_job_id = TrainingJobId::parse(training_job_id)
        .map_err(|_| UseCaseError::Validation("invalid training job id"))?;
    let use_case = ListTrainingMetricsUseCase::new(
        state.training_job_repository(),
        state.training_metric_repository(),
    );
    let metrics = use_case.execute(training_job_id).await?;

    Ok(Json(ListTrainingMetricsResponse {
        metrics: metrics
            .into_iter()
            .map(mappers::training_metric::training_metric_response)
            .collect(),
    }))
}

async fn list_training_class_metrics(
    State(state): State<TrainingJobHttpState>,
    Path(training_job_id): Path<String>,
) -> Result<Json<ListTrainingClassMetricsResponse>, TrainingJobRouteError> {
    let training_job_id = TrainingJobId::parse(training_job_id)
        .map_err(|_| UseCaseError::Validation("invalid training job id"))?;
    let use_case = ListTrainingClassMetricsUseCase::new(
        state.training_job_repository(),
        state.training_metric_repository(),
    );
    let class_metrics = use_case.execute(training_job_id).await?;

    Ok(Json(ListTrainingClassMetricsResponse {
        class_metrics: class_metrics
            .into_iter()
            .map(mappers::training_metric::training_class_metric_response)
            .collect(),
    }))
}

struct TrainingJobRouteError {
    error: UseCaseError,
}

impl From<UseCaseError> for TrainingJobRouteError {
    fn from(error: UseCaseError) -> Self {
        Self { error }
    }
}

impl IntoResponse for TrainingJobRouteError {
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
