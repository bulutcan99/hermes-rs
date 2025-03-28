use axum::routing::get;
use axum::Router;
use serde_json::map::VacantEntry;
use serde_json::Value;
use socketioxide::{
	extract::{Data, SocketRef},
	SocketIo,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct MessageInputData{
  room: String,
  text: String,
}

#[derive(Debug, serde::Serialize)]
struct MessageOutputData{
  text: String,
  user: String,
  date: chrono::DateTime<chrono::Utc>,
}

async fn on_connect(socket: SocketRef) {
    info!("Connecting to socket {:?}", socket);

  socket.on("joinRoom", |sock: SocketRef, Data::<MessageInputData>(data)| async move {
    info!("Received message in room {:?}: {:?}", data.room, data.text);
        let output = MessageOutputData {
            date: chrono::Utc::now(),
            text: data.text,
            user: format!("anon-{}", sock.id),
        };

        info!("Sending message: {:?}", output);
        if let Err(err) = sock.emit("message", &output) {
            error!("Failed to send message: {:?}", err);
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
  let (io_layer, io) = SocketIo::new_layer();
  io.ns("/", on_connect);
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(io_layer)
                .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server");
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
