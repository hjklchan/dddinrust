use crate::domain::user::{User, UserId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("conflict user")]
    UserConflict,
    #[error("user not found")]
    UserNotFound,
}

pub trait UserRepo {
    fn create(&self, user: User) -> Result<(), Error>;
    fn delete_by_id(&self, user_id: UserId) -> Result<(), Error>;
    fn update(&self, user: User) -> Result<(), Error>;
    fn get_by_id(&self, user_id: UserId) -> Result<User, Error>;
    fn all(&self) -> Result<Vec<User>, Error>;
}
