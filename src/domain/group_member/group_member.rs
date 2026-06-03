use std::slice::SliceIndex;

use crate::domain::group_member::{MemberRole, MemberStatus};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug)]
pub struct GroupMember {
    user_id: Uuid,

    nick_name: String,
    role: MemberRole,
    status: MemberStatus,
    joined_time: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupMemberError {
    #[error("you don't have permission to perform this action.")]
    Forbidden,
}

impl GroupMember {
    pub fn new(user_id: Uuid, nick_name: String, role: MemberRole) -> GroupMember {
        GroupMember {
            user_id,
            nick_name,
            role,
            status: MemberStatus::Normal,
            joined_time: chrono::Utc::now(),
        }
    }
}

impl GroupMember {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn role(&self) -> MemberRole {
        self.role
    }

    pub fn status(&self) -> MemberStatus {
        self.status
    }

    pub fn joined_time(&self) -> chrono::DateTime<Utc> {
        self.joined_time
    }

    pub fn can_remove_member(&self) -> bool {
        self.role() == MemberRole::Administrator
    }
}

// TODO Actions
impl GroupMember {
    pub fn change_nick_name(&mut self, value: String) {
        self.nick_name = value;
    }

    pub(crate) fn apply_mute(&mut self) {
        self.status = self.status.mute();
    }

    pub(crate) fn apply_normal(&mut self) {
        self.status = self.status.recover()
    }

    pub(crate) fn upgrade_to_manager(&mut self) -> Result<(), &'static str> {
        self.role = self.role.upgrade_to_manager()?;

        Ok(())
    }
}
