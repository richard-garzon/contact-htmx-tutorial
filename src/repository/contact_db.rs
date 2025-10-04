use super::super::models::contact::Contact;
use std::collections::HashMap;

pub struct ContactDB {
    db: HashMap<u32, Contact>,
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

        contact_db.db.insert(test_contact_1.id, test_contact_1);

        contact_db.db.insert(test_contact_2.id, test_contact_2);

        contact_db
    }

    pub fn update(&mut self, id: u32, other_contact: &Contact) -> bool {
        if let Some(c) = self.db.get_mut(&id) {
            c.update(other_contact);

            true
        } else {
            false
        }
    }

    pub fn all(&self) -> Vec<&Contact> {
        self.db.values().collect()
    }

    pub fn search(&self, query: String) -> Vec<&Contact> {
        let mut result: Vec<&Contact> = vec![];
        for c in self.db.values() {
            let match_first_name = c.first_name.contains(&query);
            let match_last_name = c.last_name.contains(&query);
            let match_email = c.email.contains(&query);
            let match_phone = c.phone.contains(&query);

            if match_first_name || match_last_name || match_email || match_phone {
                result.push(c);
            }
        }

        result
    }

    pub fn find(&self, id: u32) -> Option<&Contact> {
        self.db.get(&id)
    }

    pub fn save(&mut self, c: Contact) {
        self.db.insert(c.id, c);
    }
}
