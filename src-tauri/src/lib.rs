mod database;
mod errors;
mod models;

use std::fs;

use reqwest::Client;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tauri::{Manager, State};
use tokio::sync::RwLock;

use crate::{
    errors::AppError,
    models::{Project, Settings},
};

pub struct AppState {
    pub db: SqlitePool,
    pub http_client: Client,
    pub active_project: RwLock<Option<Project>>,
    pub settings: RwLock<Settings>,
}

async fn initialize_app_state(app: &tauri::App) -> Result<AppState, Box<dyn std::error::Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("notes.db");

    let connect_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations").run(&db).await?;

    let http_client = Client::builder().user_agent("MDNotes-Tauri/1.0").build()?;

    let active_project = None;
    let settings = Settings::default();

    Ok(AppState {
        db,
        http_client,
        active_project: RwLock::new(active_project),
        settings: RwLock::new(settings),
    })
}

#[tauri::command]
async fn read(state: State<'_, AppState>) -> Result<Settings, AppError> {
    let settings_guard = state.settings.read().await;
    Ok(settings_guard.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = tauri::async_runtime::block_on(async {
                initialize_app_state(app)
                    .await
                    .expect("Failed to initialize database and HTTP clients")
            });

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
