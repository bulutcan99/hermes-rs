use crate::adapter::driven::storage::db::db_connection::DB;
use crate::adapter::driven::storage::db::repository::user::DatabaseUserRepo;
use crate::adapter::driven::storage::memory::redis_connection::connect_redis;
use crate::adapter::driving::presentation::http::router::make_router;
use crate::shared::config::environment::Environment;
use crate::shared::logger::logger;
use axum_server::Server;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{fmt, Layer};

pub mod adapter;
pub mod core;
pub mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Environment::from_env()
        .load()
        .expect("Environment loading failed!");
    logger::init();
    tracing::info!("Logger initialized");
    let db = DB::new().await?;
    tracing::info!("DB initialized!");
    let cache = connect_redis().await;
    tracing::info!("Redis initialized");
    //TODO: arastir bakalim neden &db aliyor arc olmasina ragmen
    let user_repository = Arc::new(DatabaseUserRepo::new(Arc::clone(&db.pool)));
    // let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));
    // let app_state = Arc::new(AppState::new(user_service));
    // let route = make_router(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server");
    // axum_server::bind(addr)
    //     .serve(route.into_make_service())
    //     .await?;
    Ok(())
}
