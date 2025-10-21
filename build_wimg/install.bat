@echo off
echo ========================================
echo wimg Installation Script (System-wide)
echo ========================================
echo.

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: This script requires administrator privileges.
    echo Please right-click and select "Run as administrator"
    echo.
    pause
    exit /b 1
)

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0
cd /d "%SCRIPT_DIR%"

REM Create installation directory
set INSTALL_DIR=C:\Program Files\wimg
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Check if wimg.exe exists
if not exist "%SCRIPT_DIR%wimg.exe" (
    echo Error: wimg.exe not found in script directory: %SCRIPT_DIR%
    echo Please make sure you extracted all files from the ZIP
    pause
    exit /b 1
)

REM Copy wimg.exe
echo Copying wimg.exe to %INSTALL_DIR%...
copy /Y "%SCRIPT_DIR%wimg.exe" "%INSTALL_DIR%\"

REM Copy required DLLs
echo Copying required DLL files...
set DLL_FILES=libsixel-1.dll libgcc_s_seh-1.dll libwinpthread-1.dll libjpeg-8.dll libpng16-16.dll zlib1.dll
for %%D in (%DLL_FILES%) do (
    if exist "%SCRIPT_DIR%%%D" (
        copy /Y "%SCRIPT_DIR%%%D" "%INSTALL_DIR%\" >nul
    ) else (
        echo Warning: %%D not found in package
    )
)

REM Check if ffmpeg.exe and ffprobe.exe exist in current directory
if exist "%SCRIPT_DIR%ffmpeg.exe" (
    echo Found ffmpeg.exe, copying...
    copy /Y "%SCRIPT_DIR%ffmpeg.exe" "%INSTALL_DIR%\"
) else (
    echo FFmpeg not found in package, installing via winget...
    winget install --id=Gyan.FFmpeg -e --silent --accept-source-agreements --accept-package-agreements
    
    if %errorlevel% equ 0 (
        echo FFmpeg installed successfully
        
        REM Find ffmpeg installation path
        for /f "tokens=*" %%i in ('where ffmpeg.exe 2^>nul') do set FFMPEG_PATH=%%i
        
        if defined FFMPEG_PATH (
            echo Found FFmpeg at: %FFMPEG_PATH%
            for %%i in ("%FFMPEG_PATH%") do set FFMPEG_DIR=%%~dpi
            
            REM Copy ffmpeg and ffprobe to install directory
            if exist "%FFMPEG_DIR%ffmpeg.exe" copy /Y "%FFMPEG_DIR%ffmpeg.exe" "%INSTALL_DIR%\"
            if exist "%FFMPEG_DIR%ffprobe.exe" copy /Y "%FFMPEG_DIR%ffprobe.exe" "%INSTALL_DIR%\"
        ) else (
            echo Warning: FFmpeg installed but not found in PATH
            echo You may need to restart your terminal
        )
    ) else (
        echo Warning: Failed to install FFmpeg via winget
        echo wimg will work for images but not videos/GIFs
        echo You can manually install FFmpeg from: https://ffmpeg.org/download.html
    )
)

if exist "%SCRIPT_DIR%ffprobe.exe" (
    echo Found ffprobe.exe, copying...
    copy /Y "%SCRIPT_DIR%ffprobe.exe" "%INSTALL_DIR%\"
)

REM Add to system PATH
echo Adding to system PATH...

REM Check if already in PATH first
for /f "tokens=2*" %%a in ('reg query "HKLM\System\CurrentControlSet\Control\Session Manager\Environment" /v PATH 2^>nul') do set "CURRENT_PATH=%%b"

echo %CURRENT_PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% equ 0 (
    echo Already in PATH, skipping...
    goto :done
)

REM Use PowerShell to append to PATH (handles long paths better)
echo Using PowerShell to update PATH...
powershell -Command "$oldPath = [Environment]::GetEnvironmentVariable('PATH', 'Machine'); if ($oldPath -notlike '*%INSTALL_DIR%*') { $newPath = $oldPath + ';%INSTALL_DIR%'; [Environment]::SetEnvironmentVariable('PATH', $newPath, 'Machine') }"

if %errorlevel% neq 0 (
    echo Warning: Failed to update PATH automatically
    echo Please manually add this to your system PATH: %INSTALL_DIR%
    echo.
    echo To do this:
    echo 1. Press Win+X and select "System"
    echo 2. Click "Advanced system settings"
    echo 3. Click "Environment Variables"
    echo 4. Under "System variables", find and edit "Path"
    echo 5. Add new entry: %INSTALL_DIR%
    echo.
) else (
    echo PATH updated successfully
)

:done

echo.
echo ========================================
echo Installation complete!
echo ========================================
echo.
echo Files installed to: %INSTALL_DIR%
echo.
echo IMPORTANT: Please restart your terminal for changes to take effect.
echo.
echo Usage: wimg ^<file^>
echo Example: wimg image.jpg
echo Example: wimg video.mp4
echo.
pause
