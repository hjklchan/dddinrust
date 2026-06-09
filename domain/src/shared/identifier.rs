use std::str::FromStr;
use uuid::{Error as UuidError, Uuid};

#[derive(Debug, Eq, Clone, Copy, Hash)]
pub struct Identifier(Uuid);

#[derive(Debug, thiserror::Error)]
pub enum IdentifierError {}

impl Identifier {
    fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn generate_v4() -> Self {
        Self::new(Uuid::new_v4())
    }
}

impl std::cmp::PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl TryFrom<String> for Identifier {
    type Error = UuidError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Identifier(Uuid::from_str(&value)?))
    }
}
