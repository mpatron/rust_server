use rust_server::hello::hello;
use log::info;

#[test]
fn test_hello_world_integration() {
    info!("Running test intégration test_hello_world_integration");
    assert_eq!(hello(), "Hello, World! 🤣");
    info!("test_hello_world_integration passed");
}
