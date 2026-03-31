use axum::{Router, response::IntoResponse, routing::get};
use tracing::info;
// use apiresponse::{ApiResponse, ApiError};

async fn hello_world() -> &'static str {
    info!("Running hello_world({})", "Hello, World! 🤣");
    "Hello, World! 🤣"
}

async fn health_check() -> impl IntoResponse {
    "✅ Healthy"
}

pub fn hello_routers() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_hello_world() {
        info!("Running unit test_hello_world");
        let response = hello_world().await;
        assert_eq!(response, "Hello, World! 🤣");
    }
}
