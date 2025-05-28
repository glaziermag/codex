use sqlx::{PgPool, postgres::PgPoolOptions};
use thiserror::Error;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct Task {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    pub async fn new(database_url: &str) -> Result<Self, StoreError> {
        let pool = PgPoolOptions::new()
            .connect(database_url)
            .await?;
        sqlx::migrate!("migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn add_task(&self, name: &str) -> Result<Task, StoreError> {
        let rec = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks(name) VALUES ($1) RETURNING id, name",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn get_task(&self, id: i32) -> Result<Option<Task>, StoreError> {
        let rec = sqlx::query_as::<_, Task>(
            "SELECT id, name FROM tasks WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn list_tasks(&self, limit: Option<i64>) -> Result<Vec<Task>, StoreError> {
        let tasks = if let Some(limit) = limit {
            sqlx::query_as::<_, Task>(
                "SELECT id, name FROM tasks ORDER BY id LIMIT $1",
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Task>("SELECT id, name FROM tasks ORDER BY id")
                .fetch_all(&self.pool)
                .await?
        };
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_compile() {
        let _ = sqlx::migrate!("migrations");
    }
}
