use crate::{errors::CustomError, models::violation};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::SqlitePool;

pub async fn create(
    State(pool): State<SqlitePool>,
    Json(violation): Json<violation::NewViolation>,
) -> Result<(StatusCode, Json<violation::NewViolation>), CustomError> {
    let sql = r#"INSERT INTO violation (user_id, violation) values (?, ?)"#;

    let _ = sqlx::query(&sql)
        .bind(&violation.user_id)
        .bind(&violation.violation)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(violation)))
}
