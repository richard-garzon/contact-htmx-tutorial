use super::super::models::contact::Contact;
use std::collections::HashMap;

pub struct contactDB {
    db: HashMap<String, Contact>,
}

impl contactDB {
    pub fn new() -> Self {
        let mut contact_db = contactDB { db: HashMap::new() };

        let test_contact_1 = Contact::new(
            "Gerard".to_string(),
            "Way".to_string(),
            "123-456-7890".to_string(),
            "gerard@way.com".to_string(),
        );

        let test_contact_2 = Contact::new(
            "Tom".to_string(),
            "Verlane".to_string(),
            "987-654-3210".to_string(),
            "tele@vision.com".to_string(),
        );

        contact_db
            .db
            .insert(test_contact_1.first_name, test_contact_1);

        contact_db
    }
}
