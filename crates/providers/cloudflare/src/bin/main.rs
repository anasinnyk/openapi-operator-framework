use std::sync::Arc;

use cloudflare_provider::generated::client::ProviderClient;
use koof::reconciler::ControllerContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let base_url = std::env::var("PROVIDER_BASE_URL")
        .unwrap_or_else(|_| "https://api.cloudflare.com/client/v4".into());

    let kube_client = kube::Client::try_default().await?;

    let provider_client = ProviderClient::new(reqwest::Client::new(), base_url);

    let context = Arc::new(ControllerContext::new(kube_client, provider_client));

    cloudflare_provider::controllers::run_all_controllers(context).await;

    Ok(())
}
