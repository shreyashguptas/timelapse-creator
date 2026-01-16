use std::path::PathBuf;
use std::process::Command;
use std::fs;
use anyhow::{Result, Context};
use crate::video::rotation::get_rotation_filter;
use crate::storage::local::list_image_files;

pub fn create_timelapse(
    job_id: &str,
    frames_dir: PathBuf,
    output_path: PathBuf,
    fps: u32,
    rotation: u32,
) -> Result<()> {
    // Get rotation filter if needed
    let rotation_filter = get_rotation_filter(rotation);
    
    // Get sorted list of image files
    let image_files = list_image_files(job_id)?;
    if image_files.is_empty() {
        anyhow::bail!("No image files found");
    }
    
    // Create a file list for FFmpeg concat demuxer
    // FFmpeg can use a file list format: file 'path/to/image1.png'
    let list_file_path = frames_dir.parent().unwrap().join("filelist.txt");
    let mut list_content = String::new();
    for filename in &image_files {
        let file_path = frames_dir.join(filename);
        // Use absolute path and escape single quotes
        let abs_path = file_path.canonicalize()
            .unwrap_or(file_path);
        let path_str = abs_path.to_string_lossy().replace('\'', "'\\''");
        list_content.push_str(&format!("file '{}'\n", path_str));
    }
    fs::write(&list_file_path, list_content)
        .context("Failed to create file list")?;
    
    // Build FFmpeg command
    let mut cmd = Command::new("ffmpeg");
    
    // Input settings using concat demuxer
    cmd.arg("-f")
       .arg("concat")
       .arg("-safe")
       .arg("0")
       .arg("-r")
       .arg(fps.to_string())
       .arg("-i")
       .arg(&list_file_path);
    
    // Add rotation filter if needed
    if let Some(filter) = rotation_filter {
        cmd.arg("-vf").arg(filter);
    }
    
    // Output settings for high quality
    cmd.arg("-c:v")
       .arg("libx264")
       .arg("-crf")
       .arg("18")
       .arg("-preset")
       .arg("slow")
       .arg("-pix_fmt")
       .arg("yuv420p")
       .arg("-movflags")
       .arg("+faststart")
       .arg("-y") // Overwrite output file
       .arg(&output_path);
    
    // Execute FFmpeg
    let output = cmd.output()
        .context("Failed to execute FFmpeg")?;
    
    // Clean up file list
    let _ = fs::remove_file(&list_file_path);
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("FFmpeg failed: {}", stderr);
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn get_ffmpeg_progress(stderr: &[u8]) -> Option<u32> {
    let stderr_str = String::from_utf8_lossy(stderr);
    
    // Try to extract frame number from FFmpeg output
    // FFmpeg outputs lines like: frame=  123 fps= 30 q=28.0 size=    1024kB time=00:00:04.10
    for line in stderr_str.lines() {
        if line.contains("frame=") {
            if let Some(frame_start) = line.find("frame=") {
                let frame_part = &line[frame_start + 6..];
                if let Some(space_pos) = frame_part.find(' ') {
                    if let Ok(frame_num) = frame_part[..space_pos].trim().parse::<u32>() {
                        return Some(frame_num);
                    }
                }
            }
        }
    }
    
    None
}
