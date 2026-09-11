mod db;
pub mod error;
mod keys;
pub mod openai;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use db::Database;
use error::{BackendError, ErrorKind};
use openai::{ConnectionTestRequest, ConnectionTestResult, GeneratedImage, GenerationRequest};
use serde_json::Value;
use tauri::{AppHandle, Manager, State};
use tokio_util::sync::CancellationToken;

#[derive(Default)]
struct TaskRegistry {
    tokens: Mutex<HashMap<String, CancellationToken>>,
}

fn blocking<T, F>(operation: F) -> tauri::async_runtime::JoinHandle<Result<T, BackendError>>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, BackendError> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(operation)
}

#[tauri::command]
async fn generate_images(
    app: AppHandle,
    registry: State<'_, TaskRegistry>,
    request: GenerationRequest,
) -> Result<Vec<GeneratedImage>, BackendError> {
    let request_id = request.request_id.clone();
    let token = CancellationToken::new();
    registry
        .tokens
        .lock()
        .unwrap()
        .insert(request_id.clone(), token.clone());
    let written: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let handle = {
        let app = app.clone();
        let written = written.clone();
        tauri::async_runtime::spawn(async move { openai::generate(&app, &request, written).await })
    };
    let mut handle = handle;
    let result = tokio::select! {
        () = token.cancelled() => {
            handle.abort();
            let _ = (&mut handle).await;
            Err(BackendError::cancelled())
        }
        joined = &mut handle => match joined {
            Ok(result) => result,
            Err(_) => Err(BackendError::new(
                ErrorKind::Server,
                "The generation task ended unexpectedly.",
            )),
        },
    };
    registry.tokens.lock().unwrap().remove(&request_id);
    if result.is_err() {
        for path in written.lock().unwrap().drain(..) {
            let _ = std::fs::remove_file(path);
        }
    }
    result
}

#[tauri::command]
fn cancel_generation(registry: State<'_, TaskRegistry>, request_id: String) {
    if let Some(token) = registry.tokens.lock().unwrap().get(&request_id) {
        token.cancel();
    }
}

#[tauri::command]
async fn test_connection(
    request: ConnectionTestRequest,
) -> Result<ConnectionTestResult, BackendError> {
    openai::test_connection(request).await
}

#[tauri::command]
async fn workspace_load(
    database: State<'_, Arc<Database>>,
) -> Result<Option<Value>, BackendError> {
    let database = database.inner().clone();
    blocking(move || database.load())
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn workspace_save_state(
    database: State<'_, Arc<Database>>,
    state: Value,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.save_state(&state))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn providers_upsert(
    database: State<'_, Arc<Database>>,
    providers: Vec<Value>,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.upsert_providers(&providers))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn providers_remove(
    database: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || {
        keys::delete(&id)?;
        database.remove_provider(&id)
    })
    .await
    .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn history_add(
    database: State<'_, Arc<Database>>,
    images: Vec<Value>,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.add_history(&images))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Moves a history entry to the trash; its files stay on disk.
#[tauri::command]
async fn history_remove(
    database: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.set_deleted(&id, true))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Restores a trashed history entry back to the main list.
#[tauri::command]
async fn history_restore(
    database: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.set_deleted(&id, false))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Permanently deletes one trashed entry including its image files.
#[tauri::command]
async fn history_purge(
    database: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.purge_history(&id))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Permanently deletes every trashed entry; returns how many were removed.
#[tauri::command]
async fn history_empty_trash(
    database: State<'_, Arc<Database>>,
) -> Result<usize, BackendError> {
    let database = database.inner().clone();
    blocking(move || database.empty_trash())
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Lists trashed history entries, most recently deleted first.
#[tauri::command]
async fn history_deleted(
    database: State<'_, Arc<Database>>,
) -> Result<Value, BackendError> {
    let database = database.inner().clone();
    blocking(move || database.list_deleted())
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn history_set_favorite(
    database: State<'_, Arc<Database>>,
    id: String,
    favorite: bool,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.set_favorite(&id, favorite))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn presets_upsert(
    database: State<'_, Arc<Database>>,
    presets: Vec<Value>,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.upsert_presets(&presets))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn presets_remove(
    database: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.remove_preset(&id))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn tasks_list(database: State<'_, Arc<Database>>) -> Result<Value, BackendError> {
    let database = database.inner().clone();
    blocking(move || database.list_tasks())
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn tasks_upsert(
    database: State<'_, Arc<Database>>,
    task: Value,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.upsert_task(&task))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
async fn workspace_import(
    database: State<'_, Arc<Database>>,
    workspace: Value,
) -> Result<(), BackendError> {
    let database = database.inner().clone();
    blocking(move || database.import(&workspace))
        .await
        .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Opens Explorer with the image file selected.
#[tauri::command]
fn open_in_folder(path: String) -> Result<(), BackendError> {
    let target = PathBuf::from(&path);
    if !target.exists() {
        return Err(BackendError::new(
            ErrorKind::Config,
            "The image file no longer exists.",
        ));
    }
    std::process::Command::new("explorer.exe")
        .arg(format!("/select,{}", target.to_string_lossy()))
        .spawn()
        .map(|_| ())
        .map_err(|error| {
            BackendError::new(
                ErrorKind::Server,
                format!("Could not open the folder: {error}"),
            )
        })
}

/// Native save-as dialog; copies the image to the chosen location.
#[tauri::command]
async fn export_image(
    path: String,
    suggested_name: String,
) -> Result<Option<String>, BackendError> {
    blocking(move || {
        let source = PathBuf::from(&path);
        if !source.exists() {
            return Err(BackendError::new(
                ErrorKind::Config,
                "The image file no longer exists.",
            ));
        }
        let target = rfd::FileDialog::new()
            .set_file_name(&suggested_name)
            .save_file();
        match target {
            Some(target) => std::fs::copy(&source, &target)
                .map(|_| Some(target.to_string_lossy().into_owned()))
                .map_err(|error| {
                    BackendError::new(
                        ErrorKind::Server,
                        format!("Could not export the image: {error}"),
                    )
                }),
            None => Ok(None),
        }
    })
    .await
    .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

/// Copies a locally stored image file onto the system clipboard.
#[tauri::command]
async fn copy_image(path: String) -> Result<(), BackendError> {
    blocking(move || {
        let target = PathBuf::from(&path);
        if !target.exists() {
            return Err(BackendError::new(
                ErrorKind::Config,
                "The image file no longer exists.",
            ));
        }
        let bytes = std::fs::read(&target).map_err(|error| {
            BackendError::new(
                ErrorKind::Server,
                format!("Could not read the image file: {error}"),
            )
        })?;
        let decoded = image::load_from_memory(&bytes)
            .map_err(|_| {
                BackendError::new(
                    ErrorKind::InvalidResponse,
                    "The image file could not be decoded.",
                )
            })?
            .to_rgba8();
        let (width, height) = decoded.dimensions();
        let mut clipboard = arboard::Clipboard::new().map_err(|error| {
            BackendError::new(
                ErrorKind::Server,
                format!("Clipboard is unavailable: {error}"),
            )
        })?;
        clipboard
            .set_image(arboard::ImageData {
                width: width as usize,
                height: height as usize,
                bytes: std::borrow::Cow::Owned(decoded.into_raw()),
            })
            .map_err(|error| {
                BackendError::new(
                    ErrorKind::Server,
                    format!("Could not copy the image: {error}"),
                )
            })
    })
    .await
    .map_err(|error| BackendError::new(ErrorKind::Server, error.to_string()))?
}

#[tauri::command]
fn store_api_key(provider_id: String, key: String) -> Result<(), BackendError> {
    keys::store(&provider_id, &key)
}

#[tauri::command]
fn delete_api_key(provider_id: String) -> Result<(), BackendError> {
    keys::delete(&provider_id)
}

#[tauri::command]
fn has_api_key(provider_id: String) -> bool {
    keys::has(&provider_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let database = Database::open(&dir.join("imagen.db"))?;
            database.mark_stale_tasks()?;
            app.manage(Arc::new(database));
            Ok(())
        })
        .manage(TaskRegistry::default())
        .invoke_handler(tauri::generate_handler![
            generate_images,
            cancel_generation,
            test_connection,
            workspace_load,
            workspace_save_state,
            providers_upsert,
            providers_remove,
            history_add,
            history_remove,
            history_restore,
            history_purge,
            history_empty_trash,
            history_deleted,
            history_set_favorite,
            presets_upsert,
            presets_remove,
            tasks_list,
            tasks_upsert,
            workspace_import,
            open_in_folder,
            export_image,
            copy_image,
            store_api_key,
            delete_api_key,
            has_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Imagen");
}
