use std::sync::Arc;

use super::user::service::UserService;

#[derive(Debug)]
pub struct AppContext<DB> {
    pub user_service: UserService<DB>,
}
