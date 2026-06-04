use std::ops::Sub;

use chrono::{Datelike, TimeDelta, Utc};

#[derive(Debug, Clone, Copy)]
pub struct DateOfBirth {
    value: chrono::DateTime<Utc>,
}

impl DateOfBirth {
    pub fn new(value: chrono::DateTime<Utc>) -> Self {
        Self { value }
    }
}

impl From<chrono::DateTime<Utc>> for DateOfBirth {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self::new(value)
    }
}

impl DateOfBirth {
    pub fn is_adult(&self) -> bool {
        let now = Utc::now();
        let mut years = now.sub(self.value).num_days() / 365;

        println!("{} {}", now.month(), self.value.month());
        if now.day() < self.value.day() {
            years -= 1;
        }

        years >= 18
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::domain::user::date_of_birth::DateOfBirth;

    #[test]
    fn test_is_adult() {
        let dt = Utc.with_ymd_and_hms(1999, 6, 5, 0, 0, 0).unwrap();
        let date_of_birth = DateOfBirth::from(dt);

        date_of_birth.is_adult();
    }
}
