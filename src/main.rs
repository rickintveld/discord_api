use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Could not connect to database");

    let profit_routing = controllers::profit::routing();
    let violation_router = controllers::violation::routing();
    let competition_router = controllers::competition_winner::routing();
    let trade_winner_route = controllers::trade_winner::routing();
    let shared_idea_route = controllers::shared_idea::routing();

    let app = Router::new()
        .route("/", get(root))
        .nest("/profits", profit_routing)
        .nest("/violation", violation_router)
        .nest("/competition", competition_router)
        .nest("/trade-winners", trade_winner_route)
        .nest("/shared-idea", shared_idea_route)
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
        .into_make_service();

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", address);

    axum::Server::bind(&address).serve(app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, traders!"
}
