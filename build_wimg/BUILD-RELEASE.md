# Distribution Package Creation Guide

## File Structure for Distribution

Your distribution package should have this structure:

```
wimg-windows-x64/
├── wimg.exe              # Your compiled binary
├── ffmpeg.exe            # FFmpeg binary
├── ffprobe.exe           # FFprobe binary
├── install-user.bat      # User installation script (no admin)
├── install.bat           # System installation script (admin required)
└── README.txt            # Basic usage instructions
```

## File Paths and Locations

### 1. Your Compiled Binary

**Source Path:**
```
C:\Users\jhaha\OneDrive\Desktop\try_image\target\x86_64-pc-windows-gnu\release\try_image.exe
```

**Copy to distribution folder as:**
```
wimg.exe
```

### 2. FFmpeg Binaries

**Download from:**
- Official: https://ffmpeg.org/download.html#build-windows
- GitHub Builds (easier): https://github.com/BtbN/FFmpeg-Builds/releases

**Look for:** `ffmpeg-master-latest-win64-gpl.zip`

**Extract these files:**
- `bin/ffmpeg.exe` → Copy to your distribution folder
- `bin/ffprobe.exe` → Copy to your distribution folder

### 3. Installation Scripts

**Already created in your project:**
- `C:\Users\jhaha\OneDrive\Desktop\try_image\install-user.bat`
- `C:\Users\jhaha\OneDrive\Desktop\try_image\install.bat`

### Installation Directories (Where files get installed)

**User Installation (`install-user.bat`):**
```
%USERPROFILE%\wimg\
  → C:\Users\jhaha\wimg\
     ├── wimg.exe
     ├── ffmpeg.exe
     └── ffprobe.exe
```

**System Installation (`install.bat` - requires admin):**
```
C:\Program Files\wimg\
     ├── wimg.exe
     ├── ffmpeg.exe
     └── ffprobe.exe
```

## Step-by-Step: Create Distribution Package

### PowerShell Commands:

```powershell
# 1. Navigate to project directory
cd "C:\Users\jhaha\OneDrive\Desktop\try_image"

# 2. Build the release binary (if not already built)
$env:PKG_CONFIG_ALL_STATIC=1
$env:RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-pc-windows-gnu --features sixel

# 3. Create distribution directory
New-Item -ItemType Directory -Force -Path "dist\wimg-windows-x64"

# 4. Copy your compiled binary
Copy-Item "target\x86_64-pc-windows-gnu\release\try_image.exe" "dist\wimg-windows-x64\wimg.exe"

# 5. Copy installation scripts
Copy-Item "install-user.bat" "dist\wimg-windows-x64\"
Copy-Item "install.bat" "dist\wimg-windows-x64\"

# 6. Create README.txt for users
@"
wimg - Terminal Image and Video Viewer
======================================

REQUIREMENTS:
- Windows Terminal with sixel support enabled

INSTALLATION:
1. Double-click 'install-user.bat' (recommended)
   OR
2. Right-click 'install.bat' → Run as administrator (system-wide)

After installation, restart your terminal.

USAGE:
  wimg image.jpg
  wimg video.mp4
  wimg animation.gif

Press Ctrl+C to stop video/GIF playback.

For more information: https://github.com/life2harsh/wimg
"@ | Out-File -FilePath "dist\wimg-windows-x64\README.txt" -Encoding utf8

# 7. Download FFmpeg (do this manually or with script)
# Go to: https://github.com/BtbN/FFmpeg-Builds/releases
# Download: ffmpeg-master-latest-win64-gpl.zip
# Extract and copy ffmpeg.exe and ffprobe.exe to: dist\wimg-windows-x64\

# 8. Create ZIP for distribution
Compress-Archive -Path "dist\wimg-windows-x64\*" -DestinationPath "dist\wimg-windows-x64.zip" -Force

Write-Host "Distribution package created at: dist\wimg-windows-x64.zip"
```

## Alternative: Automated FFmpeg Download

Add this to your PowerShell script to automatically download FFmpeg:

```powershell
# Download and extract FFmpeg automatically
$ffmpegUrl = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
$ffmpegZip = "dist\ffmpeg.zip"
$ffmpegExtract = "dist\ffmpeg-temp"

Write-Host "Downloading FFmpeg..."
Invoke-WebRequest -Uri $ffmpegUrl -OutFile $ffmpegZip

Write-Host "Extracting FFmpeg..."
Expand-Archive -Path $ffmpegZip -DestinationPath $ffmpegExtract -Force

# Find and copy the binaries
$ffmpegBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
$ffprobeBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffprobe.exe" | Select-Object -First 1

Copy-Item $ffmpegBin.FullName "dist\wimg-windows-x64\ffmpeg.exe"
Copy-Item $ffprobeBin.FullName "dist\wimg-windows-x64\ffprobe.exe"

# Cleanup
Remove-Item $ffmpegZip -Force
Remove-Item $ffmpegExtract -Recurse -Force

Write-Host "FFmpeg binaries added successfully"
```

## Complete Automated Build Script

Create `build-release.ps1`:

```powershell
# Complete build and package script
param(
    [string]$Version = "1.0.0"
)

$ErrorActionPreference = "Stop"

Write-Host "Building wimg v$Version for distribution..." -ForegroundColor Cyan

# Clean previous builds
if (Test-Path "dist") {
    Remove-Item "dist" -Recurse -Force
}
New-Item -ItemType Directory -Force -Path "dist\wimg-windows-x64" | Out-Null

# Build binary
Write-Host "`nBuilding binary..." -ForegroundColor Yellow
$env:PKG_CONFIG_ALL_STATIC=1
$env:RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-pc-windows-gnu --features sixel

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Copy binary
Write-Host "Copying binary..." -ForegroundColor Yellow
Copy-Item "target\x86_64-pc-windows-gnu\release\try_image.exe" "dist\wimg-windows-x64\wimg.exe"

# Copy installation scripts
Write-Host "Copying installation scripts..." -ForegroundColor Yellow
Copy-Item "install-user.bat" "dist\wimg-windows-x64\"
Copy-Item "install.bat" "dist\wimg-windows-x64\"

# Create README
Write-Host "Creating README..." -ForegroundColor Yellow
@"
wimg v$Version - Terminal Image and Video Viewer
================================================

REQUIREMENTS:
- Windows Terminal with sixel support enabled

INSTALLATION:
1. Double-click 'install-user.bat' (recommended)
   OR
2. Right-click 'install.bat' → Run as administrator (system-wide)

After installation, restart your terminal.

USAGE:
  wimg image.jpg
  wimg video.mp4
  wimg animation.gif

Press Ctrl+C to stop video/GIF playback.

For more information: https://github.com/life2harsh/wimg
"@ | Out-File -FilePath "dist\wimg-windows-x64\README.txt" -Encoding utf8

# Download FFmpeg
Write-Host "Downloading FFmpeg..." -ForegroundColor Yellow
$ffmpegUrl = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
$ffmpegZip = "dist\ffmpeg.zip"
$ffmpegExtract = "dist\ffmpeg-temp"

Invoke-WebRequest -Uri $ffmpegUrl -OutFile $ffmpegZip
Expand-Archive -Path $ffmpegZip -DestinationPath $ffmpegExtract -Force

$ffmpegBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
$ffprobeBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffprobe.exe" | Select-Object -First 1

Copy-Item $ffmpegBin.FullName "dist\wimg-windows-x64\ffmpeg.exe"
Copy-Item $ffprobeBin.FullName "dist\wimg-windows-x64\ffprobe.exe"

Remove-Item $ffmpegZip -Force
Remove-Item $ffmpegExtract -Recurse -Force

# Create ZIP
Write-Host "Creating distribution ZIP..." -ForegroundColor Yellow
Compress-Archive -Path "dist\wimg-windows-x64\*" -DestinationPath "dist\wimg-windows-x64-v$Version.zip" -Force

# Calculate checksums
Write-Host "`nCalculating checksums..." -ForegroundColor Yellow
$hash = Get-FileHash "dist\wimg-windows-x64-v$Version.zip" -Algorithm SHA256
$hash.Hash | Out-File "dist\wimg-windows-x64-v$Version.zip.sha256" -Encoding utf8

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "Package: dist\wimg-windows-x64-v$Version.zip"
Write-Host "SHA256: $($hash.Hash)"
Write-Host ""
Write-Host "Upload to GitHub Releases for distribution."
```

## Usage

```powershell
# Build and package
.\build-release.ps1 -Version "1.0.0"

# Upload the resulting ZIP to GitHub releases
# Users download, extract, run install-user.bat
```

## File Sizes (Approximate)

- `wimg.exe`: ~5-8 MB (statically linked)
- `ffmpeg.exe`: ~80-100 MB
- `ffprobe.exe`: ~80-100 MB
- **Total ZIP**: ~170-210 MB

## Distribution Checklist

- [ ] Build release binary
- [ ] Copy wimg.exe to distribution folder
- [ ] Download and copy FFmpeg binaries
- [ ] Include installation scripts
- [ ] Create user-friendly README
- [ ] Test on clean Windows VM
- [ ] Create ZIP archive
- [ ] Generate SHA256 checksum
- [ ] Upload to GitHub Releases
- [ ] Update repository README with download link
