use crate::librairies::schemas::pet_shop::Customer;

pub fn generate_customer(query: &Customer) -> String {
    // Here you would implement the logic to generate a customer based on the query parameters.
    // For demonstration purposes, we'll return a simple JSON string.

    format!(
        r#"{{
            "id": "{}",
            "first_name": "{}",
            "last_name": "{}",
            "email": "{}",
            "age": {},
            "date_of_birth": "{}",
            "encryption": "{}",
            "hidden": {},
            "home_address": []
        }}"#,
        query.id,
        query.first_name,
        query.last_name,
        query.email,
        query.age,
        query.date_of_birth,
        query.encryption,
        query.hidden.unwrap_or(false)
    )
}
