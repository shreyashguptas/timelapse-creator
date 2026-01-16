use actix_web::{web, HttpResponse, Error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::{CreateTimelapseRequest, CreateTimelapseResponse, JobStatusType};
use crate::storage::local::{get_frames_directory, get_output_path};
use crate::video::processor::create_timelapse;

// In-memory job store (in production, use Redis or database)
type JobStore = Arc<Mutex<HashMap<String, JobStatusType>>>;

pub async fn create_timelapse_handler(
    req: web::Json<CreateTimelapseRequest>,
    job_store: web::Data<JobStore>,
) -> Result<HttpResponse, Error> {
    let job_id = req.job_id.clone();
    let rotation = req.rotation;
    let fps = req.fps;
    
    // Validate rotation
    if !matches!(rotation, 0 | 90 | 180 | 270) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid rotation. Must be 0, 90, 180, or 270"
        })));
    }
    
    // Validate fps
    if fps == 0 || fps > 60 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "FPS must be between 1 and 60"
        })));
    }
    
    // Check if job exists
    let frames_dir = get_frames_directory(&job_id);
    if !frames_dir.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Job not found"
        })));
    }
    
    // Update job status to processing
    {
        let mut store = job_store.lock().unwrap();
        store.insert(job_id.clone(), JobStatusType::Processing);
    }
    
    // Spawn async task to process video
    let job_id_clone = job_id.clone();
    let job_store_clone = job_store.clone();
    
    actix_web::rt::spawn(async move {
        let frames_dir = get_frames_directory(&job_id_clone);
        let output_path = get_output_path(&job_id_clone);
        
        match create_timelapse(&job_id_clone, frames_dir, output_path, fps, rotation) {
            Ok(_) => {
                let mut store = job_store_clone.lock().unwrap();
                store.insert(job_id_clone.clone(), JobStatusType::Completed);
            }
            Err(e) => {
                let mut store = job_store_clone.lock().unwrap();
                store.insert(job_id_clone.clone(), JobStatusType::Failed(e.to_string()));
            }
        }
    });
    
    Ok(HttpResponse::Ok().json(CreateTimelapseResponse {
        job_id,
        status: "processing".to_string(),
    }))
}
