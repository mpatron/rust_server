use serde::{Serialize, Deserialize};
use serde_valid::Validate;
use utoipa::{ToSchema, IntoParams};

// https://github.com/agoncal/agoncal-application-petstore-ee7/tree/master/src/main/java/org/agoncal/application/petstore/model

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Address {
    #[validate(min_length = 5)]
    #[validate(max_length = 50)]
    pub street1: String,
    pub street2: Option<String>,
    #[validate(min_length = 2)]
    #[validate(max_length = 50)]
    pub city: String,
    pub state: Option<String>,
    #[validate(min_length = 1)]
    #[validate(max_length = 10)]
    pub zipCode: String,
    pub country: Country,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Category {
    pub id: i32,
    #[validate(min_length = 1)]
    #[validate(max_length = 30)]
    pub name: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 3000)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Country {
    pub id: i32,
    #[validate(min_length = 2)]
    #[validate(max_length = 2)]
    pub isoCode: String,
    #[validate(min_length = 2)]
    #[validate(max_length = 80)]
    pub name: String,
    #[validate(min_length = 2)]
    #[validate(max_length = 80)]
    pub printableName: String,
    #[validate(min_length = 3)]
    #[validate(max_length = 3)]
    pub iso3Code: String,
    #[validate(min_length = 3)]
    #[validate(max_length = 3)]
    pub numCode: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct CreditCard {
    #[validate(min_length = 1)]
    #[validate(max_length = 30)]
    pub creditCardNumber: String,
    pub creditCardType: CreditCardType,
    pub creditCardExpDate: time::Date,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub enum CreditCardType {
    Visa,
    MasterCard,
    AmericanExpress,
    Discover,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Customer {
    pub ssid: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 50)]
    pub firstName: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 50)]
    pub lastName: String,
    #[validate(pattern = r"^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$")]
    pub email: String,
    pub dateOfBirth: time::Date,
    #[validate(minimum = 0)]
    #[validate(maximum = 10)]
    pub age: i32,
    #[validate(min_length = 1)]
    #[validate(max_length = 256)]
    pub password: String,
    pub encryption: String,
    pub hidden: Option<bool>,
    pub homeAddress: Vec<Option<Address>>,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Item {
    pub id: i32,
    #[validate(min_length = 1)]
    #[validate(max_length = 30)]
    pub name: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 3000)]
    pub description: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 512)]
    pub imagePath: String,
    #[validate(minimum = 0.0)]
    pub unitCost: f64,
    pub category: Category,
    pub product: Product,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct OrderLine {
    pub id: i32,
    #[validate(minimum = 1)]
    pub quantity: i32,
    pub item: Item,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct Product {
    pub id: i32,
    #[validate(min_length = 1)]
    #[validate(max_length = 30)]
    pub name: String,
    #[validate(min_length = 1)]
    #[validate(max_length = 3000)]    
    pub description: String,
    pub category: Category,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema, IntoParams)]
pub struct PurchaseOrder {  
    pub id: i32,
    pub orderDate: time::Date,
    #[validate(minimum = 0.0)]
    pub totalWithoutVat: f64,
    #[validate(minimum = 0.0)]
    pub vat: f64,
    #[validate(minimum = 0.0)]
    pub totalWithVat: f64,
    #[validate(minimum = 0.0)]
    pub discountRate: f64,
    #[validate(minimum = 0.0)]
    pub discount: f64,
    #[validate(minimum = 0.0)]
    pub total: f64,
    pub customer: Customer,
    pub deliveryAddress: Address,
    pub billingAddress: Option<Address>,
    pub creditCard: CreditCard,
    pub orderLines: Vec<OrderLine>,
}
