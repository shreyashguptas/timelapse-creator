use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs;
use tauri::AppHandle;

use crate::storage::get_output_path;

#[tauri::command]
pub async fn save_video(
    app: AppHandle,
    job_id: String,
    save_path: String,
) -> Result<bool, String> {
    let output_path = get_output_path(&app, &job_id);

    if !output_path.exists() {
        return Err("Video not found".to_string());
    }

    fs::copy(&output_path, &save_path).map_err(|e| format!("Failed to save video: {}", e))?;

    Ok(true)
}

#[tauri::command]
pub async fn get_video_data(app: AppHandle, job_id: String) -> Result<String, String> {
    let output_path = get_output_path(&app, &job_id);

    if !output_path.exists() {
        return Err("Video not found".to_string());
    }

    let video_data = fs::read(&output_path).map_err(|e| format!("Failed to read video: {}", e))?;

    // Return as data URL
    let base64_data = STANDARD.encode(&video_data);
    Ok(format!("data:video/mp4;base64,{}", base64_data))
}
