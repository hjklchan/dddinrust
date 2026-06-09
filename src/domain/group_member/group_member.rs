use crate::domain::{group_member::group_role::GroupRole, shared::id::Id};
use chrono::Utc;

#[derive(Debug)]
pub struct GroupMember {
    group_member_id: Id,
    // group_id: Id,
    user_id: Id,

    role: GroupRole,
    group_alias: String,
    is_muted: bool,
    joined_at: chrono::DateTime<Utc>,
    //
    // more...
}

#[derive(Debug, thiserror::Error)]
pub enum GroupMemberDomainError {
    #[error("group alias can not be empty")]
    EmptyGroupAlias,

    #[error("has already been muted")]
    HasAlreadyBeenMuted,

    #[error("this member has been muted")]
    HasBeenMuted,
}

impl GroupMember {
    fn new(
        group_member_id: Id,
        // group_id: Id,
        user_id: Id,
        group_alias: String,
        role: GroupRole,
        is_muted: bool,
        joined_at: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            group_member_id: group_member_id,
            // group_id,
            user_id,
            group_alias,
            role,
            is_muted,
            joined_at,
        }
    }

    pub fn owner_in_group(user_id: Id, mut group_alias: String) -> Self {
        let group_member_id = Id::generate();
        let is_muted = false;
        let joined_at = Utc::now();
        let role = GroupRole::Owner;
        // group_alias = group_alias.trim();

        Self::new(
            group_member_id,
            user_id,
            group_alias,
            role,
            is_muted,
            joined_at,
        )
    }

    // 加入组
    pub fn invite(group_id: Id, user_id: Id, group_alias: String) -> Self {
        let group_member_id = Id::generate();
        let is_muted = false;
        let joined_at = Utc::now();
        let role = GroupRole::Member;

        Self {
            group_member_id,
            user_id,
            group_alias,
            role,
            is_muted,
            joined_at,
        }
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn group_alias<'a>(&'a self) -> &'a str {
        &self.group_alias
    }

    pub fn joined_time(&self) -> chrono::DateTime<Utc> {
        self.joined_at
    }

    pub fn group_member_id(&self) -> Id {
        self.group_member_id
    }

    pub fn change_group_alias(&mut self, value: String) -> Result<(), GroupMemberDomainError> {
        if value.len() == 0 {
            return Err(GroupMemberDomainError::EmptyGroupAlias);
        }

        self.group_alias = value;

        Ok(())
    }

    pub fn apply_mute(&mut self) {
        self.is_muted = true;
    }

    pub fn apply_unmute(&mut self) {
        self.is_muted = false;
    }

    // 发送消息
    pub fn send_message(&self, _to: &str, _message: &str) -> Result<(), GroupMemberDomainError> {
        if self.is_muted() {
            return Err(GroupMemberDomainError::HasBeenMuted);
        }

        println!("send message {} to {}", _message, _to);

        Ok(())
    }
}
