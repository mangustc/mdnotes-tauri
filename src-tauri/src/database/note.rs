use crate::models::{SearchQuery, SortBy};
use serde::{Deserialize, Serialize};
use sqlx::SqliteExecutor;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NoteEntity {
    pub id: i64,
    pub project_id: i64,
    pub uri: String,
    pub name: String,
    pub last_modified: i64,
    pub created_at: Option<i64>,
    pub tags: String,
    pub body: String,
}

pub async fn insert_note(
    executor: impl SqliteExecutor<'_>,
    note: &NoteEntity,
) -> Result<i64, sqlx::Error> {
    let id_param = if note.id == 0 { None } else { Some(note.id) };
    let result = sqlx::query!(
        "INSERT OR REPLACE INTO notes (id, project_id, uri, name, last_modified, created_at, tags, body) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id_param,
        note.project_id,
        note.uri,
        note.name,
        note.last_modified,
        note.created_at,
        note.tags,
        note.body
    )
    .execute(executor)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn delete_by_uri(
    executor: impl SqliteExecutor<'_>,
    uri: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM notes WHERE uri = ?", uri)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn get_note_by_uri(
    executor: impl SqliteExecutor<'_>,
    uri: &str,
) -> Result<Option<NoteEntity>, sqlx::Error> {
    sqlx::query_as::<_, NoteEntity>(
        "SELECT id, project_id, uri, name, last_modified, created_at, tags, body FROM notes WHERE uri = ? LIMIT 1"
    )
    .bind(uri)
    .fetch_optional(executor)
    .await
}

pub async fn get_all_tags(
    executor: impl SqliteExecutor<'_>,
    project_id: i64,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!("SELECT tags FROM notes WHERE project_id = ?", project_id)
        .fetch_all(executor)
        .await?;

    Ok(rows.into_iter().map(|r| r.tags).collect())
}

fn escape_fts_term(term: &str) -> String {
    format!("\"{}\"", term.replace('"', "\"\""))
}

pub async fn search_notes(
    executor: impl SqliteExecutor<'_>,
    query: &SearchQuery,
) -> Result<Vec<NoteEntity>, sqlx::Error> {
    search_notes_paged(executor, query, None, None).await
}

pub async fn search_notes_paged(
    executor: impl SqliteExecutor<'_>,
    query: &SearchQuery,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<NoteEntity>, sqlx::Error> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, project_id, uri, name, last_modified, created_at, tags, body FROM notes WHERE 1=1"
    );

    let mut fts_terms = Vec::new();
    for term in &query.body_terms {
        if !term.trim().is_empty() {
            fts_terms.push(escape_fts_term(term));
        }
    }
    for term in &query.negated_body_terms {
        if !term.trim().is_empty() {
            fts_terms.push(format!("NOT {}", escape_fts_term(term)));
        }
    }

    if !fts_terms.is_empty() {
        let fts_query = fts_terms.join(" ");
        query_builder.push(" AND id IN (SELECT rowid FROM notes_fts WHERE notes_fts MATCH ");
        query_builder.push_bind(fts_query);
        query_builder.push(")");
    }

    if let Some(ref name) = query.name_filter {
        if !name.trim().is_empty() {
            query_builder.push(" AND name LIKE ");
            query_builder.push_bind(format!("%{}%", name));
        }
    }
    if let Some(ref negated_name) = query.negated_name_filter {
        if !negated_name.trim().is_empty() {
            query_builder.push(" AND name NOT LIKE ");
            query_builder.push_bind(format!("%{}%", negated_name));
        }
    }

    for tag in &query.tag_filters {
        if !tag.trim().is_empty() {
            query_builder.push(" AND tags LIKE ");
            query_builder.push_bind(format!("%{}%", tag));
        }
    }
    for tag in &query.negated_tag_filters {
        if !tag.trim().is_empty() {
            query_builder.push(" AND tags NOT LIKE ");
            query_builder.push_bind(format!("%{}%", tag));
        }
    }

    let mut order_clauses = Vec::new();
    if query.pinned_first {
        order_clauses.push("CASE WHEN tags LIKE '%pinned%' THEN 0 ELSE 1 END ASC".to_string());
    }

    let sort_col = match query.sort_by {
        SortBy::LastModified => "last_modified DESC",
        SortBy::CreatedAt => "created_at DESC",
    };
    order_clauses.push(sort_col.to_string());

    query_builder.push(" ORDER BY ");
    query_builder.push(order_clauses.join(", "));

    if let Some(l) = limit {
        query_builder.push(" LIMIT ");
        query_builder.push_bind(l);
    }
    if let Some(o) = offset {
        query_builder.push(" OFFSET ");
        query_builder.push_bind(o);
    }

    let sql_query = query_builder.build_query_as::<NoteEntity>();
    sql_query.fetch_all(executor).await
}
