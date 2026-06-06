use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder, web};
use log;

use crate::librairies::schemas::pet_shop::Customer;
use crate::services::srv_customer::generate_customer;

/*
#[utoipa::path(
    get,
    path = "/customer",
    context_path = "/api/v1/qr",
    description = "Generate a customer by id.",
    tag = "Customers",
    params(Customer),
    responses(
        (status = 200, description = "Generated customer", body = Customer)
    )
)]
     */
#[actix_web::get("/customer")]
pub async fn customer_handler(query: web::Query<Customer>) -> impl Responder {
    let customer = generate_customer(&query);

    log::info!("Generated customer for ID: {}", query.id);

    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(customer)
}
