use super::super::models::contact::Contact;
use std::collections::HashMap;

pub struct ContactDB {
    db: HashMap<String, Contact>,
}

impl ContactDB {
    pub fn new() -> Self {
        let mut contact_db = ContactDB { db: HashMap::new() };

        let test_contact_1 = Contact::new(
            1,
            "Gerard".to_string(),
            "Way".to_string(),
            "123-456-7890".to_string(),
            "gerard@way.com".to_string(),
            HashMap::new(),
        );

        let test_contact_2 = Contact::new(
            2,
            "Tom".to_string(),
            "Verlane".to_string(),
            "987-654-3210".to_string(),
            "tele@vision.com".to_string(),
            HashMap::new(),
        );

        contact_db
            .db
            .insert(test_contact_1.first_name.clone(), test_contact_1);

        contact_db
            .db
            .insert(test_contact_2.first_name.clone(), test_contact_2);

        contact_db
    }

    pub fn all(&self) -> Vec<&Contact> {
        self.db.values().collect()
    }

    pub fn get(&self, first_name: String) -> Option<&Contact> {
        self.db.get(&first_name)
    }

    pub fn save(&mut self, c: Contact) {
        self.db.insert(c.first_name.clone(), c);
    }
}
