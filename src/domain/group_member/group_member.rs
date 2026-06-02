use chrono::Utc;
use uuid::Uuid;

pub struct GroupMember {
    user_id: Uuid,

    nick_name: String,
    // TODO shoule bind a type named ::GroupRole::
    role: String,
    status: String,
    joined_time: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupMemberError {
    //
}

impl GroupMember {
    pub fn new(
        user_id: Uuid,
        nick_name: String,
        role: String,
        status: String,
    ) -> Result<GroupMember, GroupMemberError> {
        Ok(GroupMember {
            user_id,
            nick_name,
            role,
            status,
            joined_time: chrono::Utc::now(),
        })
    }
}

impl GroupMember {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn joined_time(&self) -> chrono::DateTime<Utc> {
        self.joined_time
    }
}

// TODO Actions
impl GroupMember {}
