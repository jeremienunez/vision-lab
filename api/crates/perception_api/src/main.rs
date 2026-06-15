#![forbid(unsafe_code)]

fn main() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "perception_api=info".into());

    tracing_subscriber::fmt().with_env_filter(filter).init();
    tracing::info!("perception_api bootstrap loaded");
}
