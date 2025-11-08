use anyhow::Result;
use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

use crate::{
    config::{db, jwt},
    state::AppState,
};

mod config;
mod middleware;
mod modules;
mod state;
mod utils;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let db = db::establish_connection().await?;
    let jwt_config = jwt::load_jwt_config()?;

    let state = AppState { db, jwt_config };

    let app = Router::new()
        .route(
            "/health",
            get(|| async { Json(HealthResponse { status: "ok" }) }),
        )
        .nest("/api/users", modules::user::user_route::router())
        .with_state(state);

    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 3000: {e}");
            return Err(e.into());
        }
    };

    let local_addr = listener.local_addr()?;
    println!("Server running on http://{local_addr}");

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {e}");
        return Err(e.into());
    }

    Ok(())
}
