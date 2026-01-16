use serde::{Deserialize, Serialize};

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
pub struct JobStatus {
    pub status: String,
    pub progress: Option<u32>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JobInfo {
    pub job_id: String,
    pub status: JobStatusType,
    pub file_count: usize,
    pub filenames: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum JobStatusType {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

impl JobStatusType {
    pub fn as_str(&self) -> &str {
        match self {
            JobStatusType::Pending => "pending",
            JobStatusType::Processing => "processing",
            JobStatusType::Completed => "completed",
            JobStatusType::Failed(_) => "failed",
        }
    }
}
