# AI Commerce HQ — Windows Build Script
# Creates a Windows installer (.msi) and portable exe

Write-Host "=== AI Commerce HQ — Windows Build ===" -ForegroundColor Cyan
Write-Host ""

$ProjectRoot = Split-Path -Parent $PSScriptRoot

# Ensure we're in project root
Set-Location $ProjectRoot

# Check dependencies
foreach ($cmd in @("node", "cargo", "python")) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        Write-Host "ERROR: $cmd not found." -ForegroundColor Red
        exit 1
    }
}

# Step 1: Bundle the Python backend with PyInstaller
Write-Host "Step 1: Bundling Python backend..." -ForegroundColor Yellow
$pythonCmd = "python"
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { $pythonCmd = "python3" }

& $pythonCmd -m pip install pyinstaller -q
& $pythonCmd -m PyInstaller `
    --onefile `
    --name backend `
    --distpath "src-tauri/binaries" `
    --workpath "build/pyinstaller" `
    --specpath "build" `
    --hidden-import "aiosqlite" `
    --hidden-import "sqlalchemy.dialects.sqlite" `
    --hidden-import "uvicorn.logging" `
    --hidden-import "uvicorn.loops.auto" `
    --hidden-import "uvicorn.protocols.http.auto" `
    --hidden-import "uvicorn.protocols.websockets.auto" `
    --collect-all "fastapi" `
    "backend/main.py"

if ($LASTEXITCODE -ne 0) {
    Write-Host "PyInstaller failed." -ForegroundColor Red
    exit 1
}
Write-Host "Python backend bundled: src-tauri/binaries/backend.exe" -ForegroundColor Green

# Rename for Tauri sidecar convention
$arch = "x86_64-pc-windows-msvc"
$sidecarName = "backend-$arch.exe"
Copy-Item "src-tauri/binaries/backend.exe" "src-tauri/binaries/$sidecarName" -Force

# Step 2: Build frontend
Write-Host ""
Write-Host "Step 2: Building frontend..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) { Write-Host "Frontend build failed." -ForegroundColor Red; exit 1 }
Write-Host "Frontend built." -ForegroundColor Green

# Step 3: Build Tauri app
Write-Host ""
Write-Host "Step 3: Building Tauri Windows installer..." -ForegroundColor Yellow
npm run tauri build
if ($LASTEXITCODE -ne 0) { Write-Host "Tauri build failed." -ForegroundColor Red; exit 1 }

Write-Host ""
Write-Host "=== Build Complete! ===" -ForegroundColor Green
Write-Host ""
Write-Host "Installer location:" -ForegroundColor Cyan
$msiPath = Get-ChildItem "src-tauri/target/release/bundle" -Recurse -Filter "*.msi" | Select-Object -First 1
if ($msiPath) {
    Write-Host "  $($msiPath.FullName)" -ForegroundColor White
}
$exePath = Get-ChildItem "src-tauri/target/release/bundle" -Recurse -Filter "*.exe" |
    Where-Object { $_.Name -notlike "backend*" } | Select-Object -First 1
if ($exePath) {
    Write-Host "  $($exePath.FullName)" -ForegroundColor White
}
