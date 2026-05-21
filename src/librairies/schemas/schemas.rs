use serde::{Serialize, Deserialize};
use utoipa::{ToSchema, IntoParams};

// https://github.com/agoncal/agoncal-application-petstore-ee7/tree/master/src/main/java/org/agoncal/application/petstore/model
#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct Customer {
    pub ssid: String,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub dateOfBirth: String,
    pub age: i32,
    pub encryption: String,
    pub password: String,
    pub hidden: Option<bool>,
    pub address: Option<Address>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct Address {
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub zipCode: String,
}
