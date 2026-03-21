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
mod tests {
    use super::*;

    #[test]
    fn user_creation_with_full_info() {
        let user = User::new(
            "123456".to_string(),
            "test@example.com".to_string(),
            Some("Test User".to_string()),
        );

        assert_eq!(user.id, "123456");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, Some("Test User".to_string()));
    }

    #[test]
    fn user_creation_without_name() {
        let user = User::new("789".to_string(), "anonymous@example.com".to_string(), None);

        assert_eq!(user.id, "789");
        assert_eq!(user.email, "anonymous@example.com");
        assert_eq!(user.name, None);
    }

    #[test]
    fn user_clone_works_correctly() {
        let user1 = User::new(
            "456".to_string(),
            "clone@example.com".to_string(),
            Some("Cloned User".to_string()),
        );

        let user2 = user1.clone();

        assert_eq!(user1, user2);
    }
}
