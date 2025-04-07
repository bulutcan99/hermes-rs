use crate::core::{application::usecase::app_context::AppContext, port::user::UserStorage};

#[derive(Clone, Default, Debug)]
pub struct Route<DB>
where
    DB: UserStorage,
{
    pub prefix: Option<String>,
    pub handler: Vec<Handler<DB>>,
}
#[derive(Clone, Default, Debug)]
pub struct Handler<DB>
where
    DB: UserStorage,
{
    pub uri: String,
    pub method: axum::routing::MethodRouter<AppContext<DB>>,
    pub actions: Vec<http::Method>,
}

impl<DB> Route<DB>
where
    DB: UserStorage,
{
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
