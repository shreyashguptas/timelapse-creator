mod commands;
mod models;
mod storage;
mod video;

use std::collections::HashMap;
use std::sync::Mutex;

use models::JobStatusType;

pub type JobStore = Mutex<HashMap<String, JobStatusType>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(JobStore::new(HashMap::new()))
        .invoke_handler(tauri::generate_handler![
            commands::upload::upload_images,
            commands::preview::get_preview,
            commands::timelapse::create_timelapse,
            commands::job_status::get_job_status,
            commands::download::save_video,
            commands::download::get_video_data,
            commands::cleanup::cleanup_job,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
