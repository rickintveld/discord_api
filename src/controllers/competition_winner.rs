use crate::{errors::CustomError, models::competition_winner};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::SqlitePool;

pub async fn create(
    State(pool): State<SqlitePool>,
    Json(winner): Json<competition_winner::NewCompetitionWinner>,
) -> Result<(StatusCode, Json<competition_winner::NewCompetitionWinner>), CustomError> {
    let sql =
        r#"INSERT INTO competition_winner (user_id, rank, price, prop_firm) values (?, ?, ?, ?)"#;

    let _ = sqlx::query(&sql)
        .bind(&winner.user_id)
        .bind(&winner.rank)
        .bind(&winner.price)
        .bind(&winner.prop_firm)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(winner)))
}
