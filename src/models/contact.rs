pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
}

impl Contact {
    pub fn new(first_name: String, last_name: String, phone: String, email: String) -> Self {
        Self {
            first_name,
            last_name,
            phone,
            email,
        }
    }
}
