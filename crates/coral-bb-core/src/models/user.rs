use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password_hash: String,
}
