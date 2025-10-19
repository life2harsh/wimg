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

REM Create installation directory
set INSTALL_DIR=C:\Program Files\wimg
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy files
echo Copying files to %INSTALL_DIR%...
copy /Y wimg.exe "%INSTALL_DIR%\"
copy /Y ffmpeg.exe "%INSTALL_DIR%\"
copy /Y ffprobe.exe "%INSTALL_DIR%\"

if %errorlevel% neq 0 (
    echo Error: Failed to copy files
    pause
    exit /b 1
)

REM Add to system PATH
echo Adding to system PATH...
for /f "tokens=2*" %%a in ('reg query "HKLM\System\CurrentControlSet\Control\Session Manager\Environment" /v PATH 2^>nul') do set "CURRENT_PATH=%%b"

REM Check if already in PATH
echo %CURRENT_PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% equ 0 (
    echo Already in PATH, skipping...
) else (
    setx /M PATH "%CURRENT_PATH%;%INSTALL_DIR%"
    if %errorlevel% neq 0 (
        echo Error: Failed to update PATH
        pause
        exit /b 1
    )
)

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
echo.
pause
