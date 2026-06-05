use crate::domain::user::{UserId, date_of_birth::DateOfBirth, gender::Gender, status::Status};
use chrono::Utc;

#[derive(Debug)]
pub struct User {
    id: UserId,

    nick_name: String,
    username: String,
    gender: Gender,

    date_of_birth: Option<DateOfBirth>,
    created_at: chrono::DateTime<Utc>,
    status: Status,
}

#[derive(Debug, thiserror::Error)]
pub enum UserDomainError {
    #[error("nick name can not be empty")]
    EmptyNickName,

    #[error("nick name length is invalid: {0}")]
    InvalidNickNameLength(&'static str),

    #[error("once a username is set, it cannot be changed")]
    UsernameModificationNotAllowed,

    #[error("{0}")]
    UserStatusError(&'static str),
}

impl User {
    fn new(
        id: UserId,
        nick_name: String,
        username: String,
        gender: Gender,
        date_of_birth: Option<DateOfBirth>,
        created_at: chrono::DateTime<Utc>,
        status: Status,
    ) -> Self {
        Self {
            id,
            nick_name,
            username,
            gender,
            date_of_birth,
            created_at,
            status,
        }
    }

    pub fn create(
        nick_name: String,
        username: String,
        gender: Gender,
        date_of_birth: Option<DateOfBirth>,
    ) -> Result<User, UserDomainError> {
        let id = UserId::generate();
        let created_at = Utc::now();
        let status = Status::default();

        Ok(Self::new(
            id,
            nick_name,
            username,
            gender,
            date_of_birth,
            created_at,
            status,
        ))
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn date_of_birth(&self) -> Option<DateOfBirth> {
        self.date_of_birth
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn update_date_of_birth(&mut self, value: DateOfBirth) {
        self.date_of_birth = Some(value)
    }

    pub fn update_nickname(&mut self, value: String) -> Result<(), UserDomainError> {
        if value.is_empty() {
            return Err(UserDomainError::EmptyNickName);
        }
        if value.len() >= 15 {
            return Err(UserDomainError::InvalidNickNameLength(
                "the length cannot exceed 15 characters",
            ));
        }

        self.nick_name = value;

        Ok(())
    }

    pub fn update_username(&mut self) -> Result<(), UserDomainError> {
        Err(UserDomainError::UsernameModificationNotAllowed)
    }

    pub fn update_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn suspende(&mut self) -> Result<(), UserDomainError> {
        self.status = self
            .status()
            .suspended()
            .map_err(|e| UserDomainError::UserStatusError(e))?;

        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), UserDomainError> {
        self.status = self
            .status()
            .disable()
            .map_err(|e| UserDomainError::UserStatusError(e))?;
        Ok(())
    }

    pub fn active(&mut self) -> Result<(), UserDomainError> {
        self.status = self
            .status()
            .active()
            .map_err(|e| UserDomainError::UserStatusError(e))?;
        Ok(())
    }
}
