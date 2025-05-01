use anyhow::Result;
use common_core::{sqlite_pool, sqlx};
pub use staratlas_sage;

pub async fn db_pool(database_url: &str) -> Result<sqlx::Pool<sqlx::sqlite::Sqlite>> {
    let pool = sqlite_pool(database_url).await?;
    sqlx::migrate!("./../../migrations/sage").run(&pool).await?;
    Ok(pool)
}
