use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_storage_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("timelapse-jobs")
}

pub fn ensure_job_directory(app: &AppHandle, job_id: &str) -> Result<PathBuf> {
    let job_dir = get_storage_dir(app).join(job_id);
    fs::create_dir_all(&job_dir).context("Failed to create job directory")?;

    let frames_dir = job_dir.join("frames");
    fs::create_dir_all(&frames_dir).context("Failed to create frames directory")?;

    Ok(job_dir)
}

pub fn get_frames_directory(app: &AppHandle, job_id: &str) -> PathBuf {
    get_storage_dir(app).join(job_id).join("frames")
}

pub fn get_output_path(app: &AppHandle, job_id: &str) -> PathBuf {
    get_storage_dir(app).join(job_id).join("output.mp4")
}

pub fn cleanup_job(app: &AppHandle, job_id: &str) -> Result<()> {
    let job_dir = get_storage_dir(app).join(job_id);
    if job_dir.exists() {
        fs::remove_dir_all(&job_dir).context("Failed to cleanup job directory")?;
    }
    Ok(())
}

pub fn list_image_files(app: &AppHandle, job_id: &str) -> Result<Vec<String>> {
    let frames_dir = get_frames_directory(app, job_id);
    let mut files = Vec::new();

    if !frames_dir.exists() {
        return Ok(files);
    }

    let entries = fs::read_dir(&frames_dir).context("Failed to read frames directory")?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                    if let Some(filename) = path.file_name() {
                        files.push(filename.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    // Natural sort filenames
    files.sort_by(|a, b| alphanumeric_sort::compare_str(a, b));

    Ok(files)
}
