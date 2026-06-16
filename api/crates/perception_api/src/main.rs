#![forbid(unsafe_code)]

use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "perception_api=info".into());

    tracing_subscriber::fmt().with_env_filter(filter).init();
    let address: SocketAddr = std::env::var("PERCEPTIONLAB_API_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "perception_api listening");

    let (
        dataset_repository,
        sample_repository,
        annotation_repository,
        dataset_version_repository,
        training_job_repository,
        training_job_queue,
        training_metric_repository,
        model_repository,
        model_export_repository,
        inference_run_repository,
    ): (
        Arc<dyn perception_app::DatasetRepository>,
        Arc<dyn perception_app::SampleRepository>,
        Arc<dyn perception_app::AnnotationRepository>,
        Arc<dyn perception_app::DatasetVersionRepository>,
        Arc<dyn perception_app::TrainingJobRepository>,
        Arc<dyn perception_app::TrainingJobQueue>,
        Arc<dyn perception_app::TrainingMetricRepository>,
        Arc<dyn perception_app::ModelRepository>,
        Arc<dyn perception_app::ModelExportRepository>,
        Arc<dyn perception_app::InferenceRunRepository>,
    ) = match perception_infra::RepositoryBackend::from_env() {
        perception_infra::RepositoryBackend::Postgres => {
            tracing::info!(
                "using postgres dataset, sample, annotation, dataset version, training job, training queue, training metric, model, model export, and inference run repositories"
            );
            let database_url = std::env::var("PERCEPTIONLAB_DATABASE_URL")?;
            let database_pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await?;
            let project_root =
                std::env::var("PERCEPTIONLAB_PROJECT_ROOT").unwrap_or_else(|_| ".".to_owned());
            let migrations_root = std::env::var("PERCEPTIONLAB_MIGRATIONS_ROOT")
                .unwrap_or_else(|_| format!("{project_root}/api/migrations"));
            let migrator =
                sqlx::migrate::Migrator::new(std::path::Path::new(&migrations_root)).await?;
            migrator.run(&database_pool).await?;
            (
                Arc::new(perception_infra::PostgresDatasetRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresSampleRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresAnnotationRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresDatasetVersionRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresTrainingJobRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresTrainingJobQueue::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresTrainingMetricRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresModelRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresModelExportRepository::new(
                    database_pool.clone(),
                )),
                Arc::new(perception_infra::PostgresInferenceRunRepository::new(
                    database_pool,
                )),
            )
        }
        perception_infra::RepositoryBackend::Transient => (
            Arc::new(perception_infra::TransientDatasetRepository::default()),
            Arc::new(perception_infra::TransientSampleRepository::default()),
            Arc::new(perception_infra::TransientAnnotationRepository::default()),
            Arc::new(perception_infra::TransientDatasetVersionRepository::default()),
            Arc::new(perception_infra::TransientTrainingJobRepository::default()),
            Arc::new(perception_infra::TransientTrainingJobQueue::default()),
            Arc::new(perception_infra::TransientTrainingMetricRepository::default()),
            Arc::new(perception_infra::TransientModelRepository::default()),
            Arc::new(perception_infra::TransientModelExportRepository::default()),
            Arc::new(perception_infra::TransientInferenceRunRepository::default()),
        ),
    };
    let artifact_root = std::env::var("PERCEPTIONLAB_ARTIFACT_ROOT")
        .unwrap_or_else(|_| ".perceptionlab/artifacts".to_owned());
    let overlay_renderer = Arc::new(perception_infra::SvgOverlayRenderer::new(format!(
        "{artifact_root}/overlays"
    )));
    let inference_engine: Arc<dyn perception_app::InferenceEngine> =
        match std::env::var("PERCEPTIONLAB_INFERENCE_ENGINE").as_deref() {
            Ok("yolo_cli") | Ok("yolo") => {
                tracing::info!("using yolo cli inference engine");
                Arc::new(perception_infra::YoloCliInferenceEngine::from_env())
            }
            _ => Arc::new(perception_infra::FakeInferenceEngine),
        };
    let storage_root = std::env::var("PERCEPTIONLAB_STORAGE_ROOT")
        .unwrap_or_else(|_| ".perceptionlab/storage".to_owned());
    let sample_storage = Arc::new(perception_infra::LocalSampleStorage::new(storage_root));
    let api_key_auth_config = perception_http::ApiKeyAuthConfig::from_env();
    if api_key_auth_config.is_enabled() {
        tracing::info!("api key auth enabled");
    }

    let app = perception_http::router_with_p0_ports(
        dataset_repository,
        sample_repository,
        sample_storage,
        annotation_repository,
        dataset_version_repository,
        training_job_repository,
        training_job_queue,
        training_metric_repository,
        model_repository,
        model_export_repository,
        inference_run_repository,
        overlay_renderer,
        inference_engine,
    );
    let app = perception_http::with_optional_api_key_auth(app, api_key_auth_config);

    axum::serve(listener, app).await?;

    Ok(())
}
