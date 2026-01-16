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
    // #region agent log
    let log_path = std::path::Path::new("/Users/shreyashgupta/Documents/Github/timelapse-creator/.cursor/debug.log");
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let log_id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000;
        let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}\",\"timestamp\":{},\"location\":\"processor.rs:8\",\"message\":\"create_timelapse entry\",\"data\":{{\"job_id\":\"{}\",\"frames_dir\":\"{}\",\"output_path\":\"{}\",\"fps\":{},\"rotation\":{}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"A\"}}\n", log_id, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), job_id, frames_dir.display(), output_path.display(), fps, rotation).as_bytes());
    }
    // #endregion
    
    // Get rotation filter if needed
    let rotation_filter = get_rotation_filter(rotation);
    
    // Get sorted list of image files
    let image_files = list_image_files(job_id)?;
    if image_files.is_empty() {
        anyhow::bail!("No image files found");
    }
    
    // #region agent log
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:25\",\"message\":\"image files found\",\"data\":{{\"count\":{},\"files\":{:?}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"C\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), image_files.len(), image_files.iter().take(5).collect::<Vec<_>>()).as_bytes());
    }
    // #endregion
    
    // Create a file list for FFmpeg concat demuxer
    // FFmpeg can use a file list format: file 'path/to/image1.png'
    let list_file_path = frames_dir.parent().unwrap().join("filelist.txt");
    let mut list_content = String::new();
    let mut file_exists_count = 0;
    for filename in &image_files {
        let file_path = frames_dir.join(filename);
        let exists = file_path.exists();
        if exists { file_exists_count += 1; }
        // #region agent log
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:30\",\"message\":\"checking file\",\"data\":{{\"filename\":\"{}\",\"path\":\"{}\",\"exists\":{}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"C\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), filename, file_path.display(), exists).as_bytes());
        }
        // #endregion
        // Use absolute path and escape single quotes
        let abs_path = file_path.canonicalize()
            .unwrap_or(file_path);
        let path_str = abs_path.to_string_lossy().replace('\'', "'\\''");
        list_content.push_str(&format!("file '{}'\n", path_str));
    }
    fs::write(&list_file_path, list_content.clone())
        .context("Failed to create file list")?;
    
    // #region agent log
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:38\",\"message\":\"file list created\",\"data\":{{\"list_path\":\"{}\",\"files_exist\":{},\"list_content_preview\":\"{}\"}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"B\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), list_file_path.display(), file_exists_count, list_content.lines().take(3).collect::<Vec<_>>().join("\\n")).as_bytes());
    }
    // #endregion
    
    // Check if FFmpeg exists before trying to use it
    let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
    match ffmpeg_check {
        Ok(output) if output.status.success() => {
            // FFmpeg is available, continue
        }
        Ok(_) | Err(_) => {
            // FFmpeg not found or not working
            anyhow::bail!(
                "FFmpeg is not installed or not in PATH. Please install FFmpeg:\n\
                macOS: brew install ffmpeg\n\
                Linux: sudo apt-get install ffmpeg (or equivalent)\n\
                Windows: Download from https://ffmpeg.org/download.html\n\
                After installation, verify with: ffmpeg -version"
            );
        }
    }
    
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
    
    // #region agent log
    let cmd_args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().to_string()).collect();
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:70\",\"message\":\"FFmpeg command before execution\",\"data\":{{\"command\":\"ffmpeg\",\"args\":{:?}}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), cmd_args).as_bytes());
    }
    // #endregion
    
    // Execute FFmpeg
    let output = cmd.output()
        .context("Failed to execute FFmpeg. Make sure FFmpeg is installed and in your PATH.")?;
    
    // #region agent log
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:73\",\"message\":\"FFmpeg execution result\",\"data\":{{\"success\":{},\"exit_code\":{:?},\"stderr_preview\":\"{}\",\"stdout_preview\":\"{}\"}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), output.status.success(), output.status.code(), stderr_str.lines().take(5).collect::<Vec<_>>().join("\\n"), stdout_str.lines().take(3).collect::<Vec<_>>().join("\\n")).as_bytes());
    }
    // #endregion
    
    // Clean up file list
    let _ = fs::remove_file(&list_file_path);
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let full_error = format!("FFmpeg failed with exit code {:?}\nSTDERR:\n{}\nSTDOUT:\n{}", 
            output.status.code(), stderr, stdout);
        
        // #region agent log
        if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = std::io::Write::write_all(&mut file, format!("{{\"id\":\"log_{}_{}\",\"timestamp\":{},\"location\":\"processor.rs:120\",\"message\":\"FFmpeg failed with details\",\"data\":{{\"exit_code\":{:?},\"stderr\":\"{}\",\"stdout\":\"{}\"}},\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\"}}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 1000000, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(), output.status.code(), stderr.lines().take(10).collect::<Vec<_>>().join("\\n"), stdout.lines().take(5).collect::<Vec<_>>().join("\\n")).as_bytes());
        }
        // #endregion
        
        anyhow::bail!("{}", full_error);
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
