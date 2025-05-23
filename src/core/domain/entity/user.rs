use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::password::HashedPassword;
use crate::core::domain::valueobject::role::Role;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub pid: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
    pub password_hash: HashedPassword,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    pub fn new(name: String, surname: String, email: String, password: String, role: Role) -> Self {
        let hashed_password = HashedPassword::new(password.as_str(), &email);
        User {
            id: 0,
            pid: Uuid::new_v4(),
            name,
            surname,
            email,
            role,
            password_hash: hashed_password.unwrap(),
            created_at: Timestamp::now_utc(),
            updated_at: Timestamp::now_utc(),
        }
    }

    pub fn update_role(&mut self, role: Option<Role>) -> Result<(), Error> {
        if let Some(role) = role {
            self.role = role;
        }

        Ok(())
    }
}
