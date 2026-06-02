use chrono::Utc;
use uuid::Uuid;

use crate::domain::group::group::GroupError::EmptyName;

#[derive(Debug)]
pub struct Group {
    id: Uuid,

    name: String,
    description: Option<String>,

    created_time: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("group name cannot be empty")]
    EmptyName,
}

impl Group {
    pub fn new(name: String, description: Option<String>) -> Result<Group, GroupError> {
        if name.is_empty() {
            return Err(EmptyName);
        }

        let now = Utc::now();

        Ok(Group {
            id: Uuid::new_v4(),
            name,
            description,
            created_time: now,
        })
    }
}

impl Group {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<impl AsRef<str>> {
        self.description.as_ref()
    }

    pub fn created_time(&self) -> chrono::DateTime<Utc> {
        self.created_time
    }
}
