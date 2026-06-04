#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    #[default]
    Verification,
    Active,
    Suspended,
    Disable,
}

impl Status {
    #[inline]
    fn check(self, other: Self) -> Result<(), &'static str> {
        if matches!(self, other) {
            return Err("it's already been in this status");
        }

        Ok(())
    }

    pub fn active(self) -> Result<Self, &'static str> {
        self.check(Self::Active)?;
        Ok(Self::Active)
    }

    pub fn suspended(self) -> Result<Self, &'static str> {
        self.check(Self::Suspended)?;
        Ok(Self::Suspended)
    }

    pub fn disable(self) -> Result<Self, &'static str> {
        self.check(Self::Disable)?;
        Ok(Self::Disable)
    }

    pub fn to_chinese_text(self) -> &'static str {
        match self {
            Self::Verification => "未验证",
            Self::Disable => "已禁用",
            Self::Suspended => "暂停",
            Self::Active => "正常",
        }
    }
}
