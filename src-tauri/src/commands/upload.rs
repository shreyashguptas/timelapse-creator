use std::fs;
use std::path::Path;
use tauri::AppHandle;
use uuid::Uuid;

use crate::models::UploadResponse;
use crate::storage::{ensure_job_directory, get_frames_directory};

#[tauri::command]
pub async fn upload_images(app: AppHandle, paths: Vec<String>) -> Result<UploadResponse, String> {
    let job_id = Uuid::new_v4().to_string();

    ensure_job_directory(&app, &job_id).map_err(|e| e.to_string())?;

    let frames_dir = get_frames_directory(&app, &job_id);
    let mut file_count = 0;
    let mut filenames = Vec::new();

    for path_str in paths {
        let source_path = Path::new(&path_str);

        if !source_path.exists() {
            continue;
        }

        // Get filename and validate extension
        let filename = match source_path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };

        let ext = source_path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp") {
            continue;
        }

        // Copy file to frames directory
        let dest_path = frames_dir.join(&filename);
        if let Err(e) = fs::copy(source_path, &dest_path) {
            eprintln!("Failed to copy {}: {}", path_str, e);
            continue;
        }

        file_count += 1;
        filenames.push(filename);
    }

    if file_count == 0 {
        return Err("No valid image files found".to_string());
    }

    Ok(UploadResponse {
        job_id,
        file_count,
        filenames,
    })
}
