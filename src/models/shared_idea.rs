use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct SharedIdea {
    pub id: i32,
    pub user_id: i64,
    pub idea_url: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewSharedIdea {
    pub user_id: i64,
    pub url: String,
}
