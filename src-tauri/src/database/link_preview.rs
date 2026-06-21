use serde::{Deserialize, Serialize};
use sqlx::SqliteExecutor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LinkPreviewEntity {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub fetched_at: i64,
}

pub async fn get_by_url(
    executor: impl SqliteExecutor<'_>,
    url: &str,
) -> Result<Option<LinkPreviewEntity>, sqlx::Error> {
    sqlx::query_as::<_, LinkPreviewEntity>(
        "SELECT url, title, description, image_url, fetched_at FROM link_previews WHERE url = ? LIMIT 1"
    )
    .bind(url)
    .fetch_optional(executor)
    .await
}

pub async fn insert(
    executor: impl SqliteExecutor<'_>,
    entity: &LinkPreviewEntity,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT OR REPLACE INTO link_previews (url, title, description, image_url, fetched_at) VALUES (?, ?, ?, ?, ?)",
        entity.url,
        entity.title,
        entity.description,
        entity.image_url,
        entity.fetched_at
    )
    .execute(executor)
    .await?;
    Ok(())
}
