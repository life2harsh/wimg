# wimg - Terminal Image and Video Viewer

A high-performance command-line tool for displaying images, videos, and GIFs directly in your terminal using sixel graphics. Built with Rust for cross-platform compatibility and optimal performance.

## Features

- Display images (JPEG, PNG, GIF, BMP, and more) in your terminal
- Play videos (MP4, AVI, MOV, MKV, WebM, FLV, WMV) with smooth frame rendering
- Animated GIF support with enhanced frame rates
- Dynamic FPS detection for videos (preserves original frame rate up to 30 FPS)
- Automatic terminal size detection and adaptive scaling
- Graceful Ctrl+C handling with proper cleanup
- Standalone executable with statically linked dependencies

## Requirements

### Windows

1. **Windows Terminal** with sixel support
   - Download from Microsoft Store or GitHub releases
   - Enable sixel graphics in settings

2. **MSYS2** (for building from source)
   - Download from https://www.msys2.org/
   - Install the MINGW64 toolchain

3. **FFmpeg** (for video/GIF support)
   - Download from https://ffmpeg.org/download.html
   - Add ffmpeg and ffprobe to your PATH

4. **Rust toolchain**
   - Install from https://rustup.rs/
   - Add the GNU target: `rustup target add x86_64-pc-windows-gnu`

5. **libsixel** (built from source in MSYS2)
   ```bash
   # In MSYS2 MINGW64 terminal
   pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-pkg-config git autoconf automake libtool make
   
   git clone https://github.com/libsixel/libsixel.git
   cd libsixel
   ./configure --prefix=/mingw64 --disable-python
   make
   make install
   ```

### Linux

1. **Terminal emulator** with sixel support (e.g., xterm, mlterm, wezterm)
2. **FFmpeg** (`sudo apt install ffmpeg` or equivalent)
3. **libsixel** (`sudo apt install libsixel-dev` or build from source)
4. **Rust toolchain** (from https://rustup.rs/)

### macOS

1. **Terminal emulator** with sixel support (e.g., iTerm2 with sixel enabled, WezTerm)
2. **FFmpeg** (`brew install ffmpeg`)
3. **libsixel** (`brew install libsixel`)
4. **Rust toolchain** (from https://rustup.rs/)

## Building from Source

### Windows (MSYS2 MINGW64)

```bash
# Set environment variables for static linking
export PKG_CONFIG_ALL_STATIC=1
export RUSTFLAGS="-C target-feature=+crt-static"

# Build the release binary
cargo build --release --target x86_64-pc-windows-gnu --features sixel

# The executable will be at: target/x86_64-pc-windows-gnu/release/try_image.exe
```

### Linux / macOS

```bash
cargo build --release --features sixel

# The executable will be at: target/release/try_image
```

## Installation

### Windows

1. Create a directory for binaries (if not exists):
   ```powershell
   mkdir C:\bin
   ```

2. Copy the executable:
   ```powershell
   copy target\x86_64-pc-windows-gnu\release\try_image.exe C:\bin\wimg.exe
   ```

3. Add `C:\bin` to your PATH environment variable

### Linux / macOS

```bash
sudo cp target/release/try_image /usr/local/bin/wimg
sudo chmod +x /usr/local/bin/wimg
```

## Usage

### Display an Image

```bash
wimg photo.jpg
wimg screenshot.png
```

### Play a Video

```bash
wimg video.mp4
wimg movie.avi
```

### Play an Animated GIF

```bash
wimg animation.gif
```

### Test Pattern

Run without arguments to display a test pattern:

```bash
wimg
```

### Interrupt Playback

Press `Ctrl+C` to stop video/GIF playback. The terminal will be properly cleaned up and the cursor restored.

## Technical Details

### Performance Settings

- **Videos**: Dynamic FPS detection (capped at 30 FPS for terminal compatibility)
- **GIFs**: Enhanced to 30 FPS for smoother playback
- **Resolution**: Scales to 1280px width with fast bilinear filtering
- **Display**: 10 pixels per character cell for optimal rendering speed
- **Quality**: Balanced settings for terminal viewing

### Supported Formats

**Images**: JPEG, PNG, GIF (static), BMP, TIFF, WebP, ICO, and more

**Videos**: MP4, AVI, MOV, MKV, WebM, FLV, WMV

**Animations**: GIF (with enhanced frame rate)

### How It Works

1. **FFprobe** detects the original video frame rate
2. **FFmpeg** extracts frames at the detected rate (capped at 30 FPS)
3. Frames are scaled to fit your terminal dimensions
4. **Sixel graphics** encode the frames for terminal display
5. Frames are played back with precise timing matching the original FPS

## Troubleshooting

### "ffmpeg failed" error

Make sure FFmpeg is installed and available in your PATH:

```bash
ffmpeg -version
ffprobe -version
```

### Poor video quality

The tool uses optimized settings for terminal rendering. If you need higher quality, you can modify the source code to increase resolution or change scaling filters.

### Choppy playback

This is often due to terminal rendering limitations. The tool caps at 30 FPS because most terminals cannot render sixel graphics faster than this without stuttering.

### Terminal not showing anything

Ensure your terminal supports sixel graphics:
- Windows: Use Windows Terminal with sixel enabled
- Linux: Use xterm, mlterm, or wezterm
- macOS: Use iTerm2 (with sixel enabled) or WezTerm

## Development

### Project Structure

```
.
├── Cargo.toml          # Project dependencies
├── src/
│   └── main.rs         # Main application code
└── target/             # Build output
```

### Dependencies

- `image` - Image loading and processing
- `sixel-rs` - Sixel graphics encoding (optional feature)
- `tempfile` - Temporary directory management for frame extraction
- `terminal_size` - Dynamic terminal dimension detection
- `ctrlc` - Signal handling for graceful shutdown

### Building without sixel-rs

If you want to use the pure Rust fallback encoder:

```bash
cargo build --release --no-default-features
```

## License

This project is open source. See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgments

- Built with Rust for performance and safety
- Uses libsixel for high-quality sixel encoding
- FFmpeg for video processing
- Thanks to the terminal emulator developers who support sixel graphics