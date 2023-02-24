use crate::{errors::CustomError, models::member};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::task::JoinSet;

pub fn routing() -> Router<Pool<Sqlite>> {
    let router = Router::new()
        .route("/", get(all))
        .route("/create", post(create))
        .route("/sync", post(sync))
        .route("/:user_id", get(fetch))
        .route("/:user_id", delete(delete_by_user_id));

    router
}

async fn all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let sql = r#"SELECT * FROM member "#.to_string();

    let members = sqlx::query_as::<_, member::Member>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(members))
}

async fn fetch(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<member::Member>, CustomError> {
    let sql = r#"SELECT * FROM member where user_id = ?"#.to_string();

    let member: member::Member = sqlx::query_as(&sql)
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(Json(member))
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(member): Json<member::NewMember>,
) -> Result<(StatusCode, Json<member::NewMember>), CustomError> {
    let sql =
        r#"INSERT INTO member (user_id, username, creation_date) values (?, ?, ?)"#.to_string();

    let _ = sqlx::query(&sql)
        .bind(&member.user_id)
        .bind(&member.username)
        .bind(&member.creation_date)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(member)))
}

async fn delete_by_user_id(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode, CustomError> {
    let find_query = r#"SELECT * FROM member where user_id = ?"#;
    let _find_member: member::Member = sqlx::query_as(&find_query)
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    sqlx::query(r#"DELETE FROM member WHERE user_id = ?"#)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(StatusCode::OK)
}

async fn sync(
    State(pool): State<SqlitePool>,
    Json(members): Json<member::NewMembers>,
) -> Result<StatusCode, CustomError> {
    let mut set: JoinSet<member::NewMember> = JoinSet::new();
    let sql =
        r#"INSERT INTO member (user_id, username, creation_date) values (?, ?, ?) ON CONFLICT DO NOTHING"#
            .to_string();

    for member in members.members {
        set.spawn(async move { member });
    }

    while let Some(result) = set.join_next().await {
        let member: member::NewMember = result.unwrap();

        let _ = sqlx::query(&sql)
            .bind(&member.user_id)
            .bind(&member.username)
            .bind(&member.creation_date)
            .execute(&pool)
            .await
            .map_err(|_| CustomError::InternalServerError)?;
    }

    Ok(StatusCode::CREATED)
}
