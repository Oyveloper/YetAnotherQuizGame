pub struct User {
    pub id: String,
    password: String,
    pub email: String,
}

impl User {
    pub fn new(password: String, email: String) -> User {
        let user = User {
            id: Uuid::new_v4().to_string(),
            password: password,
            email: email,
        }

        // Save the user to the database
        // ...

        user
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_password(&mut self, password: String) -> &String {
        &self.password = password;
    }
}
