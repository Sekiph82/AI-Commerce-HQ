# AI Commerce HQ — Development Setup Script
# Run this once to install all dependencies

Write-Host "=== AI Commerce HQ Setup ===" -ForegroundColor Cyan
Write-Host ""

# Check Node.js
Write-Host "Checking Node.js..." -ForegroundColor Yellow
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Node.js not found. Please install from https://nodejs.org" -ForegroundColor Red
    exit 1
}
$nodeVersion = node --version
Write-Host "Node.js: $nodeVersion" -ForegroundColor Green

# Check npm
Write-Host "Checking npm..." -ForegroundColor Yellow
$npmVersion = npm --version
Write-Host "npm: $npmVersion" -ForegroundColor Green

# Check Python
Write-Host "Checking Python..." -ForegroundColor Yellow
$pythonCmd = $null
foreach ($cmd in @("python", "python3", "py")) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        $pythonCmd = $cmd
        break
    }
}
if (-not $pythonCmd) {
    Write-Host "ERROR: Python not found. Please install from https://python.org" -ForegroundColor Red
    exit 1
}
$pythonVersion = & $pythonCmd --version
Write-Host "Python: $pythonVersion" -ForegroundColor Green

# Check Rust
Write-Host "Checking Rust..." -ForegroundColor Yellow
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust not found. Installing via rustup..." -ForegroundColor Yellow
    Write-Host "Please visit https://rustup.rs and run the installer, then re-run this script." -ForegroundColor Red
    Start-Process "https://rustup.rs"
    exit 1
}
$rustVersion = cargo --version
Write-Host "Rust: $rustVersion" -ForegroundColor Green

# Install Node dependencies
Write-Host ""
Write-Host "Installing Node.js dependencies..." -ForegroundColor Yellow
Set-Location (Split-Path -Parent $PSScriptRoot)
npm install
if ($LASTEXITCODE -ne 0) { Write-Host "npm install failed" -ForegroundColor Red; exit 1 }
Write-Host "Node dependencies installed." -ForegroundColor Green

# Install Python dependencies
Write-Host ""
Write-Host "Installing Python dependencies..." -ForegroundColor Yellow
& $pythonCmd -m pip install -r backend/requirements.txt
if ($LASTEXITCODE -ne 0) { Write-Host "pip install failed" -ForegroundColor Red; exit 1 }
Write-Host "Python dependencies installed." -ForegroundColor Green

Write-Host ""
Write-Host "=== Setup Complete! ===" -ForegroundColor Green
Write-Host ""
Write-Host "To start development:" -ForegroundColor Cyan
Write-Host "  npm run tauri:dev" -ForegroundColor White
Write-Host ""
Write-Host "Or start components separately:" -ForegroundColor Cyan
Write-Host "  Terminal 1: python backend/main.py" -ForegroundColor White
Write-Host "  Terminal 2: npm run tauri:dev" -ForegroundColor White
