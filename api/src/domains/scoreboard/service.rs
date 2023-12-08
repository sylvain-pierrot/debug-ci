use sqlx::{Pool, Sqlite};

use super::{schema::CreateScoreDto, model::Score};

pub async fn create_score(db: &Pool<Sqlite>, create_score_dto: CreateScoreDto) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO scoreboard (id, name, longest_streak)
        VALUES ($1, $2, $3)
        "#
    )
    .bind(id)
    .bind(create_score_dto.name)
    .bind(create_score_dto.longest_streak)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn fetch_last_ten_scores(db: &Pool<Sqlite>) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>(
        r#"
        SELECT id, name, longest_streak
        FROM scoreboard
        LIMIT 10
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(scores)
}
