use tauri::{AppHandle, Manager};

use crate::models::{CreateTimelapseResponse, JobStatusType};
use crate::storage::{get_frames_directory, get_output_path};
use crate::video::processor::create_timelapse_async;
use crate::JobStore;

#[tauri::command]
pub async fn create_timelapse(
    app: AppHandle,
    job_id: String,
    rotation: u32,
    fps: u32,
) -> Result<CreateTimelapseResponse, String> {
    // Validate rotation
    if !matches!(rotation, 0 | 90 | 180 | 270) {
        return Err("Invalid rotation. Must be 0, 90, 180, or 270".to_string());
    }

    // Validate fps
    if fps == 0 || fps > 60 {
        return Err("FPS must be between 1 and 60".to_string());
    }

    // Check if job exists
    let frames_dir = get_frames_directory(&app, &job_id);
    if !frames_dir.exists() {
        return Err("Job not found".to_string());
    }

    // Update job status to processing
    {
        let job_store = app.state::<JobStore>();
        let mut store = job_store.lock().map_err(|e| e.to_string())?;
        store.insert(job_id.clone(), JobStatusType::Processing(None));
    }

    // Spawn async task to process video
    let job_id_clone = job_id.clone();
    let app_clone = app.clone();

    tauri::async_runtime::spawn(async move {
        let output_path = get_output_path(&app_clone, &job_id_clone);

        match create_timelapse_async(app_clone.clone(), job_id_clone.clone(), output_path, fps, rotation).await
        {
            Ok(_) => {
                let job_store = app_clone.state::<JobStore>();
                if let Ok(mut store) = job_store.lock() {
                    store.insert(job_id_clone, JobStatusType::Completed);
                };
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                let job_store = app_clone.state::<JobStore>();
                if let Ok(mut store) = job_store.lock() {
                    let display_error = if error_msg.len() > 500 {
                        format!("{}...", &error_msg[..500])
                    } else {
                        error_msg
                    };
                    store.insert(job_id_clone, JobStatusType::Failed(display_error));
                };
            }
        }
    });

    Ok(CreateTimelapseResponse {
        job_id,
        status: "processing".to_string(),
    })
}
