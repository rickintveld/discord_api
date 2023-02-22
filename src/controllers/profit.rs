use crate::{errors::CustomError, models::profit};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};

pub fn routing() -> Router<Pool<Sqlite>> {
    let router = Router::new()
        .route("/", get(all))
        .route("/create", post(create))
        .route("/:user_id", get(fetch));

    router
}

async fn all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let sql = r#"SELECT * FROM profit "#.to_string();

    let profits = sqlx::query_as::<_, profit::Profit>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(profits))
}

async fn fetch(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<profit::Profit>, CustomError> {
    let sql = r#"SELECT * FROM profit where user_id = ?"#.to_string();

    let profit: profit::Profit = sqlx::query_as(&sql)
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(Json(profit))
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(profit): Json<profit::NewProfit>,
) -> Result<(StatusCode, Json<profit::NewProfit>), CustomError> {
    let sql = r#"INSERT INTO profit (user_id, profit, risk_to_reward, creation_date) values (?, ?, ?, ?)"#.to_string();

    let _ = sqlx::query(&sql)
        .bind(&profit.user_id)
        .bind(&profit.profit)
        .bind(&profit.risk_to_reward)
        .bind(&profit.creation_date)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(profit)))
}
