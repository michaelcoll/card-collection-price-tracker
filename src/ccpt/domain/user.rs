use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

impl User {
    pub fn new(id: String, email: String, name: Option<String>) -> Self {
        Self { id, email, name }
    }

    #[cfg(test)]
    pub fn for_testing() -> Self {
        Self {
            id: "test-user-id".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
        }
    }
}

#[cfg(test)]
#[path = "user_tests.rs"]
mod tests;
