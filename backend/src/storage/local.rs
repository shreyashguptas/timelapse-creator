use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

const TEMP_BASE_DIR: &str = "/tmp/timelapse";

pub fn ensure_job_directory(job_id: &str) -> Result<PathBuf> {
    let job_dir = Path::new(TEMP_BASE_DIR).join(job_id);
    fs::create_dir_all(&job_dir)
        .context("Failed to create job directory")?;
    
    let frames_dir = job_dir.join("frames");
    fs::create_dir_all(&frames_dir)
        .context("Failed to create frames directory")?;
    
    Ok(job_dir)
}

pub fn get_frames_directory(job_id: &str) -> PathBuf {
    Path::new(TEMP_BASE_DIR).join(job_id).join("frames")
}

pub fn get_output_path(job_id: &str) -> PathBuf {
    Path::new(TEMP_BASE_DIR).join(job_id).join("output.mp4")
}

#[allow(dead_code)]
pub fn cleanup_job(job_id: &str) -> Result<()> {
    let job_dir = Path::new(TEMP_BASE_DIR).join(job_id);
    if job_dir.exists() {
        fs::remove_dir_all(&job_dir)
            .context("Failed to cleanup job directory")?;
    }
    Ok(())
}

pub fn list_image_files(job_id: &str) -> Result<Vec<String>> {
    let frames_dir = get_frames_directory(job_id);
    let mut files = Vec::new();
    
    if !frames_dir.exists() {
        return Ok(files);
    }
    
    let entries = fs::read_dir(&frames_dir)
        .context("Failed to read frames directory")?;
    
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
    files.sort_by(|a, b| {
        alphanumeric_sort::compare_str(a, b)
    });
    
    Ok(files)
}
