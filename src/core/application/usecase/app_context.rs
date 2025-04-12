use std::sync::Arc;

use crate::core::port::user::UserStorage;

use super::user::service::UserService;

#[derive(Debug)]
pub struct AppContext<DB>
where
    DB: UserStorage,
{
    pub user_service: UserService<DB>,
}
