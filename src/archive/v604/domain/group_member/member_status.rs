#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberStatus {
    Normal,
    Mute,
}

impl MemberStatus {
    pub fn is_mute(&self) -> bool {
        *self == MemberStatus::Mute
    }

    pub fn is_normal(&self) -> bool {
        *self == MemberStatus::Normal
    }

    pub fn mute(&self) -> MemberStatus {
        MemberStatus::Mute
    }

    pub fn recover(&self) -> MemberStatus {
        MemberStatus::Normal
    }
}
