use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct CompetitionWinner {
    pub id: i32,
    pub user_id: i64,
    pub rank: i32,
    pub price: i32,
    pub prop_firm: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewCompetitionWinner {
    pub user_id: i64,
    pub rank: i32,
    pub price: i32,
    pub prop_firm: String,
}
