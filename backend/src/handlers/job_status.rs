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

    // Extract progress data from status
    let (progress, stage, current_frame, total_frames) = match &status {
        JobStatusType::Processing(Some(p)) => (
            Some(p.percent as u32),
            Some(p.stage.clone()),
            Some(p.current_frame),
            Some(p.total_frames),
        ),
        JobStatusType::Processing(None) => (Some(0), Some("preparing".to_string()), None, None),
        JobStatusType::Completed => (Some(100), Some("complete".to_string()), None, None),
        _ => (None, None, None, None),
    };

    Ok(HttpResponse::Ok().json(JobStatus {
        status: status_str.to_string(),
        progress,
        stage,
        current_frame,
        total_frames,
        error,
    }))
}
