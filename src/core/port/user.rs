use anyhow::Error;

use crate::core::domain::entity::user::User;

pub trait UserStorage: Send + Sync + 'static {
    async fn save(&self, entity: User) -> Result<User, Error>;
    async fn update(&self, id: i32, entity: User) -> Result<User, Error>;
    async fn delete(&self, id: i32) -> Result<(), Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}
