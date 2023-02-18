use crate::{errors::CustomError, models::shared_idea};
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
    Json(idea): Json<shared_idea::NewSharedIdea>,
) -> Result<(StatusCode, Json<shared_idea::NewSharedIdea>), CustomError> {
    let sql = r#"INSERT INTO shared_idea (user_id, url) values (?, ?)"#.to_string();

    let _ = sqlx::query(&sql)
        .bind(&idea.user_id)
        .bind(&idea.url)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(idea)))
}
