use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use uuid::Uuid;

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

#[async_trait]
impl UserStorage for DatabaseUserRepo {
    async fn save(&self, user: &User) -> Result<User, Error> {
        let result = sqlx::query!(
            r#"
        INSERT INTO "user" (id, name, surname, email, role, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, surname, email, role, password_hash, created_at, updated_at
        "#,
            user.id,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            Timestamp::now_utc().convert_to_offset(),
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*self.db)
        .await?;

        let saved_user = User {
            id: Some(result.id),
            name: result.name,
            surname: result.surname,
            email: result.email,
            role: Role::from(result.role),
            password_hash: HashedPassword::from(result.password_hash),
            created_at: Timestamp::from(result.created_at),
            updated_at: Timestamp::from(result.updated_at),
        };

        Ok(saved_user)
    }

    async fn update(&self, id_str: &str, user: &User) -> Result<User, Error> {
        let id = Uuid::parse_str(id_str)?;
        let result = sqlx::query!(
            r#"
        UPDATE "user"
        SET
            name = COALESCE($2, name),
            surname = COALESCE($3, surname),
            email = COALESCE($4, email),
            role = COALESCE($5, role),
            password_hash = COALESCE($6, password_hash),
            updated_at = COALESCE($7, updated_at)
        WHERE id = $1
        RETURNING id, name, surname, email, role, password_hash, created_at, updated_at
        "#,
            id,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*self.db)
        .await?;

        let updated_user = User {
            id: Some(result.id),
            name: result.name,
            surname: result.surname,
            email: result.email,
            role: Role::from(result.role),
            password_hash: HashedPassword::from(result.password_hash),
            created_at: Timestamp::from(result.created_at),
            updated_at: Timestamp::from(result.updated_at),
        };

        Ok(updated_user)
    }

    async fn delete(&self, id_str: &str) -> Result<(), Error> {
        let id = Uuid::parse_str(id_str)?;

        sqlx::query!(
            r#"
						DELETE FROM "user"
						WHERE id = $1
						"#,
            id
        )
        .execute(&*self.db)
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let rows = sqlx::query!(
            r#"
						SELECT id, name, surname, email, role, password_hash, created_at, updated_at
						FROM "user"
						"#
        )
        .fetch_all(&*self.db)
        .await?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: Some(row.id),
                name: row.name,
                surname: row.surname,
                email: row.email,
                role: Role::from(row.role),
                password_hash: HashedPassword::from(row.password_hash),
                created_at: Timestamp::from(row.created_at),
                updated_at: Timestamp::from(row.updated_at),
            })
            .collect();

        Ok(users)
    }

    async fn find_by_id(&self, id_str: &str) -> Result<Option<User>, Error> {
        let id = Uuid::parse_str(id_str)?;
        let row = sqlx::query!(
            r#"
						SELECT id, name, surname, email, role, password_hash, created_at, updated_at
						FROM "user"
						WHERE id = $1
						"#,
            id
        )
        .fetch_optional(&*self.db)
        .await?;

        let user = row.map(|row| User {
            id: Some(row.id),
            name: row.name,
            surname: row.surname,
            email: row.email,
            role: Role::from(row.role),
            password_hash: HashedPassword::from(row.password_hash),
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        });

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
						SELECT id, name, surname, email, role, password_hash, created_at, updated_at
						FROM "user"
						WHERE email = $1
						"#,
            email
        )
        .fetch_optional(&*self.db)
        .await?;

        let user = row.map(|row| User {
            id: Some(row.id),
            name: row.name,
            surname: row.surname,
            email: row.email,
            role: Role::from(row.role),
            password_hash: HashedPassword::from(row.password_hash),
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        });

        Ok(user)
    }
}
