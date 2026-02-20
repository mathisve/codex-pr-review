mod db;
mod handlers;
mod models;
mod templates;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{AppState, *};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "hotel.db".to_string());
    let pool = db::init_db(&db_path).await?;
    let state: AppState = Arc::new(pool);

    let app = Router::new()
        .route("/", get(home))
        .route("/search", get(search))
        .route("/hotel/:id", get(hotel_detail))
        .route("/room/:id", get(room_detail))
        .route("/room/:id/book", axum::routing::post(book_room))
        .route("/booking/:id", get(booking_confirmation))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Hotel booking server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
