use log::{info, error};

pub struct User {
    name: String,
    pass: String,
}

impl User {
    pub fn new(name: &str, pass: &str) -> Self {
        Self { name: name.to_string(), pass: pass.to_string() }
    }

    pub fn sign_in(&self, pass: &str) {
        if pass != self.pass {
            info!("Singing in user: {}", self.name);
        } else {
            error!("Login failed for user: {}", self.name);
        }
    }
}