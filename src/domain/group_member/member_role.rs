#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberRole {
    Member,
    Manager,
    Administrator,
}

impl MemberRole {
    pub fn is_member(&self) -> bool {
        *self == MemberRole::Member
    }
    pub fn is_manager(&self) -> bool {
        *self == MemberRole::Manager
    }
    pub fn is_administrator(&self) -> bool {
        *self == MemberRole::Administrator
    }
    pub fn upgrade_to_manager(&self) -> Result<MemberRole, &'static str> {
        if self.is_member() {
            return Ok(MemberRole::Manager);
        }

        Err("target is not a member")
    }
    pub fn downgrade_to_member(&self) -> Result<MemberRole, &'static str> {
        if self.is_manager() {
            return Ok(MemberRole::Member);
        }

        Err("target is not a manager")
    }
}
