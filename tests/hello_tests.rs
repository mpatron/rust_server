use log::info;
use rust_server::hello::hello;

#[test]
fn test_hello_world_integration() {
    info!("Running test intégration test_hello_world_integration");
    assert_eq!(hello(), "Hello, World! 🤣");
    info!("test_hello_world_integration passed");
}
