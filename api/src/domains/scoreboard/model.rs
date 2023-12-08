use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Score {
    pub id: String,
    pub name: String,
    pub longest_streak: i32,
}
