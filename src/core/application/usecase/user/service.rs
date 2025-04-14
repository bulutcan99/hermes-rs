use std::sync::Arc;

use crate::core::port::user::UserStorage;

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

impl<K> UserService<K>
where
    K: UserStorage,
{
    async fn register(
        &self,
        input: &UserRegisterRequest,
    ) -> Result<User, Box<dyn std::error::Error>> {
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
    //
    //     async fn login(
    //         &self,
    //         input: &UserLoginRequest,
    //     ) -> Result<(String, String), Box<dyn std::error::Error>> {
    //         let found_user = self
    //             .user_repository
    //             .find_by_email(input.email.as_str())
    //             .await
    //             .map_err(|_| LoginError::DbInternalError)?;
    //
    //         let Some(user) = found_user else {
    //             return Err(LoginError::UserNotFound);
    //         };
    //
    //         match user.password_hash.verify_password(&input.password) {
    //             Ok(true) => {
    //                 let access = generate_access_token(user.id.clone(), user.email.clone())
    //                     .map_err(|_| LoginError::JWTEncodingError)?;
    //
    //                 let refresh = generate_refresh_token(user.id, user.email)
    //                     .map_err(|_| LoginError::JWTEncodingError)?;
    //
    //                 Ok()
    //             }
    //             Ok(false) => Err(LoginError::BadCredentials),
    //             Err(_) => Err(LoginError::JWTEncodingError),
    //         }
    //     }
    //
    //     async fn me(&self, email: &str) -> Result<User, Box<dyn std::error::Error>> {
    //         let user = self
    //             .user_repository
    //             .find_by_email(email)
    //             .await
    //             .map_err(|_| MeError::DbInternalError)?
    //             .ok_or(MeError::UserNotFound)?;
    //         Ok(user)
    //     }
}
