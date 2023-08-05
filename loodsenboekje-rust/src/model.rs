use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entry {
    id: usize,
    how: String,
    when: String,
    by: User,
}

#[derive(Debug, Serialize)]
pub struct User {
    name: String,
}

