use actix_web::{web, HttpResponse, Error};
use crate::storage::local::get_frames_directory;

pub async fn get_preview(
    path: web::Path<(String, usize)>,
) -> Result<HttpResponse, Error> {
    let (job_id, index) = path.into_inner();
    let frames_dir = get_frames_directory(&job_id);
    
    // List files and get the one at the specified index
    let mut files: Vec<String> = std::fs::read_dir(&frames_dir)
        .map_err(|_| actix_web::error::ErrorNotFound("Job not found"))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name()?.to_str().map(|s| s.to_string())
            })
        })
        .collect();
    
    // Natural sort
    files.sort_by(|a, b| alphanumeric_sort::compare_str(a, b));
    
    if index >= files.len() {
        return Err(actix_web::error::ErrorNotFound("Index out of range"));
    }
    
    let filepath = frames_dir.join(&files[index]);
    
    if !filepath.exists() {
        return Err(actix_web::error::ErrorNotFound("File not found"));
    }
    
    let file_data = std::fs::read(&filepath)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    // Determine content type from extension
    let content_type = if filepath.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .as_deref() == Some("png") {
        "image/png"
    } else if filepath.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .as_deref() == Some("webp") {
        "image/webp"
    } else {
        "image/jpeg"
    };
    
    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .body(file_data))
}
