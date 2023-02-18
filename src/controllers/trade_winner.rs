use crate::{errors::CustomError, models::trade_winner};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::post, Router};
use sqlx::{Pool, Sqlite, SqlitePool};

pub fn routing() -> Router<Pool<Sqlite>> {
    let router = Router::new().route("/create", post(create));

    router
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(winner): Json<trade_winner::NewTradeWinner>,
) -> Result<(StatusCode, Json<trade_winner::NewTradeWinner>), CustomError> {
    let sql = r#"INSERT INTO trade_winner (user_id, url) values (?, ?)"#.to_string();

    let _ = sqlx::query(&sql)
        .bind(&winner.user_id)
        .bind(&winner.url)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(winner)))
}
