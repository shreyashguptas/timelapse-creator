use tauri::{AppHandle, Manager};

use crate::models::{JobStatus, JobStatusType};
use crate::JobStore;

#[tauri::command]
pub async fn get_job_status(app: AppHandle, job_id: String) -> Result<JobStatus, String> {
    let job_store = app.state::<JobStore>();
    let store = job_store.lock().map_err(|e| e.to_string())?;

    let status = store.get(&job_id).cloned().unwrap_or(JobStatusType::Pending);

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

    Ok(JobStatus {
        status: status_str.to_string(),
        progress,
        stage,
        current_frame,
        total_frames,
        error,
    })
}
