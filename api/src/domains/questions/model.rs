use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
    pub attempts: i32,
    pub correct_answers: i32,
}
