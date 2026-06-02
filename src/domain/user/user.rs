use uuid::Uuid;

pub struct User {
    id: Uuid,

    username: String,
    role: String,
}
