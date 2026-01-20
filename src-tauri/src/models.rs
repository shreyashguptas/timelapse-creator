use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingProgress {
    pub stage: String,
    pub current_frame: u32,
    pub total_frames: u32,
    pub percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub job_id: String,
    pub file_count: usize,
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimelapseRequest {
    pub job_id: String,
    pub rotation: u32,
    pub fps: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimelapseResponse {
    pub job_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    pub status: String,
    pub progress: Option<u32>,
    pub stage: Option<String>,
    pub current_frame: Option<u32>,
    pub total_frames: Option<u32>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum JobStatusType {
    Pending,
    Processing(Option<ProcessingProgress>),
    Completed,
    Failed(String),
}

impl JobStatusType {
    pub fn as_str(&self) -> &str {
        match self {
            JobStatusType::Pending => "pending",
            JobStatusType::Processing(_) => "processing",
            JobStatusType::Completed => "completed",
            JobStatusType::Failed(_) => "failed",
        }
    }
}
