#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberRole {
    Member,
    Manager,
    Administrator,
}

impl MemberRole {
    pub fn is_member(&self) -> bool {
        matches!(self, MemberRole::Member)
    }

    pub fn is_manager(&self) -> bool {
        matches!(self, MemberRole::Manager)
    }

    pub fn is_administrator(&self) -> bool {
        matches!(self, MemberRole::Administrator)
    }

    pub fn change_to_manager(&self) -> Result<MemberRole, &'static str> {
        if self.is_member() {
            return Ok(MemberRole::Manager);
        }

        Err("only member can be changed to manager")
    }

    pub fn change_to_member(&self) -> Result<MemberRole, &'static str> {
        if self.is_manager() {
            return Ok(MemberRole::Member);
        }

        Err("only manager can be changed to member")
    }

    pub fn change_to_administrator(&self) -> Result<MemberRole, &'static str> {
        if !self.is_administrator() {
            return Ok(MemberRole::Administrator);
        }

        Err(
            "only member who are not the group administrator can be changed to the group administrator",
        )
    }
}
