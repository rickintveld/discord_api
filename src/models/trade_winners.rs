use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct TradeWinner {
    pub id: i32,
    pub user_id: i64,
    pub url: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewTradeWinner {
    pub user_id: i64,
    pub url: String,
}
