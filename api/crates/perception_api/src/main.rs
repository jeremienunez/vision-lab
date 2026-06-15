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

    let dataset_repository = Arc::new(perception_infra::TransientDatasetRepository::default());
    let sample_repository = Arc::new(perception_infra::TransientSampleRepository::default());
    let annotation_repository =
        Arc::new(perception_infra::TransientAnnotationRepository::default());
    let dataset_version_repository =
        Arc::new(perception_infra::TransientDatasetVersionRepository::default());
    let training_job_repository =
        Arc::new(perception_infra::TransientTrainingJobRepository::default());
    let storage_root = std::env::var("PERCEPTIONLAB_STORAGE_ROOT")
        .unwrap_or_else(|_| ".perceptionlab/storage".to_owned());
    let sample_storage = Arc::new(perception_infra::LocalSampleStorage::new(storage_root));

    axum::serve(
        listener,
        perception_http::router_with_p0_ports(
            dataset_repository,
            sample_repository,
            sample_storage,
            annotation_repository,
            dataset_version_repository,
            training_job_repository,
        ),
    )
    .await?;

    Ok(())
}
