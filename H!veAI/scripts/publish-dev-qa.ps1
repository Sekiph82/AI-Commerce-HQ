param(
    [string]$Candidate = (Join-Path $PSScriptRoot '..\src-tauri\target\release\hiveai-desktop.exe')
)

$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$stableDir = Join-Path $root 'dev-bin'
$stable = Join-Path $stableDir 'H!veAI.exe'
$staged = Join-Path $stableDir 'H!veAI.exe.staged'
$desktopShortcut = Join-Path ([Environment]::GetFolderPath('Desktop')) 'H!veAI.lnk'

New-Item -ItemType Directory -Force -Path $stableDir | Out-Null
if (-not (Test-Path -LiteralPath $Candidate)) { throw "Candidate executable not found: $Candidate" }
Copy-Item -LiteralPath $Candidate -Destination $staged -Force
$bytes = [System.IO.File]::ReadAllBytes($staged)
if ($bytes.Length -lt 2 -or [Text.Encoding]::ASCII.GetString($bytes, 0, 2) -ne 'MZ') {
    Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue
    throw 'Candidate is not a valid Windows PE executable.'
}
Move-Item -LiteralPath $staged -Destination $stable -Force

$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($desktopShortcut)
$resolvedTarget = [IO.Path]::GetFullPath($shortcut.TargetPath)
if ($resolvedTarget -ne [IO.Path]::GetFullPath($stable)) {
    throw "Desktop shortcut target is not the stable executable: $resolvedTarget"
}
$expectedIcon = [IO.Path]::GetFullPath((Join-Path $stableDir 'H!veAI.ico'))
if (-not ($shortcut.IconLocation -like "$expectedIcon,*")) {
    throw "Desktop shortcut icon is not the canonical stable ICO: $($shortcut.IconLocation)"
}
Write-Output "Published validated Tauri production executable: $stable"
Write-Output "Shortcut target: $resolvedTarget"
Write-Output "Shortcut icon: $($shortcut.IconLocation)"
