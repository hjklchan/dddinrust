use chrono::Utc;

use crate::domain::group::group_id::GroupId;

pub struct Group {
    id: GroupId,

    name: String,
    description: Option<String>,
    created_at: chrono::DateTime<Utc>,

    num_of_group: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupDomainError {
    #[error("group name can not be empty")]
    EmptyGroupName,
}

impl Group {
    fn new(
        id: GroupId,
        name: String,
        description: Option<String>,
        created_at: chrono::DateTime<Utc>,
        num_of_group: i32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            created_at,
            num_of_group,
        }
    }

    pub fn create(name: String, description: Option<String>) -> Result<Self, GroupDomainError> {
        if name.is_empty() {
            return Err(GroupDomainError::EmptyGroupName);
        }

        let id = GroupId::generate();
        let created_at = Utc::now();
        let num_of_group = 0;

        Ok(Group {
            id,
            name,
            description,
            created_at,
            num_of_group,
        })
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn description<'a>(&'a self) -> Option<&'a impl AsRef<str>> {
        self.description.as_ref()
    }

    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
    }

    pub fn num_of_group(&self) -> i32 {
        self.num_of_group
    }
}
