use axum::routing::IntoMakeService;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
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

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool: Pool<Sqlite> = database_pool().await;
    let application: IntoMakeService<Router> = application(pool);

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(application)
        .await
        .unwrap();

    Ok(())
}

async fn database_pool() -> Pool<Sqlite> {
    let database_url: String = env::var("DATABASE_URL").unwrap();

    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Could not connect to database");

    return pool;
}

fn application(pool: Pool<Sqlite>) -> IntoMakeService<Router> {
    let profit_routing: Router<Pool<Sqlite>> = controllers::profit::routing();
    let violation_router: Router<Pool<Sqlite>> = controllers::violation::routing();
    let competition_router: Router<Pool<Sqlite>> = controllers::competition_winner::routing();
    let trade_winner_route: Router<Pool<Sqlite>> = controllers::trade_winner::routing();
    let shared_idea_route: Router<Pool<Sqlite>> = controllers::shared_idea::routing();
    let member_route: Router<Pool<Sqlite>> = controllers::member::routing();

    let routing: IntoMakeService<Router> = Router::new()
        .route("/", get(root))
        .nest("/competition", competition_router)
        .nest("/member", member_route)
        .nest("/profits", profit_routing)
        .nest("/shared-idea", shared_idea_route)
        .nest("/trade-winners", trade_winner_route)
        .nest("/violation", violation_router)
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
        .into_make_service();

    return routing;
}

async fn root() -> &'static str {
    "Hello, traders!"
}
