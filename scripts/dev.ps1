# Quick dev start — launches backend then Tauri dev mode
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

Write-Host "Starting AI Commerce HQ in development mode..." -ForegroundColor Cyan

# Start Python backend in a new window
$pythonCmd = "python"
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { $pythonCmd = "python3" }

Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$ProjectRoot'; $pythonCmd backend/main.py" -WindowStyle Normal

Write-Host "Backend starting on http://localhost:8765" -ForegroundColor Green
Write-Host "Waiting 3 seconds for backend to initialize..." -ForegroundColor Yellow
Start-Sleep 3

Write-Host "Starting Tauri dev mode..." -ForegroundColor Cyan
npm run tauri:dev
