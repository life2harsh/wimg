@echo off
echo ========================================
echo wimg Installation Script (User Install)
echo ========================================
echo.

REM Create user installation directory
set INSTALL_DIR=%USERPROFILE%\wimg
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

REM Add to user PATH
echo Adding to user PATH...
for /f "tokens=2*" %%a in ('reg query HKCU\Environment /v PATH 2^>nul') do set "CURRENT_PATH=%%b"

REM Check if already in PATH
echo %CURRENT_PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% equ 0 (
    echo Already in PATH, skipping...
) else (
    setx PATH "%CURRENT_PATH%;%INSTALL_DIR%"
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
