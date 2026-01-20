use tauri::AppHandle;

use crate::storage::cleanup_job as storage_cleanup;

#[tauri::command]
pub async fn cleanup_job(app: AppHandle, job_id: String) -> Result<bool, String> {
    storage_cleanup(&app, &job_id).map_err(|e| e.to_string())?;
    Ok(true)
}
