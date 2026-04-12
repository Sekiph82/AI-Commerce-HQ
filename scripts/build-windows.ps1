# =============================================================
# AI Commerce HQ — Windows Installer Build Script
# Run from project root: .\scripts\build-windows.ps1
#
# Output: src-tauri/target/release/bundle/nsis/
#         AI Commerce HQ_1.0.0_x64-setup.exe
# =============================================================

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "  AI Commerce HQ — Windows Installer Builder" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan
Write-Host ""

$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

# ---- Prerequisite checks ----

function Require($cmd, $hint) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        Write-Host "ERROR: '$cmd' not found. $hint" -ForegroundColor Red
        exit 1
    }
}

Require "node"   "Install from https://nodejs.org"
Require "cargo"  "Install Rust from https://rustup.rs"

$pythonCmd = $null
foreach ($p in @("python", "python3", "py")) {
    if (Get-Command $p -ErrorAction SilentlyContinue) { $pythonCmd = $p; break }
}
if (-not $pythonCmd) {
    Write-Host "ERROR: Python not found. Install from https://python.org" -ForegroundColor Red
    exit 1
}
Write-Host "Using Python: $pythonCmd ($( & $pythonCmd --version 2>&1 ))" -ForegroundColor DarkGray
Write-Host "Using Node: $(node --version)" -ForegroundColor DarkGray
Write-Host "Using Cargo: $(cargo --version)" -ForegroundColor DarkGray
Write-Host ""

# ---- Step 1: Install Python dependencies ----

Write-Host "[1/4] Installing Python dependencies..." -ForegroundColor Yellow
& $pythonCmd -m pip install -r backend/requirements.txt -q
if ($LASTEXITCODE -ne 0) { Write-Host "pip install failed." -ForegroundColor Red; exit 1 }
& $pythonCmd -m pip install pyinstaller -q
if ($LASTEXITCODE -ne 0) { Write-Host "PyInstaller install failed." -ForegroundColor Red; exit 1 }
Write-Host "       Python dependencies OK." -ForegroundColor Green

# ---- Step 2: Compile Python backend to standalone exe ----

Write-Host ""
Write-Host "[2/4] Compiling Python backend to backend.exe (this takes ~2-4 min)..." -ForegroundColor Yellow

# Create output directory
New-Item -ItemType Directory -Path "src-tauri/binaries" -Force | Out-Null

# Run PyInstaller using the spec file (includes all hidden imports)
Push-Location backend
& $pythonCmd -m PyInstaller backend.spec `
    --distpath "../src-tauri/binaries" `
    --workpath "../build/pyinstaller-work" `
    --noconfirm
$exitCode = $LASTEXITCODE
Pop-Location

if ($exitCode -ne 0) {
    Write-Host "PyInstaller compilation failed." -ForegroundColor Red
    exit 1
}

# Verify output
$backendExe = "src-tauri/binaries/backend.exe"
if (-not (Test-Path $backendExe)) {
    Write-Host "ERROR: Expected '$backendExe' not found after PyInstaller." -ForegroundColor Red
    exit 1
}

# Tauri sidecar convention: binary must be named backend-{target-triple}.exe
$triple = "x86_64-pc-windows-msvc"
$sidecarPath = "src-tauri/binaries/backend-$triple.exe"
Copy-Item $backendExe $sidecarPath -Force

$sizeMB = [math]::Round((Get-Item $sidecarPath).Length / 1MB, 1)
Write-Host "       Backend compiled: $sidecarPath ($sizeMB MB)" -ForegroundColor Green

# ---- Step 3: Install Node dependencies ----

Write-Host ""
Write-Host "[3/4] Installing Node dependencies..." -ForegroundColor Yellow
if (-not (Test-Path "node_modules")) {
    npm install --silent
    if ($LASTEXITCODE -ne 0) { Write-Host "npm install failed." -ForegroundColor Red; exit 1 }
}
Write-Host "       Node dependencies OK." -ForegroundColor Green

# ---- Step 4: Build Tauri installer ----

Write-Host ""
Write-Host "[4/4] Building Tauri NSIS installer..." -ForegroundColor Yellow
npm run tauri build
if ($LASTEXITCODE -ne 0) { Write-Host "Tauri build failed." -ForegroundColor Red; exit 1 }

# ---- Report output ----

Write-Host ""
Write-Host "=================================================" -ForegroundColor Green
Write-Host "  BUILD COMPLETE" -ForegroundColor Green
Write-Host "=================================================" -ForegroundColor Green
Write-Host ""

$bundleDir = "src-tauri/target/release/bundle"
$installer = Get-ChildItem $bundleDir -Recurse -Include "*.exe" `
    | Where-Object { $_.Name -notlike "backend*" -and $_.Name -like "*setup*" } `
    | Select-Object -First 1

if (-not $installer) {
    # Fallback: any exe in nsis/
    $installer = Get-ChildItem "$bundleDir/nsis" -Filter "*.exe" -ErrorAction SilentlyContinue `
        | Select-Object -First 1
}

if ($installer) {
    $sizeMB = [math]::Round($installer.Length / 1MB, 1)
    Write-Host "  Installer: $($installer.FullName)" -ForegroundColor White
    Write-Host "  Size     : $sizeMB MB" -ForegroundColor White
    Write-Host ""
    Write-Host "  The installer creates a desktop shortcut and Start Menu entry." -ForegroundColor DarkGray
    Write-Host "  No Python, Node, or Rust required on the target machine." -ForegroundColor DarkGray
} else {
    Write-Host "  Installer built — check $bundleDir" -ForegroundColor White
}

Write-Host ""
