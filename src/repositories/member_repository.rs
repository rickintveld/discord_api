use crate::{errors::CustomError, models::member};
use sqlx::SqlitePool;

pub async fn all(pool: &SqlitePool) -> Result<Vec<member::Member>, CustomError> {
    let sql = r#"SELECT * FROM member "#.to_string();

    let members = sqlx::query_as::<_, member::Member>(&sql)
        .fetch_all(pool)
        .await
        .unwrap();

    Ok(members)
}

pub async fn create(
    pool: &SqlitePool,
    member: member::NewMember,
) -> Result<member::NewMember, CustomError> {
    let sql =
        r#"INSERT INTO member (user_id, username, creation_date) values (?, ?, ?) ON CONFLICT DO NOTHING"#
            .to_string();

    let _ = sqlx::query(&sql)
        .bind(&member.user_id)
        .bind(&member.username)
        .bind(&member.creation_date)
        .execute(pool)
        .await
        .map_err(|_| CustomError::InternalServerError);

    Ok(member)
}

pub async fn find_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<member::Member, CustomError> {
    let sql = r#"SELECT * FROM member where user_id = ?"#.to_string();

    let member: member::Member = sqlx::query_as(&sql)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(member)
}

pub async fn delete(pool: &SqlitePool, user_id: i64) -> Result<String, CustomError> {
    sqlx::query(r#"DELETE FROM member WHERE user_id = ?"#)
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|_| CustomError::RecordNotFound)?;

    Ok(format!("Deleted {}", user_id))
}
