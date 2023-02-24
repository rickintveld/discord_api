use crate::{errors::CustomError, models::member, repositories::member_repository};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};

pub fn routing() -> Router<Pool<Sqlite>> {
    let router = Router::new()
        .route("/", get(all))
        .route("/create", post(create))
        .route("/sync", post(sync))
        .route("/:user_id", get(fetch))
        .route("/:user_id", delete(delete_by_user_id));

    router
}

async fn all(State(pool): State<SqlitePool>) -> Result<Json<Vec<member::Member>>, CustomError> {
    let members = member_repository::all(&pool).await?;

    Ok(Json(members))
}

async fn fetch(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<member::Member>, CustomError> {
    let member: member::Member = member_repository::find_by_user_id(&pool, user_id).await?;

    Ok(Json(member))
}

async fn create(
    State(pool): State<SqlitePool>,
    Json(member): Json<member::NewMember>,
) -> Result<StatusCode, CustomError> {
    let _create = member_repository::create(&pool, member).await;

    Ok(StatusCode::CREATED)
}

async fn delete_by_user_id(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode, CustomError> {
    let _find = member_repository::find_by_user_id(&pool, user_id).await?;
    let _delete = member_repository::delete(&pool, user_id).await?;

    Ok(StatusCode::OK)
}

async fn sync(
    State(pool): State<SqlitePool>,
    Json(members): Json<member::NewMembers>,
) -> Result<StatusCode, CustomError> {
    for member in members.members {
        let _ = member_repository::create(&pool, member).await?;
    }

    Ok(StatusCode::CREATED)
}
