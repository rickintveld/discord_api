use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Violation {
    pub id: i32,
    pub user_id: i64,
    pub violation: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewViolation {
    pub user_id: i64,
    pub violation: String,
}
