use sqlx::{Pool, Sqlite};

use crate::common::config::env::AppEnvironment;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub env: AppEnvironment,
}

impl AppState {
    pub fn new(db: Pool<Sqlite>, env: AppEnvironment) -> Self {
        Self { db, env }
    }
}
