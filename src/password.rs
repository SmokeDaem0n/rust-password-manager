use std::fmt;

pub struct PasswordData {
    pub website: String,
    pub username: String,
    pub password: String,
}

impl PasswordData {
    pub fn new(website: String, username: String, password: String) -> Self {
        Self {
            website,
            username,
            password,
        }
    }
}

impl std::fmt::Display for PasswordData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Website: {}, User: {}, Password: {}",
            self.website, self.username, self.password
        )
    }
}
