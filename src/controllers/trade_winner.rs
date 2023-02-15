use crate::{errors::CustomError, models::trade_winner};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::SqlitePool;

pub async fn create(
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
