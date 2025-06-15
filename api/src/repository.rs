use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct TaskRepository {
    _pool: sqlx::MySqlPool,  // Tambahkan underscore untuk menghilangkan warning
}

impl TaskRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { _pool: pool }
    }
}