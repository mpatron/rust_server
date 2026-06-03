use log::info;
use rust_server::librairies::schemas::pet_shop::{Customer, Validate};

#[test]
fn test_customer_new_validation() {
    info!("Running test intégration test_customer_new_validation");
    // Add your test assertions here

    let signup_data = Customer {
        firstName: "Doe".to_string(),
        lastName: "John".to_string(),
        email: "jdoe@example.com".to_string(),
        age: 19,
    };

    match signup_data.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation failed: {:?}", e),
    }

    info!("test_customer_new_validation passed");
}
