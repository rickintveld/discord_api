use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Profit {
    pub id: i32,
    pub user_id: i64,
    pub profit: f32,
    pub risk_to_reward: f32,
    pub creation_date: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewProfit {
    pub user_id: i64,
    pub profit: f32,
    pub risk_to_reward: f32,
    pub creation_date: String,
}
