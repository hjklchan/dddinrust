#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupRole {
    Owner,
    Admin,
    Member,
}

impl GroupRole {
    pub fn group_owner() -> GroupRole {
        GroupRole::Owner
    }

    fn check_and_throw_err(self, target: Self) -> Result<(), &'static str> {
        if matches!(self, target) {
            return Err("it's already been this status");
        }

        Ok(())
    }

    pub fn promote_to_admin(self) -> Result<GroupRole, &'static str> {
        self.check_and_throw_err(Self::Admin)?;

        Ok(GroupRole::Admin)
    }

    pub fn revoke_admin(self) -> Result<GroupRole, &'static str> {
        self.check_and_throw_err(GroupRole::Member)?;

        Ok(GroupRole::Member)
    }

    pub fn become_owner(self) -> Result<GroupRole, &'static str> {
        self.check_and_throw_err(GroupRole::Owner)?;

        Ok(GroupRole::Owner)
    }
}
