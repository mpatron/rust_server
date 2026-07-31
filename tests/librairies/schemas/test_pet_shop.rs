#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use env_logger::Env;
    use rust_server::controllers::ctrl_customer::customer_handler;
    //use super::*;
    use log::{error, info};
    use rust_server::librairies::schemas::pet_shop::Customer;
    use serde_json::json;
    use validator::Validate;

    fn init_test_logger() {
        let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[actix_web::test]
    async fn test_customer_new_validation() {
        init_test_logger();
        info!("Running test_customer_new_validation");
        // Add your test assertions here

        let signup_data = Customer {
            first_name: "Doe".to_string(),
            last_name: "John".to_string(),
            email: "jdoe@example.com".to_string(),
            age: 11,
            date_of_birth: chrono::Utc::now(),
            id: uuid::Uuid::now_v7(),
            encryption: "AES".to_string(),
            password: "password123".to_string(),
            hidden: Some(false),
            home_address: vec![],
        };

        let signup_json = json!(signup_data);
        error!(
            "Validating signup_data like this: {}",
            signup_json.to_string()
        );

        match signup_data.validate() {
            Ok(_) => info!("Validation passed"),
            Err(e) => {
                error!("Validation failed: {:?}", e);
                assert!(false);
            }
        }

        info!("test_customer_new_validation passed");
    }

    #[actix_web::test]
    async fn test_integration() {
        init_test_logger();
        info!("Running integration test");
        // Integration test example
        assert!(true); // Replace with actual integration test
    }

    #[actix_web::test]
    async fn test_index_get() {
        init_test_logger();
        let app = test::init_service(App::new().service(customer_handler)).await;
        let req = test::TestRequest::get()
            .uri("/customer?id=00000000-0000-0000-0000-000000000000&firstName=John&lastName=Doe&email=jdoe%40example.com&age=19&dateOfBirth=1710000000&password=password123&encryption=AES&hidden=false")
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("=>{}", resp.status().as_str());
        assert!(resp.status().is_success());
    }
    /*

       #[actix_web::test]
       async fn test_index_post() {
           let app = test::init_service(App::new().service(generate_customer)).await;
           let req = test::TestRequest::post().uri("/").to_request();
           let resp = test::call_service(&app, req).await;
           assert!(resp.status().is_client_error());
       }
    */
}
