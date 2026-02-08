#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn new() -> Self {
        User {
            id: "userId".into(),
        }
    }
}
