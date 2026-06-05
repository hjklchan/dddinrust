use chrono::Utc;

use crate::domain::{
    group::group_id::{self, GroupId},
    group_member::{
        group_member_id::GroupMemberId,
        group_role::{self, GroupRole},
    },
    user::UserId,
};

#[derive(Debug)]
pub struct GroupMember {
    id: GroupMemberId,
    group_id: GroupId,
    user_id: UserId,

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
}

impl GroupMember {
    fn new(
        id: GroupMemberId,
        group_id: GroupId,
        user_id: UserId,
        group_alias: String,
        role: GroupRole,
        is_muted: bool,
        joined_at: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            group_id,
            user_id,
            group_alias,
            role,
            is_muted,
            joined_at,
        }
    }

    pub fn owner_in_group(group_id: GroupId, user_id: UserId, mut group_alias: String) -> Self {
        let id = GroupMemberId::generate();
        let is_muted = false;
        let joined_at = Utc::now();
        let role = GroupRole::Owner;
        // group_alias = group_alias.trim();

        Self::new(
            id,
            group_id,
            user_id,
            group_alias,
            role,
            is_muted,
            joined_at,
        )
    }

    // 加入组
    pub fn join_in_group(group_id: GroupId, user_id: UserId, group_alias: String) -> Self {
        let id = GroupMemberId::generate();
        let is_muted = false;
        let joined_at = Utc::now();
        let role = GroupRole::Member;

        Self {
            id,
            group_id,
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

    pub fn group_member_id(&self) -> GroupMemberId {
        self.id
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
}
