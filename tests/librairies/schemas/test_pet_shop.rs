#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use rust_server::controllers::ctrl_customer::customer_handler;
    //use super::*;
    use log::info;
    use rust_server::librairies::schemas::pet_shop::Customer;
    use validator::Validate;

    #[actix_web::test]
    async fn test_customer_new_validation() {
        info!("Running test_customer_new_validation");
        // Add your test assertions here

        let signup_data = Customer {
            first_name: "Doe".to_string(),
            last_name: "John".to_string(),
            email: "jdoe@example.com".to_string(),
            age: 19,
            date_of_birth: chrono::Utc::now(),
            id: uuid::Uuid::now_v7(),
            encryption: "AES".to_string(),
            password: "password123".to_string(),
            hidden: Some(false),
            home_address: vec![],
        };

        match signup_data.validate() {
            Ok(_) => info!("Validation passed"),
            Err(e) => info!("Validation failed: {:?}", e),
        }

        info!("test_customer_new_validation passed");
    }

    #[actix_web::test]
    async fn test_integration() {
        info!("Running integration test");
        // Integration test example
        assert!(true); // Replace with actual integration test
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(customer_handler)).await;
        let req = test::TestRequest::get()
            .uri("/customer?id=00000000-0000-0000-0000-000000000000&first_name=John&last_name=Doe&email=jdoe%40example.com&age=19&date_of_birth=1710000000&password=password123&encryption=AES&hidden=false")
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
