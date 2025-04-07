use anyhow::Error;
use async_trait::async_trait;

use crate::core::domain::entity::user::User;

#[async_trait]
pub trait UserStorage: Send + Sync + 'static {
    async fn save(&self, entity: &User) -> Result<User, Error>;
    async fn update(&self, id_str: &str, entity: &User) -> Result<User, Error>;
    async fn delete(&self, id_str: &str) -> Result<(), Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn find_by_id(&self, id_str: &str) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}
