#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
    #[default]
    Shh,
}

impl Gender {
    pub fn male(self) -> Result<Self, &'static str> {
        if self == Self::Male {
            return Err("nothing has changed because it is the same as before");
        }

        Ok(Self::Male)
    }

    pub fn famale(self) -> Result<Self, &'static str> {
        if matches!(self, Self::Female) {
            return Err("nothing has changed because it is the same as before");
        }

        Ok(Self::Female)
    }

    pub fn shh(self) -> Result<Self, &'static str> {
        if matches!(self, Self::Shh) {
            return Err("nothing has changed because it is the same as before");
        }

        Ok(Self::Shh)
    }

    pub fn to_text(self) -> &'static str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Shh => "private",
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Female => 'F',
            Self::Male => 'M',
            Self::Shh => 'P',
        }
    }
}
