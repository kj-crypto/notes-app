mod ogmeta;
mod data_handler;
mod link_opener;
mod utils;
use std::sync::Mutex;
use std::collections::HashMap;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![data_handler::get_data, data_handler::delete_data, data_handler::upsert_data, ogmeta::fetch_og_meta, link_opener::open_url])
    .setup(|app| {
      #[cfg(debug_assertions)]
      {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
        if let Some(window) = app.get_webview_window("main") {
            window.open_devtools();
        }
      };
      app.manage(data_handler::AppState {
        data: Mutex::new(HashMap::new()),
        last_id: Mutex::new(0),
      });
      if let Err(e) = utils::http_client::init_global_network_manager() {
        eprintln!("Failed to initialize global network manager: {}", e);
        return Err(e.into())
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
