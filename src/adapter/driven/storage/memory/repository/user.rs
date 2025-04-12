use anyhow::Error;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::core::domain::entity::user::User;
use crate::core::port::user::UserStorage;

pub struct InMemoryUserRepo {
    id_counter: Mutex<u64>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            id_counter: Mutex::new(0),
        }
    }
}

impl UserStorage for InMemoryUserRepo {
    async fn save(&self, user: User) -> Result<User, Error> {
        todo!()
    }

    async fn update(&self, id: i32, entity: User) -> Result<User, Error> {
        todo!()
    }

    async fn delete(&self, id: i32) -> Result<(), Error> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        todo!()
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        todo!()
    }
}
