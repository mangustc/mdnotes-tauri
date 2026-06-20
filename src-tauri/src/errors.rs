use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Clone, Debug, Error, ts_rs::TS)]
#[serde(tag = "type", content = "data")]
#[ts(export)]
pub enum AppError {
    #[error("File not found at: {path}")]
    FileNotFound { path: String },

    #[error("Failed to read file: {path}. Details: {message}")]
    FileNotReadable { path: String, message: String },

    #[error("Failed to write file: {path}. Details: {message}")]
    FileNotWritable { path: String, message: String },

    #[error("Cannot access project directory: {path}. Details: {message}")]
    ProjectAccess { path: String, message: String },

    #[error("Authentication failed. Please log in again.")]
    SyncAuth,

    #[error("Network error. Check internet connection.")]
    SyncNetwork,

    #[error("Sync server unavailable. Try again later.")]
    SyncServer,

    #[error("Sync data corrupted. Please reset sync.")]
    SyncState,

    #[error("Cloud storage full. Free up space.")]
    SyncQuota,

    #[error("Failed to fetch link information: {path}. Details: {message}")]
    LinkFetch { path: String, message: String },

    #[error("Database error: {message}")]
    Database { message: String },

    #[error("An unexpected error occurred: {message}")]
    Generic { message: String },
}

// Convert common standard errors into serializable AppErrors
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Generic {
            message: err.to_string(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::SyncNetwork
    }
}
