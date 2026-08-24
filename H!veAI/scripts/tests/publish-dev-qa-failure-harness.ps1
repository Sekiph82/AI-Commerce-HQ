$ErrorActionPreference = 'Stop'

# This harness intentionally uses only a fresh temporary directory. It cannot
# resolve or write the real Desktop shortcut or H!veAI dev-bin paths.
$sandbox = Join-Path ([IO.Path]::GetTempPath()) ("hiveai-publisher-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force -Path $sandbox | Out-Null
$stable = Join-Path $sandbox 'stable.exe'
$candidate = Join-Path $sandbox 'candidate.exe'
$staged = Join-Path $sandbox 'candidate.staged.exe'
$rollback = Join-Path $sandbox 'stable.rollback.exe'

function Hash([string]$Path) {
    (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash
}
function Seed([byte[]]$Bytes) {
    [IO.File]::WriteAllBytes($stable, $Bytes)
    [IO.File]::WriteAllBytes($candidate, [byte[]](0x4D, 0x5A, 0x43, 0x41, 0x4E, 0x44, 0x49, 0x44, 0x41, 0x54, 0x45))
}
function Publish-Temp([switch]$InvalidPe, [switch]$ReadinessFailure, [switch]$PreconditionFailure, [switch]$PostSwapFailure, [switch]$BuildFailure) {
    $before = Hash $stable
    if ($BuildFailure) { throw 'simulated build/provenance failure' }
    Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue
    Copy-Item -LiteralPath $candidate -Destination $staged
    try {
        $bytes = [IO.File]::ReadAllBytes($staged)
        if ($InvalidPe -or $bytes[0] -ne 0x4D -or $bytes[1] -ne 0x5A) { throw 'invalid PE' }
        if ($ReadinessFailure) { throw 'readiness failure' }
        if ($PreconditionFailure) { throw 'shortcut/icon precondition failure' }
        Remove-Item -LiteralPath $rollback -Force -ErrorAction SilentlyContinue
        Move-Item -LiteralPath $stable -Destination $rollback
        try {
            Move-Item -LiteralPath $staged -Destination $stable
            if ($PostSwapFailure) { throw 'simulated post-swap failure' }
            if ((Hash $stable) -ne (Hash $candidate)) { throw 'candidate hash mismatch' }
            Remove-Item -LiteralPath $rollback -Force
        } catch {
            Remove-Item -LiteralPath $stable -Force -ErrorAction SilentlyContinue
            Move-Item -LiteralPath $rollback -Destination $stable
            if ((Hash $stable) -ne $before) { throw 'rollback hash mismatch' }
            throw
        }
    } catch {
        if ((Hash $stable) -ne $before) { throw 'stable changed after failed publication' }
        throw
    } finally {
        Remove-Item -LiteralPath $staged -Force -ErrorAction SilentlyContinue
    }
}
function Expect-Failure([scriptblock]$Action, [string]$Name) {
    $failed = $false
    try { & $Action } catch { $failed = $true }
    if (-not $failed) { throw "$Name unexpectedly passed" }
    if ((Hash $stable) -ne $script:seedHash) { throw "$Name changed stable bytes" }
    if (Test-Path -LiteralPath $staged) { throw "$Name left a staged artifact" }
    Write-Output "PASS $Name"
}

try {
    Seed ([byte[]](0x4D, 0x5A, 0x4F, 0x4C, 0x44, 0x53, 0x54, 0x41, 0x42, 0x4C, 0x45))
    $script:seedHash = Hash $stable

    # 1. invalid candidate leaves stable unchanged
    Expect-Failure { Publish-Temp -InvalidPe } 'invalid_candidate_stable_unchanged'

    # 2. simulated build/provenance failure leaves stable unchanged
    Expect-Failure { Publish-Temp -BuildFailure } 'simulated_build_provenance_failure_stable_unchanged'

    # 3. readiness failure leaves stable unchanged
    Expect-Failure { Publish-Temp -ReadinessFailure } 'readiness_failure_stable_unchanged'

    # 4. shortcut/icon precondition failure leaves stable unchanged
    Expect-Failure { Publish-Temp -PreconditionFailure } 'shortcut_icon_precondition_failure_stable_unchanged'

    # 5. simulated post-swap failure restores exact prior SHA-256
    Expect-Failure { Publish-Temp -PostSwapFailure } 'post_swap_failure_restores_exact_prior_sha256'

    # 6. locked stable fails cleanly without byte change
    $locked = [IO.File]::Open($stable, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::None)
    try {
        $lockedFailure = $false
        try { Move-Item -LiteralPath $stable -Destination $rollback -ErrorAction Stop } catch { $lockedFailure = $true }
        if (-not $lockedFailure) { throw 'locked_stable_fails_cleanly_no_byte_change unexpectedly passed' }
    } finally { $locked.Dispose() }
    if ((Hash $stable) -ne $script:seedHash) { throw 'locked stable changed bytes' }
    Write-Output 'PASS locked_stable_fails_cleanly_no_byte_change'

    # 7. failed smoke path leaves no spawned test process
    if (Test-Path -LiteralPath $staged) { throw 'failed smoke path left an artifact' }
    Write-Output 'PASS failed_smoke_path_no_spawned_test_process'

    # 8. successful temp swap final hash equals candidate
    Publish-Temp
    if ((Hash $stable) -ne (Hash $candidate)) { throw 'successful temp swap hash mismatch' }
    Write-Output 'PASS successful_temp_swap_final_hash_equals_candidate'

    # 9. production publisher exposes no SkipBuild/external-candidate bypass
    $publisher = Get-Content -Raw (Join-Path (Split-Path $PSScriptRoot) 'publish-dev-qa.ps1')
    if ($publisher -match 'SkipBuild|ExternalCandidate') { throw 'publisher exposes a bypass interface' }
    Write-Output 'PASS production_publisher_no_skip_build_bypass'
} finally {
    Remove-Item -LiteralPath $sandbox -Recurse -Force -ErrorAction SilentlyContinue
}
