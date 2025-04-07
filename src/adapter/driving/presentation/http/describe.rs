use crate::core::application::usecase::app_context::AppContext;
use crate::core::port::user::UserStorage;
use axum::routing::MethodRouter;
use regex::Regex;
use std::sync::OnceLock;

static DESCRIBE_METHOD_ACTION: OnceLock<Regex> = OnceLock::new();

fn get_describe_method_action() -> &'static Regex {
    DESCRIBE_METHOD_ACTION.get_or_init(|| Regex::new(r"\b(\w+):\s*(BoxedHandler|Route)\b").unwrap())
}

pub fn method_action<DB>(method: &MethodRouter<AppContext<DB>>) -> Vec<http::Method>
where
    DB: UserStorage,
{
    let method_str = format!("{method:?}");

    get_describe_method_action()
        .captures_iter(&method_str)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_lowercase()))
        .filter_map(|method_name| match method_name.as_str() {
            "get" => Some(http::Method::GET),
            "post" => Some(http::Method::POST),
            "put" => Some(http::Method::PUT),
            "delete" => Some(http::Method::DELETE),
            "head" => Some(http::Method::HEAD),
            "options" => Some(http::Method::OPTIONS),
            "connect" => Some(http::Method::CONNECT),
            "patch" => Some(http::Method::PATCH),
            "trace" => Some(http::Method::TRACE),
            _ => {
                tracing::info!("Unknown method: {}", method_name);
                None
            }
        })
        .into_iter()
        .collect::<Vec<_>>()
}
