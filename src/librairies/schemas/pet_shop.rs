use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use chrono::{DateTime, Utc};

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
    pub zipCode: String,
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
    pub isoCode: String,
    #[validate(length(min = 2, max = 80))]
    pub name: String,
    #[validate(length(min = 2, max = 80))]
    pub printableName: String,
    #[validate(length(min = 3, max = 3))]
    pub iso3Code: String,
    #[validate(length(min = 3, max = 3))]
    pub numCode: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct CreditCard {
    #[validate(length(min = 1, max = 30))]
    pub creditCardNumber: String,
    pub creditCardType: CreditCardType,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creditCardExpDate: DateTime<Utc>,
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
    pub firstName: String,
    #[validate(length(min = 1, max = 50))]
    pub lastName: String,
    #[validate(email)]
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub dateOfBirth: DateTime<Utc>,
    #[validate(range(min = 0, max = 100))]
    pub age: i32,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
    pub encryption: String,
    pub hidden: Option<bool>,
    pub homeAddress: Vec<Option<Address>>,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Item {
    pub id: i32,
    #[validate(length(min = 1, max = 30))]
    pub name: String,
    #[validate(length(min = 1, max = 3000))]
    pub description: String,
    #[validate(length(min = 1, max = 512))]
    pub imagePath: String,
    #[validate(range(min = 0.0, max = 1000.0))]
    pub unitCost: f64,
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
    pub orderDate: DateTime<Utc>,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub totalWithoutVat: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub vat: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub totalWithVat: f64,
    #[validate(range(min = 0.0, max = 100.0))]
    pub discountRate: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub discount: f64,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub total: f64,
    pub customer: Customer,
    pub deliveryAddress: Address,
    pub billingAddress: Option<Address>,
    pub creditCard: CreditCard,
    pub orderLines: Vec<OrderLine>,
}
