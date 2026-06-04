use std::ops::Sub;

use chrono::{Datelike, TimeDelta, Utc};

#[derive(Debug, Clone, Copy)]
pub struct DateOfBirth {
    value: chrono::DateTime<Utc>,
    now: chrono::DateTime<Utc>,
}

impl DateOfBirth {
    pub fn new(value: chrono::DateTime<Utc>) -> Self {
        let now = Utc::now();

        Self { value, now }
    }
}

impl From<chrono::DateTime<Utc>> for DateOfBirth {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self::new(value)
    }
}

impl DateOfBirth {
    pub fn is_adult(&self) -> bool {
        let mut years = self.now().year() - self.value.year();

        println!("{} {}", self.now().month(), self.value.month());
        // TODO check month
        if self.now().month() < self.value().month() || self.now().day() < self.value().day() {
            years -= 1;
        }
        println!("{}", years);

        years >= 18
    }

    pub fn is_birthday_today(&self) -> bool {
        return self.value().month() == self.now().month()
            && self.value().day() == self.now().day();
    }

    pub fn is_birthday_passed(&self) -> bool {
        return self.value().month() <= self.now().month() && self.value().day() < self.now().day();
    }

    // TODO
    pub fn days_util_birthday(&self) -> u32 {
        // 是否已经过了生日
        if self.is_birthday_passed() {
            //
        } else {
            //
        }

        0
    }

    pub fn value(&self) -> chrono::DateTime<Utc> {
        self.value
    }

    #[inline]
    fn now(&self) -> chrono::DateTime<Utc> {
        self.now
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::domain::user::date_of_birth::DateOfBirth;

    #[test]
    fn test_is_adult() {
        let dt = Utc.with_ymd_and_hms(1999, 7, 4, 0, 0, 0).unwrap();
        let date_of_birth = DateOfBirth::from(dt);

        date_of_birth.is_adult();
    }

    #[test]
    fn test_is_birthday_passed() {
        let dt = Utc.with_ymd_and_hms(1999, 6, 5, 0, 0, 0).unwrap();
        let date_of_birth = DateOfBirth::from(dt);

        let result = date_of_birth.is_birthday_passed();

        println!("{}", result);
    }
}
