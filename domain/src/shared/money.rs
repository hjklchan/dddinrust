use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Money {
    cents: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    #[error("negative amount is not allowed")]
    NegativeAmount,
}

impl Money {
    fn new(cents: i64) -> Self {
        Self { cents }
    }

    pub fn try_from_cents(cents: i64) -> Result<Self, MoneyError> {
        if cents < 0 {
            return Err(MoneyError::NegativeAmount);
        }

        Ok(Self::new(cents))
    }

    pub fn cents(&self) -> i64 {
        self.cents
    }
}
