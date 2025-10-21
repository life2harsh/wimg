# wimg - Terminal Image and Video Viewer

A high-performance command-line tool for displaying images, videos, and GIFs directly in your terminal using sixel graphics. Built with Rust for cross-platform compatibility and optimal performance.

## Quick Start

### For Windows Users (Pre-built Binary)

1. Download the latest release: [build_wimg.zip](https://github.com/life2harsh/wimg/releases)
2. Extract the ZIP file
3. Run `install-user.bat` (or `install.bat` as administrator)
4. Restart your terminal
5. Use: `wimg <file>`

**Requirements**: Windows Terminal with sixel support enabled

### For Developers (Build from Source)

See the [Building from Source](#building-from-source) section below.

## Features

- Display images (JPEG, PNG, GIF, BMP, and more) in your terminal
- Play videos (MP4, AVI, MOV, MKV, WebM, FLV, WMV) with smooth frame rendering
- Animated GIF support with enhanced frame rates
- Dynamic FPS detection for videos (preserves original frame rate up to 30 FPS)
- Automatic terminal size detection and adaptive scaling
- Graceful Ctrl+C handling with proper cleanup
- Standalone executable with statically linked dependencies
- FFmpeg bundled in distribution package (no separate installation needed)

## Requirements

### For End Users (Pre-built Binary)

**Windows:**
- Windows Terminal with sixel support enabled
- No additional installations needed (FFmpeg is included in the distribution package)

**Linux:**
- Terminal emulator with sixel support (xterm, mlterm, wezterm)
- FFmpeg installed via package manager

**macOS:**
- Terminal emulator with sixel support (iTerm2, WezTerm)
- FFmpeg installed via Homebrew

### For Developers (Building from Source)

**Windows:**

1. **MSYS2** with MINGW64 toolchain
   - Download from https://www.msys2.org/

2. **Rust toolchain**
   - Install from https://rustup.rs/
   - Add GNU target: `rustup target add x86_64-pc-windows-gnu`

3. **libsixel** (built from source in MSYS2)
   ```bash
   # In MSYS2 MINGW64 terminal
   pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-pkg-config git autoconf automake libtool make
   
   git clone https://github.com/libsixel/libsixel.git
   cd libsixel
   ./configure --prefix=/mingw64 --disable-python
   make
   make install
   ```

4. **FFmpeg** (for video/GIF support during development)
   - Download from https://ffmpeg.org/download.html
   - Add to PATH

**Linux:**

1. **Terminal emulator** with sixel support (xterm, mlterm, wezterm)
2. **FFmpeg** and **libsixel** development libraries
   ```bash
   sudo apt install ffmpeg libsixel-dev  # Debian/Ubuntu
   sudo dnf install ffmpeg libsixel-devel  # Fedora
   ```
3. **Rust toolchain** from https://rustup.rs/

**macOS:**

1. **Terminal emulator** with sixel support (iTerm2, WezTerm)
2. **FFmpeg** and **libsixel**
   ```bash
   brew install ffmpeg libsixel
   ```
3. **Rust toolchain** from https://rustup.rs/

## Building from Source

### Clone the Repository

```bash
git clone https://github.com/life2harsh/wimg.git
cd wimg
```

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

### Create Distribution Package

```powershell
# Windows only
cd build_wimg
.\build-release.ps1 -Version "1.0.0"
```

This will create `dist/wimg-windows-x64-v1.0.0.zip` with everything bundled.

## Installation

### Option 1: Pre-built Binary (Windows)

**Recommended for most users:**

1. Download the latest release from [GitHub Releases](https://github.com/life2harsh/wimg/releases)
2. Extract `wimg-windows-x64.zip`
3. Run one of the installation scripts:
   - `install-user.bat` - Install for current user (no admin required)
   - `install.bat` - Install system-wide (requires administrator)
4. Restart your terminal
5. Test: `wimg --help`

The distribution package includes everything you need: wimg.exe, ffmpeg.exe, and ffprobe.exe.

### Option 2: Build from Source

**For developers or other platforms:**

#### Windows (MSYS2 MINGW64)

```powershell
# Set environment variables for static linking
$env:PKG_CONFIG_ALL_STATIC=1
$env:RUSTFLAGS="-C target-feature=+crt-static"

# Build the release binary
cargo build --release --target x86_64-pc-windows-gnu --features sixel

# Copy to your preferred location
copy target\x86_64-pc-windows-gnu\release\try_image.exe C:\bin\wimg.exe
```

#### Linux / macOS

```bash
# Build
cargo build --release --features sixel

# Install system-wide
sudo cp target/release/try_image /usr/local/bin/wimg
sudo chmod +x /usr/local/bin/wimg

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/try_image ~/.local/bin/wimg
# Add ~/.local/bin to your PATH if not already
```

### Option 3: Create Distribution Package (Windows)

To create your own distribution package:

```powershell
cd build_wimg
.\build-release.ps1 -Version "1.0.0"
```

See [BUILD-RELEASE.md](build_wimg/BUILD-RELEASE.md) for detailed instructions.

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

## Distribution

Want to share wimg with others? See the [DISTRIBUTION.md](DISTRIBUTION.md) guide for information on:
- Creating portable ZIP packages
- Building Windows installers
- Publishing to package managers (Chocolatey, Scoop)
- Creating packages for Linux (AppImage, .deb, .rpm)
- macOS Homebrew formulas

## Troubleshooting

### "ffmpeg failed" error

**If using pre-built binary:** The distribution package includes FFmpeg. Make sure you extracted all files from the ZIP.

**If building from source:** Make sure FFmpeg is installed and available in your PATH:
```bash
ffmpeg -version
ffprobe -version
```

### Terminal not showing anything

Ensure your terminal supports sixel graphics:
- **Windows:** Use Windows Terminal (enable sixel in settings under "Rendering")
- **Linux:** Use xterm, mlterm, or wezterm
- **macOS:** Use iTerm2 (enable sixel in preferences) or WezTerm

### Poor video quality or choppy playback

The tool uses optimized settings for terminal rendering with a balance between quality and performance. Terminal rendering limitations cap playback at 30 FPS. This is normal for sixel graphics in most terminals.

### Installation script doesn't work

**Windows:** Make sure you're running the correct script:
- `install-user.bat` - For current user only (recommended)
- `install.bat` - For all users (right-click → Run as administrator)

After installation, restart your terminal for PATH changes to take effect.

## Development

### Project Structure

```
.
├── Cargo.toml                  # Project dependencies
├── src/
│   └── main.rs                 # Main application code
├── build_wimg/
│   ├── BUILD-RELEASE.md        # Distribution build guide
│   ├── build-release.ps1       # Automated build script
│   ├── install-user.bat        # User installation script
│   └── install.bat             # System installation script
├── DISTRIBUTION.md             # Distribution guide
└── target/                     # Build output
```

### Dependencies

- `image` (0.24) - Image loading and processing
- `sixel-rs` (0.5.0) - Sixel graphics encoding (optional feature)
- `tempfile` (3.8) - Temporary directory management for frame extraction
- `terminal_size` (0.3) - Dynamic terminal dimension detection
- `ctrlc` (3.4) - Signal handling for graceful shutdown

### Building without sixel-rs

If you want to use the pure Rust fallback encoder:

```bash
cargo build --release --no-default-features
```

### Contributing

Contributions are welcome! Please feel free to:
- Report bugs via GitHub Issues
- Submit pull requests
- Suggest new features
- Improve documentation

Before submitting a PR:
1. Ensure code builds without warnings
2. Test on your platform
3. Update documentation if needed

## License

This project is open source. See LICENSE file for details.

## Acknowledgments

- Built with Rust for performance and safety
- Uses libsixel for high-quality sixel encoding
- FFmpeg for video processing
- Thanks to the terminal emulator developers who support sixel graphics

## Links

- **Repository:** https://github.com/life2harsh/wimg
- **Releases:** https://github.com/life2harsh/wimg/releases
- **Issues:** https://github.com/life2harsh/wimg/issues
- **Distribution Guide:** [DISTRIBUTION.md](DISTRIBUTION.md)
- **Build Guide:** [BUILD-RELEASE.md](build_wimg/BUILD-RELEASE.md)
