use serde::{Deserialize, Serialize};
use sqlx::SqliteExecutor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProjectEntity {
    pub id: i64,
    pub name: String,
    pub root_path: String,
}

pub async fn insert_project(
    executor: impl SqliteExecutor<'_>,
    project: &ProjectEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO projects (name, root_path) VALUES (?, ?) ON CONFLICT(root_path) DO NOTHING RETURNING id",
        project.name,
        project.root_path
    )
    .fetch_optional(executor)
    .await?;

    Ok(result.map(|r| r.id).unwrap_or(-1))
}

pub async fn get_project_id(
    executor: impl SqliteExecutor<'_>,
    root_path: &str,
) -> Result<Option<i64>, sqlx::Error> {
    let result = sqlx::query_scalar!(
        "SELECT id FROM projects WHERE root_path = ? LIMIT 1",
        root_path
    )
    .fetch_optional(executor)
    .await?;
    Ok(result.flatten())
}
