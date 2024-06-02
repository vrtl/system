use std::path::PathBuf;

use axum::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tower_http::services::ServeDir;

mod error;

use error::Result;

fn setup_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    
    Ok(())
}

async fn setup_server() -> Result<()> {
    let dist = ServeDir::new(PathBuf::from("dist"));

    let app = Router::new()
        .nest_service("/", dist);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing()?;
    setup_server().await?;
    Ok(())
}
