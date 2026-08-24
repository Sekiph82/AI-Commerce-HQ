param(
    [switch]$SkipBuild,
    [string]$Candidate = (Join-Path $PSScriptRoot '..\src-tauri\target\release\hiveai-desktop.exe')
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$stableDir = Join-Path $root 'dev-bin'
$stable = Join-Path $stableDir 'H!veAI.exe'
$staged = Join-Path $stableDir 'H!veAI-candidate.exe'
$rollback = Join-Path $stableDir 'H!veAI.rollback.exe'
$desktopShortcut = Join-Path ([Environment]::GetFolderPath('Desktop')) 'H!veAI.lnk'
$expectedIcon = [IO.Path]::GetFullPath((Join-Path $stableDir 'H!veAI.ico'))

function Assert-Pe([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path)) { throw "Executable not found: $Path" }
    $bytes = [System.IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -lt 2 -or [Text.Encoding]::ASCII.GetString($bytes, 0, 2) -ne 'MZ') { throw "Not a Windows PE executable: $Path" }
}
function Assert-NoH1vePorts {
    $ports = @(Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in @(5173, 8765) })
    if ($ports.Count -gt 0) { throw 'H!veAI candidate opened a forbidden development port.' }
}
function Assert-Shortcut([string]$Target) {
    if (-not (Test-Path -LiteralPath $desktopShortcut)) { throw "Desktop shortcut missing: $desktopShortcut" }
    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($desktopShortcut)
    $resolvedTarget = [IO.Path]::GetFullPath($shortcut.TargetPath)
    if ($resolvedTarget -ne [IO.Path]::GetFullPath($Target)) { throw "Shortcut target mismatch: $resolvedTarget" }
    if ($shortcut.IconLocation -notlike "$expectedIcon,*") { throw "Shortcut icon mismatch: $($shortcut.IconLocation)" }
}

New-Item -ItemType Directory -Force -Path $stableDir | Out-Null
if (-not $SkipBuild) {
    Push-Location $root
    try { & npm run tauri:build -- --no-bundle; if ($LASTEXITCODE -ne 0) { throw "Tauri production build failed: $LASTEXITCODE" } }
    finally { Pop-Location }
}
Assert-Pe $Candidate
Assert-NoH1vePorts
Assert-Shortcut $stable
Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue
Copy-Item -LiteralPath $Candidate -Destination $staged
try {
    Assert-Pe $staged
    $beforeConhost = @(Get-Process conhost -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id)
    $process = Start-Process -FilePath $staged -PassThru
    Start-Sleep -Seconds 5
    if ($process.HasExited) { throw "Candidate exited during production smoke: $($process.ExitCode)" }
    $process.Refresh()
    if ($process.MainWindowTitle -ne 'H!veAI') { throw "Unexpected candidate window title: $($process.MainWindowTitle)" }
    Assert-NoH1vePorts
    $afterConhost = @(Get-Process conhost -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id)
    if (@($afterConhost | Where-Object { $_ -notin $beforeConhost }).Count -gt 0) { throw 'Candidate created a visible console host.' }
    if (-not $process.CloseMainWindow()) { $process.Kill() }
    $process.WaitForExit(5000)
    Assert-NoH1vePorts
    Remove-Item -LiteralPath $rollback -Force -ErrorAction SilentlyContinue
    if (Test-Path -LiteralPath $stable) { Move-Item -LiteralPath $stable -Destination $rollback }
    try {
        Move-Item -LiteralPath $staged -Destination $stable
        Assert-Pe $stable
        Assert-Shortcut $stable
        Remove-Item -LiteralPath $rollback -Force -ErrorAction SilentlyContinue
    } catch {
        Remove-Item -LiteralPath $stable -Force -ErrorAction SilentlyContinue
        if (Test-Path -LiteralPath $rollback) { Move-Item -LiteralPath $rollback -Destination $stable }
        throw
    }
} finally { Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue }
Write-Output "Published smoke-tested Tauri production executable: $stable"
Write-Output "Shortcut target: $stable"
Write-Output "Shortcut icon: $expectedIcon,0"
