use actix_web::{web, HttpResponse, Error};
use crate::storage::local::get_output_path;

pub async fn download_video(
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let job_id = path.into_inner();
    let output_path = get_output_path(&job_id);
    
    if !output_path.exists() {
        return Err(actix_web::error::ErrorNotFound("Video not found"));
    }
    
    let file_data = std::fs::read(&output_path)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok()
        .content_type("video/mp4")
        .append_header(("Content-Disposition", format!("attachment; filename=\"timelapse_{}.mp4\"", job_id)))
        .body(file_data))
}
