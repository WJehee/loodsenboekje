

#[derive(Debug, Serialize)]
pub struct Entry {
    how: String,
    when: String,
    by: User,
}

#[derive(Debug, Serialize)]
pub struct User {
    name: String,
}

