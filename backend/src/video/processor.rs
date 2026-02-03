use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use anyhow::{Result, Context};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use regex::Regex;
use crate::video::rotation::get_rotation_filter;
use crate::storage::local::list_image_files;
use crate::models::{JobStatusType, ProcessingProgress};

type JobStore = Arc<Mutex<HashMap<String, JobStatusType>>>;

/// Number of frames at the end to show in slow-motion
const SLOW_ENDING_FRAMES: usize = 5;

/// FPS for the slow-motion ending (2 fps = 0.5 seconds per frame)
const SLOW_ENDING_FPS: f64 = 2.0;

/// Parse frame number from FFmpeg stderr line
/// FFmpeg outputs lines like: frame=  123 fps= 30 q=28.0 size=    1024kB time=00:00:04.10
fn parse_frame_from_line(line: &str) -> Option<u32> {
    // Use regex to extract frame number
    let re = Regex::new(r"frame=\s*(\d+)").ok()?;
    re.captures(line)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
}

/// Update job progress in the store
fn update_job_progress(
    job_store: &JobStore,
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

/// Async version of create_timelapse with real-time progress streaming
pub async fn create_timelapse_async(
    job_id: &str,
    frames_dir: PathBuf,
    output_path: PathBuf,
    fps: u32,
    rotation: u32,
    job_store: JobStore,
) -> Result<()> {
    // Update status to preparing
    {
        let progress = ProcessingProgress {
            stage: "preparing".to_string(),
            current_frame: 0,
            total_frames: 0,
            percent: 0,
        };
        if let Ok(mut store) = job_store.lock() {
            store.insert(job_id.to_string(), JobStatusType::Processing(Some(progress)));
        }
    }

    // Get rotation filter if needed
    let rotation_filter = get_rotation_filter(rotation);

    // Get sorted list of image files
    let image_files = list_image_files(job_id)?;
    if image_files.is_empty() {
        anyhow::bail!("No image files found");
    }

    let total_frames = image_files.len() as u32;

    // Create a file list for FFmpeg concat demuxer with duration for slow-motion ending
    let list_file_path = frames_dir.parent().unwrap().join("filelist.txt");
    let mut list_content = String::new();

    // Calculate durations
    let normal_duration = 1.0 / fps as f64;
    let slow_duration = 1.0 / SLOW_ENDING_FPS;

    // Determine which frames get slow-motion treatment
    let slow_start_index = if image_files.len() > SLOW_ENDING_FRAMES {
        image_files.len() - SLOW_ENDING_FRAMES
    } else {
        0 // If we have fewer frames than SLOW_ENDING_FRAMES, all get slow-mo
    };

    for (i, filename) in image_files.iter().enumerate() {
        let file_path = frames_dir.join(filename);
        let abs_path = file_path.canonicalize().unwrap_or(file_path);
        let path_str = abs_path.to_string_lossy().replace('\'', "'\\''");

        // Use slow duration for the last few frames
        let duration = if i >= slow_start_index {
            slow_duration
        } else {
            normal_duration
        };

        list_content.push_str(&format!("file '{}'\n", path_str));
        list_content.push_str(&format!("duration {:.6}\n", duration));
    }
    fs::write(&list_file_path, &list_content).context("Failed to create file list")?;

    // Check if FFmpeg exists before trying to use it
    let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
    match ffmpeg_check {
        Ok(output) if output.status.success() => {}
        Ok(_) | Err(_) => {
            let _ = fs::remove_file(&list_file_path);
            anyhow::bail!(
                "FFmpeg is not installed or not in PATH. Please install FFmpeg:\n\
                macOS: brew install ffmpeg\n\
                Linux: sudo apt-get install ffmpeg (or equivalent)\n\
                Windows: Download from https://ffmpeg.org/download.html\n\
                After installation, verify with: ffmpeg -version"
            );
        }
    }

    // Build FFmpeg command using tokio async process
    let mut cmd = TokioCommand::new("ffmpeg");

    // Input settings using concat demuxer
    // Note: We don't set -r before -i because we use duration directives in the file list
    // for the slow-motion ending effect. The durations control frame display time.
    cmd.arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&list_file_path);

    // Add rotation filter if needed
    if let Some(filter) = rotation_filter {
        cmd.arg("-vf").arg(filter);
    }

    // Output settings - highest quality with fastest speed
    cmd.arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("18")
        .arg("-preset")
        .arg("veryfast")
        .arg("-threads")
        .arg("0")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-movflags")
        .arg("+faststart")
        .arg("-progress")
        .arg("pipe:2")  // Output progress to stderr
        .arg("-y")
        .arg(&output_path);

    // Set up stderr capture for progress
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::null());

    // Update to encoding stage
    update_job_progress(&job_store, job_id, "encoding", 0, total_frames);

    // Spawn the process
    let mut child = cmd.spawn().context("Failed to spawn FFmpeg process")?;

    // Read stderr for progress
    let stderr = child.stderr.take().expect("Failed to capture stderr");
    let reader = BufReader::new(stderr);
    let mut lines = reader.lines();

    let job_id_owned = job_id.to_string();
    let job_store_clone = job_store.clone();

    // Process stderr lines for progress updates
    while let Ok(Some(line)) = lines.next_line().await {
        if let Some(frame) = parse_frame_from_line(&line) {
            update_job_progress(&job_store_clone, &job_id_owned, "encoding", frame, total_frames);
        }
    }

    // Wait for the process to complete
    let status = child.wait().await.context("Failed to wait for FFmpeg")?;

    // Clean up file list
    let _ = fs::remove_file(&list_file_path);

    if !status.success() {
        anyhow::bail!("FFmpeg failed with exit code {:?}", status.code());
    }

    // Update to finalizing stage briefly
    update_job_progress(&job_store, job_id, "finalizing", total_frames, total_frames);

    // Ensure output file is fully synced to disk before signaling completion
    // This prevents race conditions where the file appears complete but data
    // is still in the kernel write cache
    let file = std::fs::File::open(&output_path)
        .context("Failed to open output file for sync")?;
    file.sync_all()
        .context("Failed to sync output file to disk")?;
    drop(file);

    Ok(())
}

#[allow(dead_code)]
pub fn get_ffmpeg_progress(stderr: &[u8]) -> Option<u32> {
    let stderr_str = String::from_utf8_lossy(stderr);

    for line in stderr_str.lines() {
        if let Some(frame) = parse_frame_from_line(line) {
            return Some(frame);
        }
    }

    None
}
