#![allow(unused)]

use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    group::group::GroupError::EmptyName,
    group_member::{GroupMember, MemberRole},
};

#[derive(Debug)]
pub struct Group {
    id: Uuid,
    // 群主 Id
    owner_id: Uuid,

    name: String,
    description: Option<String>,
    members: Vec<GroupMember>,

    created_time: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum GroupError {
    #[error("group name cannot be empty")]
    EmptyName,
    #[error("group member not found")]
    GroupMemberNotFound,
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("{0}")]
    DisallowOperation(String),
}

impl Group {
    pub fn new(
        owner_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<Group, GroupError> {
        if name.is_empty() {
            return Err(EmptyName);
        }

        let now = Utc::now();

        let mut new_group = Group {
            id: Uuid::new_v4(),
            owner_id,
            name,
            description,
            members: Default::default(),
            created_time: now,
        };
        // 默认添加房主 Member
        let administrator = GroupMember::new(
            owner_id,
            "Host".to_string(),
            crate::domain::group_member::MemberRole::Administrator,
        );
        new_group.members.push(administrator);

        Ok(new_group)
    }
}

// Actions
impl Group {
    pub fn add_member(&mut self, member: GroupMember) -> Result<(), ()> {
        self.members.push(member);

        Ok(())
    }

    pub fn invite_member(&mut self, member: GroupMember) -> Result<(), ()> {
        // TODO - 任意角色的群成员都可以邀请人
        // TODO - 只有群主邀请（当前）
        self.members.push(member);
        // TODO 记录日志

        Ok(())
    }

    pub fn kick_member(&mut self, operator_id: Uuid, member_id: Uuid) -> Result<(), GroupError> {
        // find operator
        let operator = self.find_member(operator_id)?;

        // check role
        // only administrator can do
        if !operator.can_remove_member() {
            return Err(GroupError::Forbidden(
                "you're not group administrator".to_owned(),
            ));
        }

        // find member
        let member_idx = self
            .members
            .iter()
            .position(|gm| {
                return gm.user_id() == member_id;
            })
            .ok_or(GroupError::GroupMemberNotFound)?;

        // kick out
        self.members.remove(member_idx);

        Ok(())
    }

    pub fn mute_member(&mut self, operator_id: Uuid, member_id: Uuid) -> Result<(), GroupError> {
        // find operator
        let operator = self.find_member(operator_id)?;

        // check role
        if operator.role() != MemberRole::Administrator || operator.role() != MemberRole::Manager {
            return Err(GroupError::Forbidden(
                "you're not group administrator or group manger".to_owned(),
            ));
        }

        // find member
        let member = self.find_member(member_id)?;

        // do mute
        member.apply_mute();

        Ok(())
    }

    // TODO
    pub fn unmute_member(&mut self, operator_id: Uuid, member_id: Uuid) -> () {}

    pub fn promote_to_manager(
        &mut self,
        operator_id: Uuid,
        member_id: Uuid,
    ) -> Result<(), GroupError> {
        let operator = self.find_member(operator_id)?;

        // check role
        if !operator.role().is_administrator() {
            return Err(GroupError::Forbidden(
                "you're not administrator in group".to_owned(),
            ));
        }

        // find member
        let member = self.find_member(member_id)?;

        member
            .promote_to_manager()
            .map_err(|err| GroupError::Forbidden(err.to_string()))?;

        Ok(())
    }

    pub fn demote_to_member(
        &mut self,
        operator_id: Uuid,
        member_id: Uuid,
    ) -> Result<(), GroupError> {
        let operator = self.find_member(operator_id)?;

        // check role
        if !operator.role().is_administrator() {
            return Err(GroupError::Forbidden(
                "you're not administrator in group".to_owned(),
            ));
        }

        // find member
        let member = self.find_member(member_id)?;

        member
            .demote_to_member()
            .map_err(|err| GroupError::Forbidden(err.to_string()))?;

        Ok(())
    }

    pub fn transfer_administrator(
        &mut self,
        operator_id: Uuid,
        member_id: Uuid,
    ) -> Result<(), GroupError> {
        let operator = self.find_member(operator_id)?;

        // check role
        if !operator.role().is_administrator() {
            return Err(GroupError::Forbidden(
                "you're not administrator in group".to_owned(),
            ));
        }

        operator
            .demote_to_member()
            .map_err(|err| GroupError::Forbidden(err.to_string()))?;

        // find member
        let member = self.find_member(member_id)?;

        member
            .transfer_administrator()
            .map_err(|err| GroupError::DisallowOperation(err.into()))?;

        Ok(())
    }

    fn find_member(&mut self, member_id: Uuid) -> Result<&mut GroupMember, GroupError> {
        self.members
            .iter_mut()
            .find(|gm| gm.user_id() == member_id)
            .ok_or(GroupError::GroupMemberNotFound)
    }
}

// Getters
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

    pub fn count_of_members(&self) -> usize {
        self.members.len()
    }

    pub fn group_owner(&self) -> Result<&GroupMember, GroupError> {
        todo!()
    }
}
