#![forbid(unsafe_code)]
//! HTTP routes, DTOs, mappers, and public API errors for PerceptionLab.

use axum::Router;
use std::sync::Arc;

use perception_app::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, InferenceEngine,
    InferenceRunRepository, ModelExportRepository, ModelRepository, OverlayRenderer,
    SampleRepository, SampleStorage, TrainingJobQueue, TrainingJobRepository,
    TrainingMetricRepository,
};

pub mod dto;
pub mod mappers;
pub mod routes;
pub mod state;

pub const CRATE_NAME: &str = "perception_http";

pub fn router() -> Router {
    routes::health::routes()
}

pub fn router_with_dataset_repository(dataset_repository: Arc<dyn DatasetRepository>) -> Router {
    routes::health::routes().merge(routes::datasets::routes(state::HttpState::new(
        dataset_repository,
    )))
}

pub fn router_with_application_ports(
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    sample_storage: Arc<dyn SampleStorage>,
) -> Router {
    routes::health::routes()
        .merge(routes::datasets::routes(state::HttpState::new(
            dataset_repository.clone(),
        )))
        .merge(routes::samples::routes(state::SampleHttpState::new(
            dataset_repository,
            sample_repository,
            sample_storage,
        )))
}

pub fn router_with_annotation_ports(
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    sample_storage: Arc<dyn SampleStorage>,
    annotation_repository: Arc<dyn AnnotationRepository>,
) -> Router {
    router_with_application_ports(
        dataset_repository.clone(),
        sample_repository.clone(),
        sample_storage,
    )
    .merge(routes::dataset_stats::routes(
        state::DatasetStatsHttpState::new(
            dataset_repository.clone(),
            sample_repository.clone(),
            annotation_repository.clone(),
        ),
    ))
    .merge(routes::annotations::routes(
        state::AnnotationHttpState::new(
            dataset_repository,
            sample_repository,
            annotation_repository,
        ),
    ))
}

pub fn router_with_version_ports(
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    sample_storage: Arc<dyn SampleStorage>,
    annotation_repository: Arc<dyn AnnotationRepository>,
    dataset_version_repository: Arc<dyn DatasetVersionRepository>,
) -> Router {
    router_with_annotation_ports(
        dataset_repository.clone(),
        sample_repository.clone(),
        sample_storage,
        annotation_repository.clone(),
    )
    .merge(routes::dataset_versions::routes(
        state::DatasetVersionHttpState::new(
            dataset_repository,
            sample_repository,
            annotation_repository,
            dataset_version_repository,
        ),
    ))
}

pub fn router_with_training_job_ports(
    dataset_version_repository: Arc<dyn DatasetVersionRepository>,
    training_job_repository: Arc<dyn TrainingJobRepository>,
    training_job_queue: Arc<dyn TrainingJobQueue>,
    training_metric_repository: Arc<dyn TrainingMetricRepository>,
) -> Router {
    routes::health::routes().merge(routes::training_jobs::routes(
        state::TrainingJobHttpState::new(
            dataset_version_repository,
            training_job_repository,
            training_job_queue,
            training_metric_repository,
        ),
    ))
}

pub fn router_with_model_ports(
    model_repository: Arc<dyn ModelRepository>,
    model_export_repository: Arc<dyn ModelExportRepository>,
    inference_run_repository: Arc<dyn InferenceRunRepository>,
    overlay_renderer: Arc<dyn OverlayRenderer>,
    inference_engine: Arc<dyn InferenceEngine>,
) -> Router {
    routes::health::routes().merge(routes::models::routes(state::ModelHttpState::new(
        model_repository,
        model_export_repository,
        inference_run_repository,
        overlay_renderer,
        inference_engine,
    )))
}

pub fn router_with_p0_ports(
    dataset_repository: Arc<dyn DatasetRepository>,
    sample_repository: Arc<dyn SampleRepository>,
    sample_storage: Arc<dyn SampleStorage>,
    annotation_repository: Arc<dyn AnnotationRepository>,
    dataset_version_repository: Arc<dyn DatasetVersionRepository>,
    training_job_repository: Arc<dyn TrainingJobRepository>,
    training_job_queue: Arc<dyn TrainingJobQueue>,
    training_metric_repository: Arc<dyn TrainingMetricRepository>,
    model_repository: Arc<dyn ModelRepository>,
    model_export_repository: Arc<dyn ModelExportRepository>,
    inference_run_repository: Arc<dyn InferenceRunRepository>,
    overlay_renderer: Arc<dyn OverlayRenderer>,
    inference_engine: Arc<dyn InferenceEngine>,
) -> Router {
    router_with_version_ports(
        dataset_repository,
        sample_repository,
        sample_storage,
        annotation_repository,
        dataset_version_repository.clone(),
    )
    .merge(routes::training_jobs::routes(
        state::TrainingJobHttpState::new(
            dataset_version_repository,
            training_job_repository,
            training_job_queue,
            training_metric_repository,
        ),
    ))
    .merge(routes::models::routes(state::ModelHttpState::new(
        model_repository,
        model_export_repository,
        inference_run_repository,
        overlay_renderer,
        inference_engine,
    )))
}
