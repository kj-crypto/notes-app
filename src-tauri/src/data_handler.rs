use serde::{Serialize, Deserialize};
use std::collections::HashMap;
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

pub type DataId = u64;
pub type DataMap = HashMap<DataId, Data>;

pub struct AppState {
  pub data: Mutex<DataMap>,
  pub last_id: Mutex<DataId>,
}

#[tauri::command]
pub async fn get_data(state: tauri::State<'_, AppState>) -> Result<DataMap, String> {
    let mut data = state.data.lock().unwrap();
    let mut last_id = state.last_id.lock().unwrap();
    if data.is_empty() {
        let file = get_data_file();
        *data = HashMap::new();
        if std::fs::metadata(&file).is_ok() {
            if let Ok(content) = std::fs::read_to_string(&file) {
                if let Ok(parsed) = serde_json::from_str(&content) {
                    *data = parsed;
                    *last_id = *data.keys().max().unwrap_or(&0);
                }
            }
        }
    }
    #[cfg(debug_assertions)]
    {
        println!("[DataHandler] get_data returned: {} items", data.len());
    }
    Ok(data.clone())
}

fn save_data(data: &DataMap) {
    let file = get_data_file();
    std::fs::write(file, serde_json::to_string_pretty(data).unwrap()).unwrap();
}

#[tauri::command]
pub async fn delete_data(id: DataId, state: tauri::State<'_, AppState>) -> Result<DataId, String> {
    let mut data = state.data.lock().unwrap();
    let mut last_id = state.last_id.lock().unwrap();
    if data.remove(&id).is_none() {
        return Err("Data not found".to_string());
    }
    save_data(&data);
    *last_id = *data.keys().max().unwrap_or(&0);
    #[cfg(debug_assertions)]
    {
        println!("[DataHandler] delete_data: {} items remaining", data.len());
    }
    Ok(id)
}

#[tauri::command]
pub async fn upsert_data(id: Option<DataId>, data: Data, state: tauri::State<'_, AppState>) -> Result<DataId, String> {
    let mut cached_data = state.data.lock().unwrap();
    let mut last_id = state.last_id.lock().unwrap();
    let changed_id;
    if let Some(id) = id {
        if let Some(value) = cached_data.get_mut(&id) {
            *value = data.clone();
            changed_id = id;
            #[cfg(debug_assertions)]
            {
                println!("[DataHandler] upsert_data: updated item {}", id);
            }
        }
        else {
            return Err(format!("Id {} not found", id));
        }
    }
    else {
        *last_id += 1;
        if data.r#type == Type::Link {
           for obj in cached_data.values() {
               if obj.data == data.data {
                   return Err(format!("Link {} already exists", data.data));
               }
           }
        }
        cached_data.insert(*last_id, data.clone());
        changed_id = *last_id;
        #[cfg(debug_assertions)]
        {
            println!("[DataHandler] upsert_data: created new item of id {}", *last_id);
        }
    }
    save_data(&cached_data);
    Ok(changed_id)
}

fn get_data_file() -> PathBuf {
    const DEFAULT_DATA_FILE: &str = env!("DATA_JSON_FILENAME", "DATA_JSON_FILENAME must be set at build time (e.g., via `DATA_JSON_FILENAME=yourfile.json cargo build`).");
    let mut data_file = DEFAULT_DATA_FILE.to_string();
    if let Ok(env) = std::env::var("DATA_JSON_FILENAME") {
        if !env.is_empty() {
            data_file = env;
        }
    }
    // In development, use env variable
    #[cfg(debug_assertions)]
    {
        PathBuf::from(data_file)
    }

    #[cfg(not(debug_assertions))]
    {
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
