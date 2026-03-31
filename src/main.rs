mod hello;
mod upload;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_logs() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    init_logs();

    let version = "APP_VERSION";
    match env::var(version) {
        Ok(val) => println!("{version}: {val:?}"),
        Err(e) => println!("La varible {version} est vide: {e}"),
    }

    info!("🚀 Server starting...");
    let app = Router::new()
        .merge(hello::hello_routers())
        .merge(upload::upload_routers());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("🧱 Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("🦀 Server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use tracing::info;

    #[tokio::test]
    async fn test_main() {
        info!("Running unit test_main");
        assert_eq!(1, 1);
    }
}
