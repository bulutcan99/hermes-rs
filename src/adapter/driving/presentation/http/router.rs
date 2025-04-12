use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::core::application::usecase::app_context::AppContext;
use crate::core::port::user::UserStorage;

pub fn make_router<S>(app_state: Arc<AppContext<S>>) -> Router
where
    S: UserStorage + 'static,
{
    todo!()
    // let protected_routes = Router::new().route(
    //     "/api/v1/users/me",
    //     get(me_handler).layer(from_fn_with_state(app_state.clone(), is_authenticated)),
    // );
    //
    // let public_routes = Router::new()
    //     .route("/api/v1/healthchecker", get(health_checker_handler))
    //     .route(
    //         "/api/v1/auth/register",
    //         post(auth::register::register_handler),
    //     )
    //     .route("/api/v1/auth/login", post(login_handler));
    //
    // Router::new()
    //     .merge(public_routes)
    //     .merge(protected_routes)
    //     .layer(CookieManagerLayer::new())
    //     .with_state(app_state)
}
