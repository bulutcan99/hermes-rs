use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres};

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::password::HashedPassword;
use crate::core::domain::valueobject::role::Role;
use crate::core::port::user::UserStorage;

#[derive(Debug, Clone)]
pub struct DatabaseUserRepo {
    db: Arc<Pool<Postgres>>,
}

impl DatabaseUserRepo {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        DatabaseUserRepo { db }
    }
}

impl UserStorage for DatabaseUserRepo {
    async fn save(&self, user: User) -> Result<User, Error> {
        let db = self.db.clone();
        let result = sqlx::query!(
            r#"
            INSERT INTO "user" (pid, name, surname, email, role, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, pid, name, surname, email, role, password_hash, created_at, updated_at
            "#,
            user.pid,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            user.created_at.convert_to_offset(),
            user.updated_at.convert_to_offset(),
        )
        .fetch_one(&*db)
        .await?;

        Ok(User {
            id: result.id,
            pid: result.pid,
            name: result.name,
            surname: result.surname,
            email: result.email,
            role: Role::from(result.role),
            password_hash: HashedPassword::from(result.password_hash),
            created_at: Timestamp::from(result.created_at),
            updated_at: Timestamp::from(result.updated_at),
        })
    }

    async fn update(&self, id: i32, user: User) -> Result<User, Error> {
        let db = self.db.clone();
        let result = sqlx::query!(
            r#"
            UPDATE "user"
            SET
                name = $2,
                surname = $3,
                email = $4,
                role = $5,
                password_hash = $6,
                updated_at = $7
            WHERE id = $1
            RETURNING id, pid, name, surname, email, role, password_hash, created_at, updated_at
            "#,
            id,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*db)
        .await?;

        Ok(User {
            id: result.id,
            pid: result.pid,
            name: result.name,
            surname: result.surname,
            email: result.email,
            role: Role::from(result.role),
            password_hash: HashedPassword::from(result.password_hash),
            created_at: Timestamp::from(result.created_at),
            updated_at: Timestamp::from(result.updated_at),
        })
    }

    async fn delete(&self, id: i32) -> Result<(), Error> {
        let db = self.db.clone();
        sqlx::query!(
            r#"
            DELETE FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .execute(&*db)
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let db = self.db.clone();
        let rows = sqlx::query!(
            r#"
            SELECT id, pid, name, surname, email, role, password_hash, created_at, updated_at
            FROM "user"
            "#
        )
        .fetch_all(&*db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                pid: row.pid,
                name: row.name,
                surname: row.surname,
                email: row.email,
                role: Role::from(row.role),
                password_hash: HashedPassword::from(row.password_hash),
                created_at: Timestamp::from(row.created_at),
                updated_at: Timestamp::from(row.updated_at),
            })
            .collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        let db = self.db.clone();
        let row = sqlx::query!(
            r#"
            SELECT id, pid, name, surname, email, role, password_hash, created_at, updated_at
            FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*db)
        .await?;

        Ok(row.map(|row| User {
            id: row.id,
            pid: row.pid,
            name: row.name,
            surname: row.surname,
            email: row.email,
            role: Role::from(row.role),
            password_hash: HashedPassword::from(row.password_hash),
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let db = self.db.clone();
        let row = sqlx::query!(
            r#"
            SELECT id, pid, name, surname, email, role, password_hash, created_at, updated_at
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*db)
        .await?;

        Ok(row.map(|row| User {
            id: row.id,
            pid: row.pid,
            name: row.name,
            surname: row.surname,
            email: row.email,
            role: Role::from(row.role),
            password_hash: HashedPassword::from(row.password_hash),
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        }))
    }
}
