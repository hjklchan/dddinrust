use uuid::Uuid;

#[derive(Debug, Eq, Clone, Copy)]
pub struct GroupMemberId {
    value: Uuid,
}

impl GroupMemberId {
    pub fn new(value: Uuid) -> Self {
        Self { value: value }
    }

    pub fn generate() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for GroupMemberId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl PartialEq for GroupMemberId {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
