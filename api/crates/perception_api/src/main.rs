#![forbid(unsafe_code)]

use std::net::SocketAddr;

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

    axum::serve(listener, perception_http::router()).await?;

    Ok(())
}
