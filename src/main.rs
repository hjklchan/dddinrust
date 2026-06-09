use dddinrust::domain::user::repo::UserRepo;

use crate::repos::UserMemoryRepository;

fn main() {}

mod repos {
    use dddinrust::domain::{
        self,
        user::{User, UserId, repo::Error},
    };

    pub struct UserMemoryRepository {
        //
    }

    impl domain::user::repo::UserRepo for UserMemoryRepository {
        async fn create(&self, user: User) -> Result<(), Error> {
            todo!()
        }

        async fn delete_by_id(&self, user_id: UserId) -> Result<(), Error> {
            todo!()
        }

        async fn update(&self, user: User) -> Result<(), Error> {
            todo!()
        }

        async fn get_by_id(&self, user_id: UserId) -> Result<User, Error> {
            todo!()
        }

        async fn all(&self) -> Result<Vec<User>, Error> {
            todo!()
        }
    }
}
