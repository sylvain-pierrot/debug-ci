use sqlx::{Pool, Sqlite};

use super::{model::Question, schema::{CreateQuestionDto, IncrementAnswerCountDto}};

pub async fn create_question(
    create_question_dto: CreateQuestionDto,
    db: &Pool<Sqlite>,
) -> Result<Question, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();

    let counter = sqlx::query_as::<_, Question>(
        r#"
        INSERT INTO questions (id, statement, answer, explanation, attempts, correct_answers)
        VALUES ($1, $2, $3, $4, 0, 0)
        RETURNING id, statement, answer, explanation, attempts, correct_answers
        "#,
    )
    .bind(id)
    .bind(create_question_dto.statement)
    .bind(create_question_dto.answer)
    .bind(create_question_dto.explanation)
    .fetch_one(db)
    .await?;

    Ok(counter)
}

pub async fn fetch_random_questions(db: &Pool<Sqlite>) -> Result<Vec<Question>, sqlx::Error> {
    let counters = sqlx::query_as::<_, Question>(
        r#"
        SELECT id, statement, answer, explanation, attempts, correct_answers
        FROM questions
        ORDER BY RANDOM()
        LIMIT 10
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(counters)
}

pub async fn increment_answer_count(id: String, increment_answer_count_dto: IncrementAnswerCountDto, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let increment_count = match increment_answer_count_dto.is_correct {
        true => 1,
        false => 0,
    };

    sqlx::query(
        r#"
        UPDATE questions
        SET attempts = attempts + 1, correct_answers = $1
        WHERE id = $2
        "#,
    )
    .bind(increment_count)
    .bind(id)
    .execute(db)
    .await?;

    Ok(())
}

// pub async fn get_counters(db: &Pool<Sqlite>) -> Result<Vec<Counter>, sqlx::Error> {
//     let counters = sqlx::query_as::<_, Counter>(
//         r#"
//         SELECT name, value, description
//         FROM counter
//         "#,
//     )
//     .fetch_all(db)
//     .await?;

//     Ok(counters)
// }

// pub async fn get_counter(name: String, db: &Pool<Sqlite>) -> Result<Counter, sqlx::Error> {
//     let counter = sqlx::query_as::<_, Counter>(
//         r#"
//         SELECT name, value, description
//         FROM counter
//         WHERE name = $1
//         "#,
//     )
//     .bind(name)
//     .fetch_one(db)
//     .await?;

//     Ok(counter)
// }

// pub async fn update_counter(
//     counter_name: String,
//     update_counter_dto: UpdateCounterDto,
//     db: &Pool<Sqlite>,
// ) -> Result<Counter, sqlx::Error> {
//     let counter = sqlx::query_as::<_, Counter>(
//         r#"
//         UPDATE counter
//         SET value = $2, description = $3
//         WHERE name = $1
//         RETURNING name, value, description
//         "#,
//     )
//     .bind(counter_name)
//     .bind(update_counter_dto.value)
//     .bind(update_counter_dto.description)
//     .fetch_one(db)
//     .await?;

//     Ok(counter)
// }

// pub async fn increment_counter(
//     name: String,
//     value: i32,
//     db: &Pool<Sqlite>,
// ) -> Result<Counter, sqlx::Error> {
//     let counter = sqlx::query_as::<_, Counter>(
//         r#"
//         UPDATE counter
//         SET value = value + $2
//         WHERE name = $1
//         RETURNING name, value, description
//         "#,
//     )
//     .bind(name)
//     .bind(value)
//     .fetch_one(db)
//     .await?;

//     Ok(counter)
// }

// pub async fn delete_counter(name: String, db: &Pool<Sqlite>) -> Result<Counter, sqlx::Error> {
//     let counter = sqlx::query_as::<_, Counter>(
//         r#"
//         DELETE FROM counter
//         WHERE name = $1
//         RETURNING name, value
//         "#,
//     )
//     .bind(name)
//     .fetch_one(db)
//     .await?;

//     Ok(counter)
// }
