use chrono::Utc;

use crate::domain::user::{UserId, gender::Gender, status::Status};

#[derive(Debug)]
pub struct User {
    id: UserId,

    nick_name: String,
    username: String,
    gender: Gender,

    date_of_birth: Option<chrono::DateTime<Utc>>,
    created_at: chrono::DateTime<Utc>,
    status: Status,
}

impl User {
    pub fn new(id: UserId, nick_name: String, username: String) -> Self {
        Self {
            id,
            nick_name,
            username,
            gender: Gender::Shh,
            date_of_birth: None,
            created_at: chrono::Utc::now(),
            status: Default::default(),
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn date_of_birth(&self) -> Option<chrono::DateTime<Utc>> {
        self.date_of_birth
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn update_date_of_birth(&mut self, value: chrono::DateTime<Utc>) {
        self.date_of_birth = Some(value)
    }

    pub fn update_nickname(&mut self) {
        // todo
    }
}
