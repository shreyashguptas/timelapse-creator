use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs;
use tauri::AppHandle;

use crate::storage::get_frames_directory;

#[tauri::command]
pub async fn get_preview(app: AppHandle, job_id: String, index: usize) -> Result<String, String> {
    let frames_dir = get_frames_directory(&app, &job_id);

    // List files and get the one at the specified index
    let mut files: Vec<String> = fs::read_dir(&frames_dir)
        .map_err(|_| "Job not found".to_string())?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.path().file_name()?.to_str().map(|s| s.to_string()))
        })
        .collect();

    // Natural sort
    files.sort_by(|a, b| alphanumeric_sort::compare_str(a, b));

    if index >= files.len() {
        return Err("Index out of range".to_string());
    }

    let filepath = frames_dir.join(&files[index]);

    if !filepath.exists() {
        return Err("File not found".to_string());
    }

    let file_data = fs::read(&filepath).map_err(|e| format!("Failed to read file: {}", e))?;

    // Determine content type from extension
    let ext = filepath
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let mime_type = match ext.as_str() {
        "png" => "image/png",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };

    // Return as data URL
    let base64_data = STANDARD.encode(&file_data);
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}
