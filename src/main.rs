use std::error::Error;
use std::sync::Arc;

pub mod adapter;
pub mod core;
pub mod shared;


//todo: mail kismindaki hata giderilicek
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Environment::from_env()
        .load()
        .expect("Environment loading failed!");
    logger::init();
    info!("Logger initialized!");
    let db = DB::new().await?;
    info!("DB initialized!");
    let cache = connect_redis().await;
    info!("Redis initialized");
    let user_repository = Arc::new(UserRepository::new(Arc::clone(&db.pool)));
    // let company_repository = CompanyRepository::new(Arc::clone(&db.pool));
    let user_service = Arc::new(UserService::new(Arc::clone(&user_repository)));
    let mailer = EmailSender::new();
    let task_context = TaskContext::new(cache, mailer);
    let app_state = Arc::new(AppState::new(user_service, task_context));
    let route = make_router(app_state);
    Server::bind().serve(route.into_make_service()).await?;
    Ok(())
}
