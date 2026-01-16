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
        // #region agent log
        let log_path = std::path::Path::new("/Users/shreyashgupta/Documents/Github/timelapse-creator/.cursor/debug.log");
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"create_timelapse.rs:51\",\"message\":\"spawned async task\",\"data\":{{\"job_id\":\"{}\",\"fps\":{},\"rotation\":{}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), job_id_clone, fps, rotation).as_bytes());
        }
        // #endregion
        
        let frames_dir = get_frames_directory(&job_id_clone);
        let output_path = get_output_path(&job_id_clone);
        
        // #region agent log
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"create_timelapse.rs:55\",\"message\":\"paths before create_timelapse\",\"data\":{{\"frames_dir\":\"{}\",\"output_path\":\"{}\",\"frames_dir_exists\":{},\"output_dir_exists\":{}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"C\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), frames_dir.display(), output_path.display(), frames_dir.exists(), output_path.parent().map(|p| p.exists()).unwrap_or(false)).as_bytes());
        }
        // #endregion
        
        match create_timelapse(&job_id_clone, frames_dir, output_path, fps, rotation) {
            Ok(_) => {
                // #region agent log
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"create_timelapse.rs:59\",\"message\":\"timelapse creation succeeded\",\"data\":{{\"job_id\":\"{}\"}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), job_id_clone).as_bytes());
                }
                // #endregion
                let mut store = job_store_clone.lock().unwrap();
                store.insert(job_id_clone.clone(), JobStatusType::Completed);
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                // #region agent log
                if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"create_timelapse.rs:65\",\"message\":\"timelapse creation failed\",\"data\":{{\"job_id\":\"{}\",\"error\":\"{}\"}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), job_id_clone, error_msg).as_bytes());
                }
                // #endregion
                let mut store = job_store_clone.lock().unwrap();
                // Truncate error message if too long for UI
                let display_error = if error_msg.len() > 500 {
                    format!("{}...", &error_msg[..500])
                } else {
                    error_msg.clone()
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
