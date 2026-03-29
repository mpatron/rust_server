use axum_server::Server;
use rust_server::app;
use std::env;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// https://oneuptime.com/blog/post/2026-02-06-opentelemetry-tracing-rust-tracing-crate/view

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

    let app = app();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    info!("🧱 Listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

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
