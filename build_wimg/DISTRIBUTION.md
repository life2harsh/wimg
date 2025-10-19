# Distribution Guide for wimg

This guide explains how to create distributable packages for wimg that users can download and run without needing to build from source.

## Windows Distribution Options

### Option 1: Portable ZIP Package (Recommended for GitHub Releases)

Create a self-contained package that users can extract and run:

**What to include:**
1. `wimg.exe` - The compiled binary
2. `ffmpeg.exe` and `ffprobe.exe` - Bundled from ffmpeg distribution
3. `README.md` - Usage instructions
4. `install.bat` - Automated installation script

**Steps to create:**

```powershell
# 1. Build the release binary
$env:PKG_CONFIG_ALL_STATIC=1
$env:RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-pc-windows-gnu --features sixel

# 2. Create distribution directory
mkdir wimg-windows-x64
cd wimg-windows-x64

# 3. Copy the executable
copy ..\target\x86_64-pc-windows-gnu\release\try_image.exe wimg.exe

# 4. Download and extract FFmpeg (static build)
# Get from: https://github.com/BtbN/FFmpeg-Builds/releases
# Extract ffmpeg.exe and ffprobe.exe to this directory

# 5. Create install.bat (see below)

# 6. Create ZIP
Compress-Archive -Path * -DestinationPath ..\wimg-windows-x64.zip
```

**Create `install.bat`:**

```batch
@echo off
echo Installing wimg...

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo This script requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    pause
    exit /b 1
)

REM Create installation directory
set INSTALL_DIR=C:\Program Files\wimg
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy files
echo Copying files to %INSTALL_DIR%...
copy /Y wimg.exe "%INSTALL_DIR%\"
copy /Y ffmpeg.exe "%INSTALL_DIR%\"
copy /Y ffprobe.exe "%INSTALL_DIR%\"

REM Add to system PATH
echo Adding to system PATH...
setx /M PATH "%PATH%;%INSTALL_DIR%"

echo.
echo Installation complete!
echo Please restart your terminal for changes to take effect.
echo.
echo Usage: wimg ^<file^>
pause
```

**Create `install-user.bat` (no admin required):**

```batch
@echo off
echo Installing wimg for current user...

REM Create user installation directory
set INSTALL_DIR=%USERPROFILE%\wimg
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy files
echo Copying files to %INSTALL_DIR%...
copy /Y wimg.exe "%INSTALL_DIR%\"
copy /Y ffmpeg.exe "%INSTALL_DIR%\"
copy /Y ffprobe.exe "%INSTALL_DIR%\"

REM Add to user PATH
echo Adding to user PATH...
for /f "tokens=2*" %%a in ('reg query HKCU\Environment /v PATH 2^>nul') do set "CURRENT_PATH=%%b"
setx PATH "%CURRENT_PATH%;%INSTALL_DIR%"

echo.
echo Installation complete!
echo Please restart your terminal for changes to take effect.
echo.
echo Usage: wimg ^<file^>
pause
```

### Option 2: Windows Installer (MSI/EXE)

Use **WiX Toolset** or **Inno Setup** to create a professional installer:

#### Using Inno Setup (Easier)

1. Download Inno Setup: https://jrsoftware.org/isinfo.php

2. Create `wimg-setup.iss`:

```inno
[Setup]
AppName=wimg
AppVersion=1.0.0
DefaultDirName={pf}\wimg
DefaultGroupName=wimg
OutputDir=dist
OutputBaseFilename=wimg-setup
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "wimg.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "ffmpeg.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "ffprobe.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
Name: "addtopath"; Description: "Add to system PATH"; GroupDescription: "Additional tasks:"

[Code]
procedure CurStepChanged(CurStep: TSetupStep);
var
  ResultCode: Integer;
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
  begin
    Exec('setx', '/M PATH "' + GetEnv('PATH') + ';' + ExpandConstant('{app}') + '"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  end;
end;
```

3. Compile with Inno Setup to create `wimg-setup.exe`

### Option 3: Chocolatey Package

Create a Chocolatey package for easy installation via `choco install wimg`:

**Create `wimg.nuspec`:**

```xml
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>wimg</id>
    <version>1.0.0</version>
    <title>wimg</title>
    <authors>life2harsh</authors>
    <description>Terminal image and video viewer using sixel graphics</description>
    <projectUrl>https://github.com/life2harsh/wimg</projectUrl>
    <tags>terminal sixel image video cli</tags>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
```

### Option 4: Scoop Manifest

Create a Scoop manifest for `scoop install wimg`:

**Create `wimg.json`:**

```json
{
    "version": "1.0.0",
    "description": "Terminal image and video viewer using sixel graphics",
    "homepage": "https://github.com/life2harsh/wimg",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "https://github.com/life2harsh/wimg/releases/download/v1.0.0/wimg-windows-x64.zip",
            "bin": "wimg.exe"
        }
    },
    "checkver": "github",
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/life2harsh/wimg/releases/download/v$version/wimg-windows-x64.zip"
            }
        }
    }
}
```

## Linux Distribution Options

### AppImage (Portable Single File)

Create a single executable file that works on any Linux distribution:

```bash
# Use cargo-appimage
cargo install cargo-appimage
cargo appimage --release
```

### Debian Package (.deb)

```bash
# Use cargo-deb
cargo install cargo-deb
cargo deb

# Creates: target/debian/wimg_1.0.0_amd64.deb
```

Users install with:
```bash
sudo dpkg -i wimg_1.0.0_amd64.deb
```

### RPM Package (.rpm)

```bash
# Use cargo-generate-rpm
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

### Snap Package

Create `snap/snapcraft.yaml` and publish to Snap Store.

### Flatpak

Create Flatpak manifest for Flathub distribution.

## macOS Distribution Options

### Homebrew Formula

Create a Homebrew tap:

**Create `wimg.rb`:**

```ruby
class Wimg < Formula
  desc "Terminal image and video viewer using sixel graphics"
  homepage "https://github.com/life2harsh/wimg"
  url "https://github.com/life2harsh/wimg/archive/v1.0.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build
  depends_on "libsixel"
  depends_on "ffmpeg"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/wimg", "--version"
  end
end
```

Users install with:
```bash
brew tap life2harsh/wimg
brew install wimg
```

### DMG Package

Create a macOS disk image with drag-to-install interface.

## Recommended Distribution Strategy

### For GitHub Releases:

1. **Create portable ZIP packages** for each platform
2. **Use GitHub Actions** to automate builds
3. **Attach to releases** with clear installation instructions

**Example GitHub Actions workflow (`.github/workflows/release.yml`):**

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-pc-windows-gnu
      
      - name: Build
        run: |
          $env:PKG_CONFIG_ALL_STATIC=1
          $env:RUSTFLAGS="-C target-feature=+crt-static"
          cargo build --release --target x86_64-pc-windows-gnu --features sixel
      
      - name: Package
        run: |
          mkdir wimg-windows-x64
          copy target\x86_64-pc-windows-gnu\release\try_image.exe wimg-windows-x64\wimg.exe
          # Download and copy ffmpeg.exe and ffprobe.exe
          Compress-Archive -Path wimg-windows-x64\* -DestinationPath wimg-windows-x64.zip
      
      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        with:
          upload_url: ${{ github.event.release.upload_url }}
          asset_path: ./wimg-windows-x64.zip
          asset_name: wimg-windows-x64.zip
          asset_content_type: application/zip

  build-linux:
    runs-on: ubuntu-latest
    # Similar steps for Linux

  build-macos:
    runs-on: macos-latest
    # Similar steps for macOS
```

## User Installation (After Distribution)

### Windows:
1. Download `wimg-windows-x64.zip`
2. Extract to any folder
3. Run `install-user.bat` (or `install.bat` as admin)
4. Restart terminal
5. Use `wimg <file>`

### Linux:
```bash
# AppImage
chmod +x wimg.AppImage
./wimg.AppImage <file>

# Or .deb
sudo dpkg -i wimg_1.0.0_amd64.deb
wimg <file>
```

### macOS:
```bash
brew install wimg
wimg <file>
```

## Best Practices

1. **Include FFmpeg** in Windows distributions (it's large but necessary)
2. **Test on clean VMs** to ensure no missing dependencies
3. **Provide checksums** (SHA256) for security
4. **Version your releases** with semantic versioning
5. **Write clear release notes** for each version
6. **Consider file size** - compress executables with UPX if needed
7. **Sign your executables** (optional but recommended for Windows)

## Notes

- Windows Defender may flag unsigned executables
- Users need a sixel-capable terminal (document this clearly)
- FFmpeg is ~80MB, consider hosting it separately if size is a concern
- For enterprise deployment, consider MSI packages with Group Policy support
