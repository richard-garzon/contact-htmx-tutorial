use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub errors: HashMap<String, String>,
}

impl Contact {
    pub fn new(
        id: i32,
        first_name: String,
        last_name: String,
        phone: String,
        email: String,
        errors: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            phone,
            email,
            errors,
        }
    }
}
