use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Project {
    pub name: String,
    pub root_path: PathBuf,
    pub notes_relative_path: PathBuf,
    pub assets_relative_path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Settings {
    pub reverse_layout: bool,
    pub sync_provider: SyncProvider,
    pub yandex_oauth_token: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, ts_rs::TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export)]
pub enum SyncProvider {
    None,
    Yandex,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            reverse_layout: false,
            sync_provider: SyncProvider::None,
            yandex_oauth_token: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SearchQuery {
    pub body_terms: Vec<String>,
    pub negated_body_terms: Vec<String>,
    pub tag_filters: Vec<String>,
    pub negated_tag_filters: Vec<String>,
    pub name_filter: Option<String>,
    pub negated_name_filter: Option<String>,
    pub sort_by: SortBy,
    pub pinned_first: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export)]
pub enum SortBy {
    LastModified,
    CreatedAt,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            body_terms: vec![],
            negated_body_terms: vec![],
            tag_filters: vec![],
            negated_tag_filters: vec![],
            name_filter: None,
            negated_name_filter: None,
            sort_by: SortBy::LastModified,
            pinned_first: false,
        }
    }
}
