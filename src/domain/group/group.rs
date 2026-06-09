use crate::domain::{group_member::GroupMember, shared::id::Id};
use chrono::Utc;
use std::collections::HashMap;

pub struct Group {
    group_id: Id,

    name: String,
    description: Option<String>,
    created_at: chrono::DateTime<Utc>,

    num_of_group: i32,
    members: HashMap<Id, GroupMember>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupDomainError {
    #[error("group name can not be empty")]
    EmptyGroupName,
    #[error("can not join in this group, reason: {0}")]
    UnableToJoinIn(&'static str),
}

impl Group {
    fn new(
        group_id: Id,
        name: String,
        description: Option<String>,
        created_at: chrono::DateTime<Utc>,
        num_of_group: i32,
        members: HashMap<Id, GroupMember>,
    ) -> Self {
        Self {
            group_id,
            name,
            description,
            created_at,
            num_of_group,
            members,
        }
    }

    // create a new group and a member owns this group
    pub fn create(
        creator_id: Id,
        creator_nick_name: String,
        name: String,
        description: Option<String>,
    ) -> Result<Self, GroupDomainError> {
        if name.is_empty() {
            return Err(GroupDomainError::EmptyGroupName);
        }

        let owner = GroupMember::owner_in_group(creator_id, creator_nick_name);

        let group_id = Id::generate();
        let created_at = Utc::now();
        let num_of_group = 0;

        let mut members = HashMap::new();
        members.insert(creator_id, owner);

        Ok(Group {
            group_id,
            name,
            description,
            created_at,
            num_of_group,
            members,
        })
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn group_id(&self) -> Id {
        self.group_id
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

    pub fn change_name(&mut self, value: String) {
        self.name = value;
    }

    pub fn change_description(&mut self, value: String) {}

    // pub fn invite_user_in_group(&mut self, user_id: Id, user_nick_name: String)

    pub fn members(&self) -> &HashMap<Id, GroupMember> {
        &self.members
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::group::Group;
    use uuid::Uuid;

    #[test]
    fn test_create_a_new_group() {
        let user_id = Uuid::new_v4();
        let user_nick_name = "Lucaz";
        println!("mock user: {} - {}", user_nick_name, user_id);

        let group = Group::create(
            user_id.into(),
            user_nick_name.into(),
            "PHP Development".into(),
            None,
        )
        .unwrap();
        println!("create a new group named: {}", group.name());

        println!("group members: {:#?}", group.members());
    }
}
