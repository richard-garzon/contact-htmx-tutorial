use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct Contact {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub errors: HashMap<String, String>,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            id: 0,
            first_name: "First Name".to_string(),
            last_name: "Last Name".to_string(),
            phone: "Phone #".to_string(),
            email: "Email".to_string(),
            errors: HashMap::new(),
        }
    }
}

impl Contact {
    pub fn new(
        id: u32,
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

    pub fn update(&mut self, other: &Contact) {
        self.first_name = other.first_name.clone();
        self.last_name = other.last_name.clone();
        self.phone = other.phone.clone();
        self.email = other.email.clone();
        self.errors = other.errors.clone();
    }

    pub fn validate(&mut self) {
        if self.email.is_empty() {
            self.errors
                .insert("email".to_string(), "Email is required".to_string());
        }
        if self.first_name.is_empty() {
            self.errors.insert(
                "first_name".to_string(),
                "First name is required".to_string(),
            );
        }
        if self.last_name.is_empty() {
            self.errors
                .insert("last_name".to_string(), "Last name is required".to_string());
        }
        if self.phone.is_empty() {
            self.errors
                .insert("phone".to_string(), "Phone number is required.".to_string());
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}
