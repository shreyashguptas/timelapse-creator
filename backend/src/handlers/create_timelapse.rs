use actix_web::{web, HttpResponse, Error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::{CreateTimelapseRequest, CreateTimelapseResponse, JobStatusType};
use crate::storage::local::{get_frames_directory, get_output_path};
use crate::video::processor::create_timelapse_async;

pub type JobStore = Arc<Mutex<HashMap<String, JobStatusType>>>;

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
    
    // Update job status to processing (with no progress yet)
    {
        let mut store = job_store.lock().unwrap();
        store.insert(job_id.clone(), JobStatusType::Processing(None));
    }
    
    // Spawn async task to process video
    let job_id_clone = job_id.clone();
    // Clone the inner Arc (web::Data wraps in another Arc, so get_ref gives us &Arc<...>)
    let job_store_arc = Arc::clone(job_store.get_ref());

    actix_web::rt::spawn(async move {
        let frames_dir = get_frames_directory(&job_id_clone);
        let output_path = get_output_path(&job_id_clone);

        match create_timelapse_async(
            &job_id_clone,
            frames_dir,
            output_path,
            fps,
            rotation,
            job_store_arc.clone(),
        ).await {
            Ok(_) => {
                let mut store = job_store_arc.lock().unwrap();
                store.insert(job_id_clone.clone(), JobStatusType::Completed);
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                let mut store = job_store_arc.lock().unwrap();
                // Truncate error message if too long for UI
                let display_error = if error_msg.len() > 500 {
                    format!("{}...", &error_msg[..500])
                } else {
                    error_msg
                };
                store.insert(job_id_clone.clone(), JobStatusType::Failed(display_error));
            }
        }
    });
    
    Ok(HttpResponse::Ok().json(CreateTimelapseResponse {
        job_id,
        status: "processing".to_string(),
    }))
}
