use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Error};
use futures_util::TryStreamExt;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use crate::models::UploadResponse;
use crate::storage::local::{ensure_job_directory, get_frames_directory};

pub async fn upload_files(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let job_id = Uuid::new_v4().to_string();
    ensure_job_directory(&job_id)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let frames_dir = get_frames_directory(&job_id);
    let mut file_count = 0;
    let mut filenames = Vec::new();
    
    // Process each file in the multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            // Validate file extension
            let ext = filename
                .rsplit('.')
                .next()
                .unwrap_or("")
                .to_lowercase();
            
            if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                continue; // Skip non-image files
            }
            
            // Sanitize filename
            let sanitized_filename = sanitize_filename::sanitize(filename);
            let filepath = frames_dir.join(&sanitized_filename);
            
            // Create file and write chunks
            let filepath_clone = filepath.clone();
            let mut file = web::block(move || File::create(&filepath_clone))
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?
                .map_err(actix_web::error::ErrorInternalServerError)?;
            
            // Write file in chunks
            while let Ok(Some(chunk)) = field.try_next().await {
                file = web::block(move || {
                    file.write_all(&chunk)?;
                    Ok::<_, std::io::Error>(file)
                })
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?
                .map_err(actix_web::error::ErrorInternalServerError)?;
            }

            file_count += 1;
            filenames.push(sanitized_filename);
        }
    }
    
    if file_count == 0 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No valid image files uploaded"
        })));
    }
    
    Ok(HttpResponse::Ok().json(UploadResponse {
        job_id,
        file_count,
        filenames,
    }))
}
