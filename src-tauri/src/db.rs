use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection};
use serde_json::{json, Value};

use crate::error::{BackendError, ErrorKind};
use crate::keys;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    base_url TEXT NOT NULL DEFAULT '',
    models TEXT NOT NULL DEFAULT '[]',
    default_model TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'untested',
    sort_index INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS history (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    src TEXT NOT NULL DEFAULT '',
    path TEXT,
    thumb TEXT,
    prompt TEXT NOT NULL DEFAULT '',
    params TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT '',
    favorite INTEGER NOT NULL DEFAULT 0,
    width INTEGER NOT NULL DEFAULT 0,
    height INTEGER NOT NULL DEFAULT 0,
    seed TEXT NOT NULL DEFAULT '',
    filter TEXT NOT NULL DEFAULT 'none',
    position TEXT NOT NULL DEFAULT '50% 50%',
    batch_id TEXT NOT NULL DEFAULT '',
    refs TEXT NOT NULL DEFAULT '[]'
);
CREATE INDEX IF NOT EXISTS history_created ON history (created_at DESC);
CREATE TABLE IF NOT EXISTS presets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    prompt TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT 'My presets',
    params TEXT NOT NULL DEFAULT '{}',
    image TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    provider_id TEXT NOT NULL DEFAULT '',
    model TEXT NOT NULL DEFAULT '',
    prompt TEXT NOT NULL DEFAULT '',
    params TEXT NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending',
    error_kind TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT '',
    finished_at TEXT,
    result_count INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS tasks_created ON tasks (created_at DESC);
";

fn db_error(error: impl std::fmt::Display) -> BackendError {
    BackendError::new(
        ErrorKind::Server,
        format!("Local storage failed: {error}"),
    )
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, BackendError> {
        let conn = Connection::open(path).map_err(db_error)?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(db_error)?;
        conn.execute_batch(SCHEMA).map_err(db_error)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn get_meta(conn: &Connection, key: &str) -> Option<Value> {
        conn.query_row("SELECT value FROM meta WHERE key = ?1", params![key], |row| {
            row.get::<_, String>(0)
        })
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
    }

    fn set_meta(conn: &Connection, key: &str, value: &Value) -> Result<(), BackendError> {
        conn.execute(
            "INSERT INTO meta (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, serde_json::to_string(value).map_err(db_error)?],
        )
        .map(|_| ())
        .map_err(db_error)
    }

    fn list_providers(conn: &Connection) -> Result<Value, BackendError> {
        let mut statement = conn
            .prepare(
                "SELECT id, name, kind, description, base_url, models,
                        default_model, status FROM providers ORDER BY sort_index",
            )
            .map_err(db_error)?;
        let rows = statement
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let models: String = row.get(5)?;
                Ok(json!({
                    "id": id,
                    "name": row.get::<_, String>(1)?,
                    "kind": row.get::<_, String>(2)?,
                    "description": row.get::<_, String>(3)?,
                    "baseUrl": row.get::<_, String>(4)?,
                    "models": serde_json::from_str::<Value>(&models)
                        .unwrap_or_else(|_| json!([])),
                    "defaultModel": row.get::<_, String>(6)?,
                    "status": row.get::<_, String>(7)?,
                    "hasKey": keys::has(&id),
                }))
            })
            .map_err(db_error)?;
        let mut providers = Vec::new();
        for row in rows {
            providers.push(row.map_err(db_error)?);
        }
        Ok(Value::Array(providers))
    }

    fn list_history(conn: &Connection) -> Result<Value, BackendError> {
        let mut statement = conn
            .prepare(
                "SELECT id, title, src, path, thumb, prompt, params, created_at,
                        favorite, width, height, seed, filter, position, batch_id, refs
                 FROM history ORDER BY created_at DESC",
            )
            .map_err(db_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(json!({
                    "id": row.get::<_, String>(0)?,
                    "title": row.get::<_, String>(1)?,
                    "src": row.get::<_, String>(2)?,
                    "path": row.get::<_, Option<String>>(3)?,
                    "thumb": row.get::<_, Option<String>>(4)?,
                    "prompt": row.get::<_, String>(5)?,
                    "params": serde_json::from_str::<Value>(
                        &row.get::<_, String>(6)?
                    )
                    .unwrap_or_else(|_| json!({})),
                    "createdAt": row.get::<_, String>(7)?,
                    "favorite": row.get::<_, i64>(8)? != 0,
                    "width": row.get::<_, i64>(9)?,
                    "height": row.get::<_, i64>(10)?,
                    "seed": row.get::<_, String>(11)?,
                    "filter": row.get::<_, String>(12)?,
                    "position": row.get::<_, String>(13)?,
                    "batchId": row.get::<_, String>(14)?,
                    "references": serde_json::from_str::<Value>(
                        &row.get::<_, String>(15)?
                    )
                    .unwrap_or_else(|_| json!([])),
                }))
            })
            .map_err(db_error)?;
        let mut history = Vec::new();
        for row in rows {
            history.push(row.map_err(db_error)?);
        }
        Ok(Value::Array(history))
    }

    fn list_presets(conn: &Connection) -> Result<Value, BackendError> {
        let mut statement = conn
            .prepare(
                "SELECT id, name, description, prompt, category, params, image
                 FROM presets ORDER BY rowid",
            )
            .map_err(db_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(json!({
                    "id": row.get::<_, String>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "description": row.get::<_, String>(2)?,
                    "prompt": row.get::<_, String>(3)?,
                    "category": row.get::<_, String>(4)?,
                    "params": serde_json::from_str::<Value>(
                        &row.get::<_, String>(5)?
                    )
                    .unwrap_or_else(|_| json!({})),
                    "image": row.get::<_, String>(6)?,
                }))
            })
            .map_err(db_error)?;
        let mut presets = Vec::new();
        for row in rows {
            presets.push(row.map_err(db_error)?);
        }
        Ok(Value::Array(presets))
    }

    pub fn load(&self) -> Result<Option<Value>, BackendError> {
        let conn = self.conn.lock().unwrap();
        let providers = Self::list_providers(&conn)?;
        let history = Self::list_history(&conn)?;
        let presets = Self::list_presets(&conn)?;
        let empty = providers
            .as_array()
            .map(|list| list.is_empty())
            .unwrap_or(true)
            && history
                .as_array()
                .map(|list| list.is_empty())
                .unwrap_or(true)
            && presets.as_array().map(|list| list.is_empty()).unwrap_or(true)
            && Self::get_meta(&conn, "prompt").is_none();
        if empty {
            return Ok(None);
        }
        Ok(Some(json!({
            "version": 1,
            "prompt": Self::get_meta(&conn, "prompt"),
            "params": Self::get_meta(&conn, "params"),
            "references": Self::get_meta(&conn, "references"),
            "preferMock": Self::get_meta(&conn, "preferMock"),
            "selectedId": Self::get_meta(&conn, "selectedId"),
            "resultIds": Self::get_meta(&conn, "resultIds"),
            "providers": providers,
            "history": history,
            "presets": presets,
        })))
    }

    pub fn save_state(&self, state: &Value) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        for key in [
            "prompt",
            "params",
            "references",
            "preferMock",
            "selectedId",
            "resultIds",
        ] {
            if let Some(value) = state.get(key) {
                Self::set_meta(&conn, key, value)?;
            }
        }
        Ok(())
    }

    pub fn upsert_providers(&self, providers: &[Value]) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        let existing: Vec<String> = conn
            .prepare("SELECT id FROM providers")
            .map_err(db_error)?
            .query_map([], |row| row.get(0))
            .map_err(db_error)?
            .collect::<Result<_, _>>()
            .map_err(db_error)?;
        let incoming: Vec<String> = providers
            .iter()
            .filter_map(|provider| provider.get("id").and_then(Value::as_str))
            .map(str::to_owned)
            .collect();
        for stale in existing
            .iter()
            .filter(|id| !incoming.contains(id))
        {
            conn.execute("DELETE FROM providers WHERE id = ?1", params![stale])
                .map_err(db_error)?;
        }
        let mut statement = conn
            .prepare(
                "INSERT INTO providers
                 (id, name, kind, description, base_url, models, default_model, status, sort_index)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                 ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    kind = excluded.kind,
                    description = excluded.description,
                    base_url = excluded.base_url,
                    models = excluded.models,
                    default_model = excluded.default_model,
                    status = excluded.status,
                    sort_index = excluded.sort_index",
            )
            .map_err(db_error)?;
        for (index, provider) in providers.iter().enumerate() {
            statement
                .execute(params![
                    provider.get("id").and_then(Value::as_str).unwrap_or(""),
                    provider.get("name").and_then(Value::as_str).unwrap_or(""),
                    provider.get("kind").and_then(Value::as_str).unwrap_or("custom"),
                    provider
                        .get("description")
                        .and_then(Value::as_str)
                        .unwrap_or(""),
                    provider.get("baseUrl").and_then(Value::as_str).unwrap_or(""),
                    provider
                        .get("models")
                        .map(|value| serde_json::to_string(value).unwrap_or_default())
                        .unwrap_or_else(|| "[]".into()),
                    provider
                        .get("defaultModel")
                        .and_then(Value::as_str)
                        .unwrap_or(""),
                    provider
                        .get("status")
                        .and_then(Value::as_str)
                        .unwrap_or("untested"),
                    index as i64,
                ])
                .map_err(db_error)?;
        }
        Ok(())
    }

    pub fn remove_provider(&self, id: &str) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM providers WHERE id = ?1", params![id])
            .map(|_| ())
            .map_err(db_error)
    }

    fn insert_history(conn: &Connection, image: &Value) -> Result<(), BackendError> {
        conn.execute(
            "INSERT INTO history
             (id, title, src, path, thumb, prompt, params, created_at, favorite,
              width, height, seed, filter, position, batch_id, refs)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                src = excluded.src,
                path = excluded.path,
                thumb = excluded.thumb,
                prompt = excluded.prompt,
                params = excluded.params,
                created_at = excluded.created_at,
                favorite = excluded.favorite,
                width = excluded.width,
                height = excluded.height,
                seed = excluded.seed,
                filter = excluded.filter,
                position = excluded.position,
                batch_id = excluded.batch_id,
                refs = excluded.refs",
            params![
                image.get("id").and_then(Value::as_str).unwrap_or(""),
                image.get("title").and_then(Value::as_str).unwrap_or(""),
                image.get("src").and_then(Value::as_str).unwrap_or(""),
                image.get("path").and_then(Value::as_str),
                image.get("thumb").and_then(Value::as_str),
                image.get("prompt").and_then(Value::as_str).unwrap_or(""),
                image
                    .get("params")
                    .map(|value| serde_json::to_string(value).unwrap_or_default())
                    .unwrap_or_else(|| "{}".into()),
                image
                    .get("createdAt")
                    .and_then(Value::as_str)
                    .unwrap_or(""),
                image
                    .get("favorite")
                    .and_then(Value::as_bool)
                    .unwrap_or(false) as i64,
                image.get("width").and_then(Value::as_i64).unwrap_or(0),
                image.get("height").and_then(Value::as_i64).unwrap_or(0),
                image.get("seed").and_then(Value::as_str).unwrap_or(""),
                image
                    .get("filter")
                    .and_then(Value::as_str)
                    .unwrap_or("none"),
                image
                    .get("position")
                    .and_then(Value::as_str)
                    .unwrap_or("50% 50%"),
                image.get("batchId").and_then(Value::as_str).unwrap_or(""),
                image
                    .get("references")
                    .map(|value| serde_json::to_string(value).unwrap_or_default())
                    .unwrap_or_else(|| "[]".into()),
            ],
        )
        .map(|_| ())
        .map_err(db_error)
    }

    pub fn add_history(&self, images: &[Value]) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        for image in images {
            Self::insert_history(&conn, image)?;
        }
        Ok(())
    }

    pub fn set_favorite(&self, id: &str, favorite: bool) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE history SET favorite = ?2 WHERE id = ?1",
            params![id, favorite as i64],
        )
        .map(|_| ())
        .map_err(db_error)
    }

    fn upsert_preset(conn: &Connection, preset: &Value) -> Result<(), BackendError> {
        conn.execute(
            "INSERT INTO presets (id, name, description, prompt, category, params, image)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                prompt = excluded.prompt,
                category = excluded.category,
                params = excluded.params,
                image = excluded.image",
            params![
                preset.get("id").and_then(Value::as_str).unwrap_or(""),
                preset.get("name").and_then(Value::as_str).unwrap_or(""),
                preset
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or(""),
                preset.get("prompt").and_then(Value::as_str).unwrap_or(""),
                preset
                    .get("category")
                    .and_then(Value::as_str)
                    .unwrap_or("My presets"),
                preset
                    .get("params")
                    .map(|value| serde_json::to_string(value).unwrap_or_default())
                    .unwrap_or_else(|| "{}".into()),
                preset.get("image").and_then(Value::as_str).unwrap_or(""),
            ],
        )
        .map(|_| ())
        .map_err(db_error)
    }

    pub fn upsert_presets(&self, presets: &[Value]) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        let existing: Vec<String> = conn
            .prepare("SELECT id FROM presets")
            .map_err(db_error)?
            .query_map([], |row| row.get(0))
            .map_err(db_error)?
            .collect::<Result<_, _>>()
            .map_err(db_error)?;
        let incoming: Vec<String> = presets
            .iter()
            .filter_map(|preset| preset.get("id").and_then(Value::as_str))
            .map(str::to_owned)
            .collect();
        for stale in existing.iter().filter(|id| !incoming.contains(id)) {
            conn.execute("DELETE FROM presets WHERE id = ?1", params![stale])
                .map_err(db_error)?;
        }
        for preset in presets {
            Self::upsert_preset(&conn, preset)?;
        }
        Ok(())
    }

    pub fn remove_preset(&self, id: &str) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM presets WHERE id = ?1", params![id])
            .map(|_| ())
            .map_err(db_error)
    }

    pub fn upsert_task(&self, task: &Value) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tasks
             (id, provider_id, model, prompt, params, status, error_kind,
              error_message, created_at, finished_at, result_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(id) DO UPDATE SET
                provider_id = excluded.provider_id,
                model = excluded.model,
                prompt = excluded.prompt,
                params = excluded.params,
                status = excluded.status,
                error_kind = excluded.error_kind,
                error_message = excluded.error_message,
                created_at = excluded.created_at,
                finished_at = excluded.finished_at,
                result_count = excluded.result_count",
            params![
                task.get("id").and_then(Value::as_str).unwrap_or(""),
                task.get("providerId").and_then(Value::as_str).unwrap_or(""),
                task.get("model").and_then(Value::as_str).unwrap_or(""),
                task.get("prompt").and_then(Value::as_str).unwrap_or(""),
                task.get("params")
                    .map(|value| serde_json::to_string(value).unwrap_or_default())
                    .unwrap_or_else(|| "{}".into()),
                task.get("status").and_then(Value::as_str).unwrap_or("pending"),
                task.get("errorKind").and_then(Value::as_str),
                task.get("errorMessage").and_then(Value::as_str),
                task.get("createdAt").and_then(Value::as_str).unwrap_or(""),
                task.get("finishedAt").and_then(Value::as_str),
                task.get("resultCount").and_then(Value::as_i64).unwrap_or(0),
            ],
        )
        .map_err(db_error)?;
        // Keep the table bounded to the newest 500 tasks.
        conn.execute(
            "DELETE FROM tasks WHERE id NOT IN
             (SELECT id FROM tasks ORDER BY created_at DESC LIMIT 500)",
            [],
        )
        .map_err(db_error)?;
        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Value, BackendError> {
        let conn = self.conn.lock().unwrap();
        let mut statement = conn
            .prepare(
                "SELECT id, provider_id, model, prompt, params, status,
                        error_kind, error_message, created_at, finished_at,
                        result_count
                 FROM tasks ORDER BY created_at DESC LIMIT 100",
            )
            .map_err(db_error)?;
        let rows = statement
            .query_map([], |row| {
                Ok(json!({
                    "id": row.get::<_, String>(0)?,
                    "providerId": row.get::<_, String>(1)?,
                    "model": row.get::<_, String>(2)?,
                    "prompt": row.get::<_, String>(3)?,
                    "params": serde_json::from_str::<Value>(
                        &row.get::<_, String>(4)?
                    )
                    .unwrap_or_else(|_| json!({})),
                    "status": row.get::<_, String>(5)?,
                    "errorKind": row.get::<_, Option<String>>(6)?,
                    "errorMessage": row.get::<_, Option<String>>(7)?,
                    "createdAt": row.get::<_, String>(8)?,
                    "finishedAt": row.get::<_, Option<String>>(9)?,
                    "resultCount": row.get::<_, i64>(10)?,
                }))
            })
            .map_err(db_error)?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row.map_err(db_error)?);
        }
        Ok(Value::Array(tasks))
    }

    /// Tasks left pending/running when the app quit are marked interrupted.
    pub fn mark_stale_tasks(&self) -> Result<(), BackendError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET status = 'interrupted'
             WHERE status IN ('pending', 'running')",
            [],
        )
        .map(|_| ())
        .map_err(db_error)
    }

    /// Full import used for the one-time IndexedDB migration.
    pub fn import(&self, workspace: &Value) -> Result<(), BackendError> {
        self.save_state(workspace)?;
        if let Some(providers) = workspace.get("providers").and_then(Value::as_array)
        {
            self.upsert_providers(providers)?;
        }
        if let Some(history) = workspace.get("history").and_then(Value::as_array) {
            self.add_history(history)?;
        }
        if let Some(presets) = workspace.get("presets").and_then(Value::as_array) {
            self.upsert_presets(presets)?;
        }
        Ok(())
    }
}
