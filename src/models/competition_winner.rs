use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct CompetitionWinner {
    pub id: i32,
    pub user_id: i64,
    pub number: i32,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewCompetitionWinner {
    pub user_id: i64,
    pub number: i32,
}
