mod hello;

use axum::Router;
use rust_server::{hello::hello_routers, upload::upload_routers};
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
async fn main() {
    init_logs();

    let version = "APP_VERSION";
    match env::var(version) {
        Ok(val) => println!("{version}: {val:?}"),
        Err(e) => println!("La varible {version} est vide: {e}"),
    }

    info!("🚀 Server starting...");
    let app = Router::new().merge(hello_routers()).merge(upload_routers());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("🧱 Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    info!("🦀 Server stopped");
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
