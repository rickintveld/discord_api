use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Member {
    pub id: i32,
    pub user_id: i64,
    pub username: String,
    pub creation_date: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewMember {
    pub user_id: i64,
    pub username: String,
    pub creation_date: String,
}
