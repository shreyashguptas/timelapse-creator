use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;

use crate::models::{JobStatusType, ProcessingProgress};
use crate::storage::{get_frames_directory, list_image_files};
use crate::video::rotation::get_rotation_filter;
use crate::JobStore;

fn parse_frame_from_line(line: &str) -> Option<u32> {
    let re = Regex::new(r"frame=\s*(\d+)").ok()?;
    re.captures(line)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
}

fn update_job_progress(
    job_store: &Mutex<std::collections::HashMap<String, JobStatusType>>,
    job_id: &str,
    stage: &str,
    current_frame: u32,
    total_frames: u32,
) {
    let percent = if total_frames > 0 {
        ((current_frame as f32 / total_frames as f32) * 100.0).min(99.0) as u8
    } else {
        0
    };

    let progress = ProcessingProgress {
        stage: stage.to_string(),
        current_frame,
        total_frames,
        percent,
    };

    if let Ok(mut store) = job_store.lock() {
        store.insert(job_id.to_string(), JobStatusType::Processing(Some(progress)));
    }
}

pub async fn create_timelapse_async(
    app: AppHandle,
    job_id: String,
    output_path: PathBuf,
    fps: u32,
    rotation: u32,
) -> Result<()> {
    let job_store = app.state::<JobStore>();

    // Update status to preparing
    {
        let progress = ProcessingProgress {
            stage: "preparing".to_string(),
            current_frame: 0,
            total_frames: 0,
            percent: 0,
        };
        if let Ok(mut store) = job_store.lock() {
            store.insert(job_id.clone(), JobStatusType::Processing(Some(progress)));
        }
    }

    // Get rotation filter if needed
    let rotation_filter = get_rotation_filter(rotation);

    // Get sorted list of image files
    let image_files = list_image_files(&app, &job_id)?;
    if image_files.is_empty() {
        anyhow::bail!("No image files found");
    }

    let total_frames = image_files.len() as u32;
    let frames_dir = get_frames_directory(&app, &job_id);

    // Create a file list for FFmpeg concat demuxer
    let list_file_path = frames_dir.parent().unwrap().join("filelist.txt");
    let mut list_content = String::new();
    for filename in &image_files {
        let file_path = frames_dir.join(filename);
        let abs_path = file_path.canonicalize().unwrap_or(file_path);
        let path_str = abs_path.to_string_lossy().replace('\'', "'\\''");
        list_content.push_str(&format!("file '{}'\n", path_str));
    }
    fs::write(&list_file_path, &list_content).context("Failed to create file list")?;

    // Build FFmpeg arguments
    let mut args: Vec<String> = vec![
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-r".to_string(),
        fps.to_string(),
        "-i".to_string(),
        list_file_path.to_string_lossy().to_string(),
    ];

    // Add rotation filter if needed
    if let Some(filter) = rotation_filter {
        args.push("-vf".to_string());
        args.push(filter);
    }

    // Output settings
    args.extend([
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        "18".to_string(),
        "-preset".to_string(),
        "veryfast".to_string(),
        "-threads".to_string(),
        "0".to_string(),
        "-pix_fmt".to_string(),
        "yuv420p".to_string(),
        "-movflags".to_string(),
        "+faststart".to_string(),
        "-progress".to_string(),
        "pipe:2".to_string(),
        "-y".to_string(),
        output_path.to_string_lossy().to_string(),
    ]);

    // Update to encoding stage
    update_job_progress(&job_store, &job_id, "encoding", 0, total_frames);

    // Try to use bundled ffmpeg sidecar first, fall back to system ffmpeg
    let shell = app.shell();

    // Try sidecar first (bundled ffmpeg)
    let sidecar_result = shell.sidecar("ffmpeg");

    let output = match sidecar_result {
        Ok(sidecar) => {
            // Use bundled ffmpeg
            sidecar
                .args(&args)
                .output()
                .await
                .context("Failed to run bundled FFmpeg")?
        }
        Err(_) => {
            // Fall back to system ffmpeg
            shell
                .command("ffmpeg")
                .args(&args)
                .output()
                .await
                .context("FFmpeg not found. Please install FFmpeg or ensure the bundled binary is available.")?
        }
    };

    // Parse progress from stderr
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    for line in stderr_str.lines() {
        if let Some(frame) = parse_frame_from_line(line) {
            update_job_progress(&job_store, &job_id, "encoding", frame, total_frames);
        }
    }

    // Clean up file list
    let _ = fs::remove_file(&list_file_path);

    if !output.status.success() {
        let error_msg = stderr_str.to_string();
        let display_error = if error_msg.len() > 500 {
            format!("{}...", &error_msg[..500])
        } else {
            error_msg
        };
        anyhow::bail!("FFmpeg failed: {}", display_error);
    }

    // Update to finalizing stage
    update_job_progress(&job_store, &job_id, "finalizing", total_frames, total_frames);

    // Ensure output file is fully synced
    let file = fs::File::open(&output_path).context("Failed to open output file for sync")?;
    file.sync_all().context("Failed to sync output file to disk")?;
    drop(file);

    Ok(())
}
