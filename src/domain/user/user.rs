use chrono::Utc;
use uuid::Uuid;

use crate::domain::user::user::UserError::EmptyName;

#[derive(Debug, Clone, Copy, Default)]
pub enum Gender {
    Famale,
    Male,
    #[default]
    Mystery,
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("field name cannot be empty")]
    EmptyName,
    #[error("date time cannot be zero")]
    ZeroDateTime,
}

#[derive(Debug)]
pub struct User {
    id: Uuid,

    name: String,
    gender: Gender,
    date_of_birth: Option<chrono::DateTime<Utc>>,

    is_suspended: bool,
}

impl User {
    pub fn new(
        name: String,
        gender: Gender,
        date_of_birth: Option<chrono::DateTime<Utc>>,
    ) -> Result<User, UserError> {
        if name.is_empty() {
            return Err(EmptyName);
        }

        Ok(User {
            id: Uuid::new_v4(),
            name,
            gender,
            date_of_birth,
            is_suspended: false,
        })
    }
}

// Attribute getters
impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn date_of_birth(&self) -> Option<chrono::DateTime<Utc>> {
        self.date_of_birth
    }

    pub fn is_suspended(&self) -> bool {
        self.is_suspended
    }
}

// Actions
impl User {
    pub fn suspend(&mut self) {
        self.is_suspended = true
    }
}
