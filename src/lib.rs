use axum::{Router, routing::get};
use tracing::info;

async fn hello_world() -> &'static str {
    info!("Running hello_world({})","Hello, World! 🤣");
    "Hello, World! 🤣"
}

pub fn app() -> Router {
    Router::new().route("/", get(hello_world))
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
