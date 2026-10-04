use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::path::{PathBuf};
use std::env;


#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Note,
    Link,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub r#type: Type, // 'type' is a Rust keyword, use raw identifier
    pub data: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

pub type DataId = u64;
pub type DataMap = HashMap<DataId, Data>;

pub static CACHED_DATA: Lazy<Mutex<DataMap>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static LAST_ID: Lazy<Mutex<DataId>> = Lazy::new(|| Mutex::new(0));

#[tauri::command]
pub async fn get_data() -> Result<DataMap, String> {
    load_data();
    let data = CACHED_DATA.lock().unwrap();
    Ok(data.clone())
}

fn load_data() {
    let mut data = CACHED_DATA.lock().unwrap();
    let mut last_id = LAST_ID.lock().unwrap();
    let file = get_data_file();
    if data.is_empty() {
        if std::fs::metadata(&file).is_err() {
            *data = HashMap::new();
        } else {
            *data = serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
        }
    }
    *last_id = *data.keys().max().unwrap_or(&0);
}

fn save_data(data: &DataMap) {
    let file = get_data_file();
    std::fs::write(file, serde_json::to_string_pretty(data).unwrap()).unwrap();
}

#[tauri::command]
pub async fn delete_data(id: DataId) -> ApiResponse {
    let mut data = CACHED_DATA.lock().unwrap();
    let mut last_id = LAST_ID.lock().unwrap();
    if data.remove(&id).is_none() {
        return ApiResponse { status: "error".to_string(), message: "Data not found".to_string() };
    }
    save_data(&data);
    *last_id = *data.keys().max().unwrap_or(&0);
    ApiResponse { status: "success".to_string(), message: "Data deleted".to_string() }
}

#[tauri::command]
pub async fn upsert_data(id: Option<DataId>, data: Data) -> ApiResponse {
    let mut cached_data = CACHED_DATA.lock().unwrap();
    let mut last_id = LAST_ID.lock().unwrap();
    if let Some(id) = id {
        if let Some(value) = cached_data.get_mut(&id) {
            *value = data.clone();
        }
        else {
            return ApiResponse { status: "error".to_string(), message: format!("Id {} not found", id) };
        }
    }
    else {
        *last_id += 1;
        if data.r#type == Type::Link {
           for obj in cached_data.values() {
               if obj.data == data.data {
                   return ApiResponse { status: "error".to_string(), message: format!("Link {} already exists", data.data) };
               }
           }
        }
        cached_data.insert(*last_id, data.clone());
    }
    save_data(&cached_data);
    ApiResponse { status: "success".to_string(), message: "Data updated".to_string() }
}

fn get_data_file() -> PathBuf {
    const DEFAULT_DATA_FILE: &str = env!("DATA_JSON_FILENAME", "DATA_JSON_FILENAME must be set at build time (e.g., via `DATA_JSON_FILENAME=yourfile.json cargo build`).");
    let mut data_file = DEFAULT_DATA_FILE.to_string();
    match std::env::var("DATA_JSON_FILENAME") {
        Ok(env) => { if !env.is_empty() { data_file = env } },
        Err(_) => ()
    }
    // In development, use env variable
    if cfg!(debug_assertions) {
        PathBuf::from(data_file)
    } else {
        // In production, try to use the AppImage location if available
        if let Ok(appimage_path) = env::var("APPIMAGE") {
            let dir = PathBuf::from(appimage_path)
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."));
            dir.join(data_file)
        } else {
            // Fallback: use the directory of the running binary
            env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
                .join(data_file)
        }
    }
}
