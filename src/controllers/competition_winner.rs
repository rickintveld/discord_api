use crate::{errors::CustomError, models::competition_winner};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::SqlitePool;

pub async fn create(
    State(pool): State<SqlitePool>,
    Json(winner): Json<competition_winner::NewCompetitionWinner>,
) -> Result<(StatusCode, Json<competition_winner::NewCompetitionWinner>), CustomError> {
    let sql =
        r#"INSERT INTO competition_winner (user_id, rank, price, prop_firm) values (?, ?, ?, ?)"#
            .to_string();

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

pub async fn fetch(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<competition_winner::CompetitionWinner>, CustomError> {
    let sql = r#"SELECT * FROM competition_winner where id = ?"#.to_string();

    let competition_winner: competition_winner::CompetitionWinner = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(Json(competition_winner))
}

pub async fn fetch_all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let sql = r#"SELECT * FROM competition_winner"#.to_string();

    let competition_winners = sqlx::query_as::<_, competition_winner::CompetitionWinner>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(competition_winners))
}

pub async fn fetch_by_user_id(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let sql = r#"SELECT * FROM competition_winner where user_id = ?"#.to_string();

    let competition_winners = sqlx::query_as::<_, competition_winner::CompetitionWinner>(&sql)
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(competition_winners))
}
