use axum::{
    routing::{get, post},
    Router,
};

use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://database.db")
        .await
        .expect("Could not connect to database");

    let app = Router::new()
        .route("/", get(root))
        .route("/profits", get(controllers::profit::all))
        .route("/profits/create", post(controllers::profit::create))
        .route("/profits/:user_id", get(controllers::profit::fetch))
        .route("/violation/create", post(controllers::violation::create))
        .route(
            "/competition-winner/create",
            post(controllers::competition_winner::create),
        )
        .route(
            "/trade-winner/create",
            post(controllers::trade_winner::create),
        )
        .route(
            "/shared-idea/create",
            post(controllers::shared_idea::create),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
