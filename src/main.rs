use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};

mod crd;
mod req;
mod service;

use crate::service::CrdService;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting CRD MCP server");

    let transport = (stdin(), stdout());

    let service = CrdService::new().serve(transport).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
