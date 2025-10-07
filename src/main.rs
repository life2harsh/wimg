#[cfg(feature = "sixel")]
use sixel_rs::encoder::{Encoder, QuickFrameBuilder};
#[cfg(feature = "sixel")]
use sixel_rs::pixelformat::PixelFormat;
use image::{DynamicImage, GenericImageView};
use std::env;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        eprintln!("Example: {} photo.jpg", args[0]);
        display_test_pattern()?;
        return Ok(());
    }
    
    let image_path = &args[1];
    
    if !Path::new(image_path).exists() {
        eprintln!("Error: File '{}' not found", image_path);
        std::process::exit(1);
    }
    
    display_image(image_path)?;
    Ok(())
}

fn display_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    let img = resize_for_terminal(img, 100, 40);
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
    println!("Displaying test pattern...");
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
    let char_aspect = 0.5f32;
    let max_w = (max_cols * 8) as f32;
    let max_h = max_rows as f32 * 16.0 * char_aspect;
    let mut nw = w as f32;
    let mut nh = h as f32;
    if nw > max_w {
        let s = max_w / nw;
        nw *= s;
        nh *= s;
    }
    if nh > max_h {
        let s = max_h / nh;
        nw *= s;
        nh *= s;
    }
    img.resize(nw.max(1.0).round() as u32, nh.max(1.0).round() as u32, image::imageops::FilterType::Lanczos3)
}