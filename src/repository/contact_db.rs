use super::super::models::contact::Contact;
use std::collections::HashMap;

pub struct ContactDB {
    db: HashMap<u32, Contact>,
}

impl ContactDB {
    pub fn new(count: usize) -> Self {
        let mut contact_db = ContactDB { db: HashMap::new() };

        for i in 1..=count {
            let contact = Contact::new(
                i as u32,
                format!("FirstName{}", i),
                format!("LastName{}", i),
                format!("555-555-{:04}", i),
                format!("user{}@example.com", i),
                HashMap::new(),
            );
            contact_db.db.insert(contact.id, contact);
        }

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

    pub fn email_exists(&self, email: &String) -> bool {
        let mut exists = false;
        for c in self.db.values() {
            if *email == c.email {
                exists = true;
            }
        }

        exists
    }

    pub fn find(&self, id: u32) -> Option<&Contact> {
        self.db.get(&id)
    }

    pub fn delete(&mut self, id: u32) {
        self.db.remove(&id);
    }

    pub fn save(&mut self, c: Contact) {
        self.db.insert(c.id, c);
    }
}
