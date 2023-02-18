use crate::{errors::CustomError, models::violation};
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
    Json(violation): Json<violation::NewViolation>,
) -> Result<(StatusCode, Json<violation::NewViolation>), CustomError> {
    let sql = r#"INSERT INTO violation (user_id, violation) values (?, ?)"#.to_string();

    let _ = sqlx::query(&sql)
        .bind(&violation.user_id)
        .bind(&violation.violation)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(violation)))
}
