use sqlx::SqlitePool;
use tracing::info;

/// Simple repository wrapper around a [`SqlitePool`].
pub struct Repository {
    pool: SqlitePool,
}

impl Repository {
    /// Establish a new connection pool.
    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        info!("database connected");
        Ok(Self { pool })
    }
}

