# M14A Native Test, Publication, and ACTIVE Project Confinement Remediation Log

Date: 2026-09-02
Product: H!veAI
Branch: H!veAI
Scope: M14-R35, M14-R36, M14-R37 only

## Final state

M14A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE

M13 remains PASS/CLOSED. M15-M20 remain planned/blocked. M21 was not started.

## Remediation evidence

### M14-R35: native Rust test loader

The pre-fix command was:

```text
cargo test --manifest-path src-tauri/Cargo.toml --lib -- --test-threads=1
```

It failed before the Rust harness entered with `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND`. The failing executable was `src-tauri/target/debug/deps/hiveai_desktop_lib-27497844d9c49998.exe`. Native import inspection showed `comctl32.dll` and `TaskDialogIndirect`; the host `C:\Windows\System32\comctl32.dll` exports the subclass APIs used by the host but not `TaskDialogIndirect`. The test target was linking the desktop Tauri shell through watcher/app command references even though the test harness only required the library logic.

The governed fix isolates Tauri desktop-shell imports and event emission behind `cfg(not(test))`, keeps the watcher logic testable with a test-only handle alias, and does not alter DLL search paths or weaken application security. The repaired executable launched and the serial feature-enabled library run completed `321 passed; 0 failed`.

### M14-R36: candidate readiness and publication

The pre-fix candidate failure was reproduced with a fresh `WEBVIEW2_USER_DATA_FOLDER` both outside and inside the publisher smoke path. The candidate process and WebView child stayed alive, the title was `H!veAI`, but the page remained `about:blank` and no fresh `HIVEAI_FRONTEND_READY` marker appeared. Startup checkpoint evidence stopped after `M14A startup checkpoint: setup entered`.

The exact cause was the pre-migration SQLite backup inside `DatabaseState::initialize`. The existing database was `110936064` bytes, and the bounded backup was advancing only 5 pages per 25 ms. Startup remained in that backup long enough for the 15-second readiness window to expire. The fix retains bounded backup behavior while increasing the bounded step to 1024 pages with a 1 ms delay. The governed publisher then built `--no-bundle`, smoke-tested the candidate, swapped it, smoke-tested stable, validated PE and shortcut state, and cleaned staging/rollback artifacts.

### M14-R37: exact ACTIVE project confinement

Both the provider-neutral Agent Session Center and the preserved Codex adapter require exact `ACTIVE` status before an operation starts or retries, canonicalize the registered project root, and reject unavailable or non-directory roots. The direct adversarial fixture covers `ACTIVE`, `MISSING`, `ARCHIVED`, unknown status, unavailable root, cross-project task IDs, and cross-project session events, stop, retry, resume, resize, and permission authorization paths. The expected bounded errors are asserted, and no arbitrary cwd, worktree, executable, PID, shell, or argument vector is accepted from the frontend.

## Explicit 40-gate record

1. **PASS** - `git fetch origin H!veAI`, followed by fast-forward-only synchronization from `47a2d46` to `48d3241`.
2. **PASS** - Exact branch `H!veAI`; repository worktree was the requested `AI-Commerce-HQ/H!veAI` checkout. Unrelated `start-demo.bat` and `task.md` were preserved.
3. **PASS** - Read in full: M14A prompt, M14 prompt, M14 implementation log, and M14 strict audit.
4. **PASS** - Reproduced the exact pre-fix `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND` before test-harness entry.
5. **PASS** - Root cause captured as the test executable importing desktop-shell `comctl32.dll!TaskDialogIndirect`, unavailable on the host, through unisolated Tauri/watcher references.
6. **PASS** - Repaired native test executable launched and executed the Rust harness.
7. **PASS** - Focused M13 Codex backend tests: `24 passed; 0 failed`.
8. **PASS** - Focused M14 Agent Session Center tests with PTY support: `9 passed; 0 failed`.
9. **PASS** - Claude resolver, fixed invocation, lifecycle fixture, and session-center fixture coverage passed within the focused M14 set and full serial run.
10. **PASS** - PTY lifecycle fixture and resize validation passed with `--features pty-support`; bounded fixture result `1 passed; 0 failed`.
11. **PASS** - ACTIVE/MISSING/ARCHIVED/unknown/unavailable project adversarial cases passed.
12. **PASS** - Cross-project task/session confinement cases passed, including stop/retry/permission authorization and status-related session operations.
13. **PASS** - Stateful redaction, durable event persistence, byte caps, event caps, and terminal persistence evidence passed in focused Codex tests.
14. **PASS** - Retry provenance and cross-project provenance mismatch tests passed.
15. **PASS** - Restart/orphan recovery tests passed in the full native regression.
16. **PASS** - Full serial Rust library regression with PTY support: `321 passed; 0 failed; 0 ignored`.
17. **PASS** - Focused M14 frontend tests: `2 files, 9 tests passed`.
18. **PASS** - Full frontend suite: `12 files, 104 tests passed`.
19. **PASS** - `npm run typecheck`.
20. **PASS** - `npm run build` production frontend build.
21. **PASS** - `npm audit --audit-level=high`; 0 vulnerabilities.
22. **PASS** - `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`.
23. **PASS** - `cargo check --manifest-path src-tauri/Cargo.toml --all-targets` and PTY-feature all-targets check.
24. **PASS** - `git diff --check`.
25. **PASS** - Pre-fix candidate readiness failure reproduced outside and inside the publisher path with a fresh WebView profile.
26. **PASS** - Missing marker root cause proven as the bounded pre-migration backup stall, not a readiness timeout race or WebView crash.
27. **PASS** - Publisher failure/rollback harness: all 9 checks passed, including exact prior SHA-256 restoration and artifact cleanup.
28. **PASS** - Governed Tauri production `--no-bundle` publication completed.
29. **PASS** - Fresh candidate emitted `HIVEAI_FRONTEND_READY`.
30. **PASS** - Stable executable emitted `HIVEAI_FRONTEND_READY` after swap.
31. **PASS** - Candidate and stable SHA-256 both equal `4871C222F64E4FC97E9BFDAE95F441F7CED6A49D0DCC5DF346F785E3B1EB6EFF`.
32. **PASS** - Desktop shortcut target is `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.exe`; icon is `...\dev-bin\H!veAI.ico,0`.
33. **PASS** - Publisher native smoke found no visible console host after startup; fixed background process policy remained in force.
34. **PASS** - No forbidden development listener on ports 5173 or 8765.
35. **PASS** - Real Codex executable readiness: `codex-cli 0.149.1` resolved from `C:\Users\sekip\AppData\Roaming\npm\codex.ps1`.
36. **PASS** - Real Claude executable readiness/version: `Claude Code 2.1.248` resolved from `C:\Users\sekip\.local\bin\claude.exe`.
37. **PENDING USER ACCEPTANCE** - No harmless real Claude operation was launched in this remediation because it would create an external provider operation and side effects outside the disposable fixture. Native user acceptance remains the required final confirmation for this gate.
38. **PASS** - M15-M20 were not activated or implemented.
39. **PASS** - M21 was not started.
40. **PASS** - Roadmap remains `14 / 20 = 70%` pending independent M14 strict re-audit and user native/visual acceptance.

## Publication identity

Implementation and tracking commit: `295a930f12d27dbc8d72fc7de96ef0a2c6647cc2`

The stable `H!veAI/dev-bin/H!veAI.exe` is the successfully smoke-tested candidate byte-for-byte, with the M14 Agent Session Center and the M14A publication-readiness fix present. No candidate or rollback executable remained after publication.

M15 and M21 remain untouched. This log is immutable evidence for the M14A remediation run and does not declare M14 independently closed.
