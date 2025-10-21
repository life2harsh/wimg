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

# Copy libsixel-1.dll
Write-Host "Copying libsixel-1.dll..." -ForegroundColor Yellow
if (Test-Path "C:\msys64\mingw64\bin\libsixel-1.dll") {
    Copy-Item "C:\msys64\mingw64\bin\libsixel-1.dll" "dist\wimg-windows-x64\libsixel-1.dll"
} else {
    Write-Host "Warning: libsixel-1.dll not found at C:\msys64\mingw64\bin\" -ForegroundColor Yellow
    Write-Host "Looking for libsixel-1.dll in current directory..." -ForegroundColor Yellow
    if (Test-Path "libsixel-1.dll") {
        Copy-Item "libsixel-1.dll" "dist\wimg-windows-x64\libsixel-1.dll"
    } else {
        Write-Host "Error: libsixel-1.dll not found! wimg will not work without it." -ForegroundColor Red
        exit 1
    }
}

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

try {
    Invoke-WebRequest -Uri $ffmpegUrl -OutFile $ffmpegZip -TimeoutSec 300
    Expand-Archive -Path $ffmpegZip -DestinationPath $ffmpegExtract -Force

    $ffmpegBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
    $ffprobeBin = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffprobe.exe" | Select-Object -First 1

    if ($null -eq $ffmpegBin -or $null -eq $ffprobeBin) {
        throw "FFmpeg binaries not found in downloaded archive"
    }

    Copy-Item $ffmpegBin.FullName "dist\wimg-windows-x64\ffmpeg.exe"
    Copy-Item $ffprobeBin.FullName "dist\wimg-windows-x64\ffprobe.exe"

    Remove-Item $ffmpegZip -Force
    Remove-Item $ffmpegExtract -Recurse -Force
} catch {
    Write-Host "Error downloading FFmpeg: $_" -ForegroundColor Red
    Write-Host "Please download FFmpeg manually from: https://github.com/BtbN/FFmpeg-Builds/releases" -ForegroundColor Yellow
    Write-Host "Extract ffmpeg.exe and ffprobe.exe to: dist\wimg-windows-x64\" -ForegroundColor Yellow
    exit 1
}

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
Write-Host "Size: $([math]::Round((Get-Item "dist\wimg-windows-x64-v$Version.zip").Length / 1MB, 2)) MB"
Write-Host ""
Write-Host "Upload to GitHub Releases for distribution."
