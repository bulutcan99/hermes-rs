use std::sync::Arc;

use async_trait::async_trait;
use validator::ValidationErrors;

use crate::adapter::driving::presentation::http::handler::auth::login::UserLoginRequest;
use crate::adapter::driving::presentation::http::handler::auth::register::UserRegisterRequest;
use crate::core::application::usecase::auth::error::{LoginError, MeError, RegisterError};
use crate::core::application::usecase::auth::jwt::{generate_access_token, generate_refresh_token};
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::role;
use crate::core::port::user::{UserManagement, UserStorage};

#[derive(Debug, Clone)]
pub struct UserService<K>
where
    K: UserStorage,
{
    user_repository: Arc<K>,
}

impl<K> UserService<K>
where
    K: UserStorage,
{
    pub fn new(user_repository: Arc<K>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<K> UserManagement for UserService<K>
where
    K: UserStorage,
{
    async fn register(
        &self,
        input: &UserRegisterRequest,
    ) -> Result<User, RegisterError<ValidationErrors>> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await
            .map_err(|_| RegisterError::DbInternalError)?;

        if found_user.is_some() {
            return Err(RegisterError::UserAlreadyRegistered);
        }

        let new_user = User::new(
            input.name.clone(),
            input.surname.clone(),
            input.email.clone(),
            input.password.clone(),
            role::Role::USER,
        );

        let registered_user = self
            .user_repository
            .save(&new_user)
            .await
            .map_err(|_| RegisterError::DbInternalError)?;

        Ok(registered_user)
    }

    async fn login(&self, input: &UserLoginRequest) -> Result<(String, String), LoginError> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await
            .map_err(|_| LoginError::DbInternalError)?;

        let Some(user) = found_user else {
            return Err(LoginError::UserNotFound);
        };

        match user.password_hash.verify_password(&input.password) {
            Ok(true) => {
                let access = generate_access_token(user.id.clone(), user.email.clone())
                    .map_err(|_| LoginError::JWTEncodingError)?;

                let refresh = generate_refresh_token(user.id, user.email)
                    .map_err(|_| LoginError::JWTEncodingError)?;

                Ok()
            }
            Ok(false) => Err(LoginError::BadCredentials),
            Err(_) => Err(LoginError::JWTEncodingError),
        }
    }

    async fn me(&self, email: &str) -> Result<User, MeError> {
        let user = self
            .user_repository
            .find_by_email(email)
            .await
            .map_err(|_| MeError::DbInternalError)?
            .ok_or(MeError::UserNotFound)?;
        Ok(user)
    }
}
