#[cfg(feature = "sixel")]
use sixel_rs::encoder::{Encoder, QuickFrameBuilder};
#[cfg(feature = "sixel")]
use sixel_rs::pixelformat::PixelFormat;
use image::{DynamicImage, GenericImageView};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tempfile::TempDir;
use terminal_size::{Width   , Height, terminal_size};

fn get_exe_dir() -> Option<PathBuf> {
    env::current_exe().ok()?.parent().map(|p| p.to_path_buf())
}

fn find_ffmpeg() -> String {
    if let Some(exe_dir) = get_exe_dir() {
        let local_ffmpeg = exe_dir.join("ffmpeg.exe");
        if local_ffmpeg.exists() {
            if let Some(path_str) = local_ffmpeg.to_str() {
                return path_str.to_string();
            }
        }
    }
    "ffmpeg".to_string()
}

fn find_ffprobe() -> String {
    if let Some(exe_dir) = get_exe_dir() {
        let local_ffprobe = exe_dir.join("ffprobe.exe");
        if local_ffprobe.exists() {
            if let Some(path_str) = local_ffprobe.to_str() {
                return path_str.to_string();
            }
        }
    }
    "ffprobe".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <image_or_video_path>", args[0]);
        display_test_pattern()?;
        return Ok(());
    }
    
    let file_path = &args[1];
    
    if !Path::new(file_path).exists() {
        eprintln!("Error: File '{}' not found", file_path);
        std::process::exit(1);
    }
    
    let ext = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    if matches!(ext.as_str(), "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" | "wmv" | "gif") {
        display_video(file_path)?;
    } else {
        display_image(file_path)?;
    }
    
    Ok(())
}

fn get_terminal_dimensions() -> (u32, u32) {
    terminal_size()
        .map(|(Width(w), Height(h))| {
            (w.saturating_sub(1) as u32, h.saturating_sub(1) as u32)
        })
        .unwrap_or((120, 40))
}

fn detect_video_fps(path: &str) -> Option<f64> {
    let ffprobe_cmd = find_ffprobe();
    let output = Command::new(ffprobe_cmd)
        .args(&[
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=r_frame_rate",
            "-of", "default=noprint_wrappers=1:nokey=1",
            path
        ])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let fps_str = String::from_utf8(output.stdout).ok()?;
    let fps_str = fps_str.trim();
    
    if let Some((num, den)) = fps_str.split_once('/') {
        let numerator: f64 = num.parse().ok()?;
        let denominator: f64 = den.parse().ok()?;
        Some(numerator / denominator)
    } else {
        fps_str.parse().ok()
    }
}

fn display_video(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let frame_pattern = temp_dir.path().join("frame_%04d.png");
    
    let is_gif = path.to_lowercase().ends_with(".gif");
    
    let original_fps = if is_gif {
        30.0
    } else {
        detect_video_fps(path).unwrap_or(30.0)
    };
    
    let fps = original_fps.min(30.0);
    
    let fps_filter = format!(
        "fps={},scale=1280:-1:flags=fast_bilinear",
        fps
    );
    
    let ffmpeg_cmd = find_ffmpeg();
    let output = Command::new(ffmpeg_cmd)
        .args(&[
            "-i", path,
            "-vf", &fps_filter,
            "-q:v", "2",
            "-pix_fmt", "rgb24",
            "-f", "image2",
            frame_pattern.to_str().unwrap()
        ])
        .output()?;
    
    if !output.status.success() {
        return Err("ffmpeg failed. Make sure ffmpeg is installed and in PATH.".into());
    }
    
    let mut frame_files: Vec<_> = fs::read_dir(temp_dir.path())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("png"))
        .collect();
    
    frame_files.sort();
    
    if frame_files.is_empty() {
        return Err("No frames extracted from video".into());
    }
    
    let (cols, rows) = get_terminal_dimensions();
    
    let frame_delay = Duration::from_millis(33);
    
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    print!("\x1b[2J\x1b[H\x1b[?25l");
    io::stdout().flush()?;
    
    for frame_path in frame_files {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        let start = Instant::now();
        
        if let Ok(img) = image::open(&frame_path) {
            let resized = resize_for_terminal(img, cols, rows);
            let rgb = resized.to_rgb8();
            let (w, h) = rgb.dimensions();
            
            print!("\x1b[H");
            
            #[cfg(feature = "sixel")]
            {
                if let Ok(enc) = Encoder::new() {
                    let frame_data = QuickFrameBuilder::new()
                        .width(w as usize)
                        .height(h as usize)
                        .format(PixelFormat::RGB888)
                        .pixels(rgb.as_raw().to_vec());
                    let _ = enc.encode_bytes(frame_data);
                    io::stdout().flush()?;
                }
            }
            
            #[cfg(not(feature = "sixel"))]
            {
                let sixel = encode_sixel(rgb.as_raw(), w as usize, h as usize, 256);
                print!("\x1bPq{sixel}\x1b\\");
                io::stdout().flush()?;
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed < frame_delay {
            thread::sleep(frame_delay - elapsed);
        }
    }
    
    print!("\x1b[2J\x1b[H\x1b[?25h");
    println!("Video playback finished.");
    Ok(())
}

fn display_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (cols, rows) = get_terminal_dimensions();
    
    let img = image::open(path)?;
    let img = resize_for_terminal(img, cols, rows);
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    #[cfg(feature = "sixel")]
    {
        let enc = Encoder::new().map_err(|e| format!("Encoder error: {:?}", e))?;
        let frame = QuickFrameBuilder::new()
            .width(w as usize)
            .height(h as usize)
            .format(PixelFormat::RGB888)
            .pixels(rgb.as_raw().to_vec());
        enc.encode_bytes(frame).map_err(|e| format!("Encode error: {:?}", e))?;
        io::stdout().flush()?;
    }
    #[cfg(not(feature = "sixel"))]
    {
        let sixel = encode_sixel(rgb.as_raw(), w as usize, h as usize, 256);
        print!("\x1bPq{sixel}\x1b\\");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}

fn display_test_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let w = 120usize;
    let h = 60usize;
    let mut px = Vec::with_capacity(w * h * 3);
    for y in 0..h {
        for x in 0..w {
            let r = (x * 255 / w) as u8;
            let g = (y * 255 / h) as u8;
            let b = ((x + y) % 256) as u8;
            px.extend_from_slice(&[r, g, b]);
        }
    }
    #[cfg(feature = "sixel")]
    {
        let enc = Encoder::new().map_err(|e| format!("Encoder error: {:?}", e))?;
        let frame = QuickFrameBuilder::new()
            .width(w)
            .height(h)
            .format(PixelFormat::RGB888)
            .pixels(px);
        enc.encode_bytes(frame).map_err(|e| format!("Encode error: {:?}", e))?;
        io::stdout().flush()?;
    }
    #[cfg(not(feature = "sixel"))]
    {
        let sixel = encode_sixel(&px, w, h, 128);
        print!("\x1bPq{sixel}\x1b\\");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}

#[cfg(not(feature = "sixel"))]
fn encode_sixel(rgb: &[u8], width: usize, height: usize, max_palette: usize) -> String {
    let mut palette: Vec<(u8,u8,u8)> = Vec::new();
    let mut map: std::collections::HashMap<(u8,u8,u8), u8> = std::collections::HashMap::new();
    for chunk in rgb.chunks(3) {
        if chunk.len() < 3 { continue; }
        let c = (chunk[0], chunk[1], chunk[2]);
        if map.contains_key(&c) { continue; }
        if palette.len() >= max_palette { break; }
        let idx = palette.len() as u8;
        palette.push(c);
        map.insert(c, idx);
    }
    let mut indexed: Vec<u8> = Vec::with_capacity(width * height);
    for chunk in rgb.chunks(3) {
        if chunk.len() < 3 { continue; }
        let c = (chunk[0], chunk[1], chunk[2]);
        let idx = *map.get(&c).unwrap_or(&nearest(&palette, c));
        indexed.push(idx);
    }
    let mut out = String::new();
    out.push('\"');
    out.push_str("0;0;");
    out.push_str(&width.to_string());
    out.push(';');
    out.push_str(&height.to_string());
    for (i,(r,g,b)) in palette.iter().enumerate() {
        let pr = (*r as f32 / 255.0 * 100.0).round() as u8;
        let pg = (*g as f32 / 255.0 * 100.0).round() as u8;
        let pb = (*b as f32 / 255.0 * 100.0).round() as u8;
        out.push_str(&format!("#{};2;{};{};{}", i, pr, pg, pb));
    }
    for ci in 0..palette.len() {
        out.push_str(&format!("#{}", ci));
        let mut y = 0;
        while y < height {
            let mut x = 0;
            let mut line_started = false;
            while x < width {
                let mut bits = 0u8;
                for bit in 0..6 {
                    let yy = y + bit;
                    if yy >= height { break; }
                    let idx = yy * width + x;
                    if indexed[idx] as usize == ci {
                        bits |= 1 << bit;
                    }
                }
                if bits != 0 {
                    out.push((63 + bits) as char);
                    line_started = true;
                } else if line_started {
                    out.push('?');
                }
                x += 1;
            }
            out.push('$');
            y += 6;
        }
        out.push('-');
    }
    out.push('-');
    out
}

#[cfg(not(feature = "sixel"))]
fn nearest(palette: &[(u8,u8,u8)], c: (u8,u8,u8)) -> u8 {
    if palette.is_empty() { return 0; }
    let (tr,tg,tb) = c;
    let mut best = 0usize;
    let mut bestd = u32::MAX;
    for (i,(r,g,b)) in palette.iter().enumerate() {
        let dr = *r as i32 - tr as i32;
        let dg = *g as i32 - tg as i32;
        let db = *b as i32 - tb as i32;
        let d = (dr*dr + dg*dg + db*db) as u32;
        if d < bestd {
            bestd = d;
            best = i;
        }
    }
    best as u8
}

fn resize_for_terminal(img: DynamicImage, max_cols: u32, max_rows: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let img_aspect = w as f32 / h as f32;
    
    let max_w = (max_cols * 10) as f32;
    let max_h = (max_rows * 10) as f32;
    
    let (new_w, new_h) = if max_w / max_h > img_aspect {
        let new_h = max_h;
        let new_w = new_h * img_aspect;
        (new_w, new_h)
    } else {
        let new_w = max_w;
        let new_h = new_w / img_aspect;
        (new_w, new_h)
    };
    
    img.resize(
        new_w.max(1.0).round() as u32,
        new_h.max(1.0).round() as u32,
        image::imageops::FilterType::Triangle
    )
}