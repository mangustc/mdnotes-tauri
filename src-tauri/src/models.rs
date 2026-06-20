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
