use actix_web::{web, HttpResponse, Error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::{JobStatus, JobStatusType};

type JobStore = Arc<Mutex<HashMap<String, JobStatusType>>>;

pub async fn get_job_status(
    path: web::Path<String>,
    job_store: web::Data<JobStore>,
) -> Result<HttpResponse, Error> {
    let job_id = path.into_inner();
    let store = job_store.lock().unwrap();
    
    let status = store.get(&job_id)
        .cloned()
        .unwrap_or(JobStatusType::Pending);
    
    let (status_str, error) = match &status {
        JobStatusType::Failed(err) => ("failed", Some(err.clone())),
        _ => (status.as_str(), None),
    };
    
    // Calculate progress if processing
    let progress = if matches!(status, JobStatusType::Processing) {
        // Simple progress estimation - could be improved with actual FFmpeg output parsing
        Some(50) // Placeholder
    } else {
        None
    };
    
    Ok(HttpResponse::Ok().json(JobStatus {
        status: status_str.to_string(),
        progress,
        error,
    }))
}
