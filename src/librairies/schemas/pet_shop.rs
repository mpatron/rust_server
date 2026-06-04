use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

// https://github.com/agoncal/agoncal-application-petstore-ee7/tree/master/src/main/java/org/agoncal/application/petstore/model

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Address {
    #[validate(length(min = 5, max = 50))]
    pub street1: String,
    pub street2: Option<String>,
    #[validate(length(min = 2, max = 50))]
    pub city: String,
    pub state: Option<String>,
    #[validate(length(min = 1, max = 10))]
    pub zip_code: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Category {
    pub id: i32,
    #[validate(length(min = 1, max = 30))]
    pub name: String,
    #[validate(length(min = 1, max = 3000))]
    pub description: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Country {
    pub id: i32,
    #[validate(length(min = 2, max = 2))]
    pub iso_code: String,
    #[validate(length(min = 2, max = 80))]
    pub name: String,
    #[validate(length(min = 2, max = 80))]
    pub printable_name: String,
    #[validate(length(min = 3, max = 3))]
    pub iso3_code: String,
    #[validate(length(min = 3, max = 3))]
    pub num_code: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreditCard {
    #[validate(length(min = 1, max = 30))]
    pub credit_card_number: String,
    pub credit_card_type: CreditCardType,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub credit_card_exp_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum CreditCardType {
    Visa,
    MasterCard,
    AmericanExpress,
    Discover,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Customer {
    pub ssid: String,
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_of_birth: DateTime<Utc>,
    #[validate(range(min = 0, max = 100))]
    pub age: i32,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
    pub encryption: String,
    pub hidden: Option<bool>,
    pub home_address: Vec<Option<Address>>,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Item {
    pub id: i32,
    #[validate(length(min = 1, max = 30))]
    pub name: String,
    #[validate(length(min = 1, max = 3000))]
    pub description: String,
    #[validate(length(min = 1, max = 512))]
    pub image_path: String,
    #[validate(range(min = 0.0, max = 1000.0))]
    pub unit_cost: f64,
    pub category: Category,
    pub product: Product,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct OrderLine {
    pub id: i32,
    #[validate(range(min = 1, max = 20))]
    pub quantity: i32,
    pub item: Item,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Product {
    pub id: i32,
    #[validate(length(min = 1, max = 30))]
    pub name: String,
    #[validate(length(min = 1, max = 3000))]
    pub description: String,
    pub category: Category,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct PurchaseOrder {
    pub id: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub order_date: DateTime<Utc>,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub total_without_vat: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub vat: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub total_with_vat: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub discount_rate: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub discount: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub total: f64,
    pub customer: Customer,
    pub delivery_address: Address,
    pub billing_address: Option<Address>,
    pub credit_card: CreditCard,
    pub order_lines: Vec<OrderLine>,
}
