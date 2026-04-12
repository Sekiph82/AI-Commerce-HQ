@echo off
title AI Commerce HQ — Dev
cd /d "%~dp0"

echo.
echo ================================================
echo   AI Commerce HQ — Starting...
echo ================================================
echo.

where python >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Python not found.
    echo Please install Python from https://python.org
    echo Make sure to tick "Add Python to PATH" during install.
    echo.
    pause
    exit /b 1
)

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Node.js not found.
    echo Please install Node.js from https://nodejs.org
    echo.
    pause
    exit /b 1
)

where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust not found.
    echo Please install Rust from https://rustup.rs
    echo After installing, close this window and open a new one.
    echo.
    pause
    exit /b 1
)

echo All tools found. Starting app...
echo.

python dev.py

echo.
echo ================================================
echo   App has exited.
echo ================================================
pause
