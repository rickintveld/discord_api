use crate::{errors::CustomError, models::shared_idea};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::SqlitePool;

pub async fn create(
    State(pool): State<SqlitePool>,
    Json(idea): Json<shared_idea::NewSharedIdea>,
) -> Result<(StatusCode, Json<shared_idea::NewSharedIdea>), CustomError> {
    let sql = r#"INSERT INTO shared_idea (user_id, url) values (?, ?)"#;

    let _ = sqlx::query(&sql)
        .bind(&idea.user_id)
        .bind(&idea.url)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(idea)))
}
