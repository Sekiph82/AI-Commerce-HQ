# M13C Windows Codex Executable Resolution Native Remediation Log

Date: 2026-09-02
Product: H!veAI
Branch: H!veAI
Finding: R32
Final builder state: M13C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE

## Scope and synchronization

- Canonical repository: `Sekiph82/AI-Commerce-HQ`
- Starting working root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`
- `git fetch origin H!veAI` completed.
- Pre-fix `HEAD`: `e7c10be`
- Remote advanced from `e7c10be` to `e42d150`.
- `git rev-list --left-right --count HEAD...origin/H!veAI`: `0 3`
- `git merge --ff-only origin/H!veAI` completed; synchronized pre-fix `HEAD`: `e42d150`.
- User-owned untracked `start-demo.bat` and `task.md` were preserved and were not included.

This run closed R32 only. Accepted M13, M13A R27-R29, and M13B R30-R31 behavior and evidence were preserved. No M14 or M21 work was started.

## Exact pre-fix failure

The pre-fix resolver walked PATH entries and returned the first file named `codex.exe` or extensionless `codex`. On this Windows environment it reported 34 PATH entries and selected:

`C:\Users\sekip\AppData\Roaming\npm\codex`

The same bounded candidate inventory found the later valid native candidates:

- `C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\codex.exe`
- `C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\b99306303521e97e\codex.exe`

`where.exe codex` also placed the extensionless npm shim first. A direct CreateProcess-equivalent launch of the exact selected extensionless file with `--version` failed with:

`[WinError 193] %1 is not a valid Win32 application`

This is the reproduced form of the governed native readiness diagnostic `961 is not a valid Win32 application. (os error 193)`. The failure was caused by accepting a non-PE extensionless shim before a later valid `codex.exe`, not by absence of Codex.

## R32 remediation

`codex_adapter.rs` now uses one `resolve_codex_executable()` result for both readiness and session start.

- Windows scans PATH in deterministic order and treats existing `codex` files as explicitly skipped non-native candidates.
- Only `codex.exe` can be selected for direct process launch on Windows.
- Each `.exe` candidate is checked with bounded PE metadata: `MZ`, a PE header within the first 1 MiB, a supported Windows machine value, and a supported PE optional-header magic.
- Invalid `.exe` candidates are skipped so later valid candidates remain eligible.
- A deterministic `%LOCALAPPDATA%\OpenAI\Codex\bin` fallback is subordinate to PATH and is not user-path hard-coded.
- No `cmd.exe /c`, PowerShell wrapper, shell interpolation, generic executable override, arbitrary command, or arbitrary flags were added.
- Diagnostics are bounded and distinguish missing candidates, skipped invalid/non-native candidates, native probe failure, and successful native selection without dumping PATH or local environment data.

## Direct adversarial evidence

The disposable Windows resolver tests all passed in `cargo test codex_adapter::tests --lib` (24 passed):

1. Earlier extensionless `codex` shim is skipped and a later valid `codex.exe` is selected.
2. Invalid `.exe` is skipped and a later valid `.exe` is selected.
3. Multiple valid `.exe` candidates select the first candidate in supplied PATH order.
4. Extensionless and invalid candidates produce no selected executable and a truthful skipped count.
5. Readiness and start use the same resolver policy and deterministic result.
6. Existing prompt injection, process containment, streaming redaction, durable event truth, stop/escalation, and restart reconciliation tests remain green.

## Real native verification

Post-fix bounded resolution selected:

`C:\Users\sekip\AppData\Local\OpenAI\Codex\bin\codex.exe`

Direct harmless probe result:

- Command: exact selected executable with one fixed `--version` argument
- Exit: `0`
- Output: `codex-cli 0.130.0-alpha.5`

The governed publisher produced and smoke-tested:

- Stable executable: `H!veAI\dev-bin\H!veAI.exe`
- Stable SHA-256: `D856C71C738A7F5CBCE692996BE6B97F08CAAB217EEC5E8DF1D583F3334CA0F5`
- Stable size: `21032448` bytes
- Icon: `H!veAI\dev-bin\H!veAI.ico`
- Icon SHA-256: `F0C1CC62F000C959AB10493B902E1A7B2F4E10E1E4BB9C837BF3841BC194C5AD`
- Desktop shortcut target: the stable executable above
- Desktop shortcut icon: the icon above, index `0`
- Candidate executable left behind: `False`
- Publisher smoke/console gate: passed; no forbidden visible console host was created.

Canonical media bytes were not changed. SHA-256 values after publication:

- `src\assets\opening-video.mp4`: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`
- `src\assets\H!veAI.mp4`: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`
- `git diff --quiet -- src/assets/opening-video.mp4 src/assets/H!veAI.mp4`: exit `0`

## Verification gates

| Gate | Result |
| --- | --- |
| Focused `codex_adapter::tests` | PASS, 24/24 |
| Full Rust library regression | PASS, 311/311, serial test execution to avoid the existing parallel stop-fixture timing race |
| Focused M13 frontend test | PASS, 3/3 |
| Full frontend regression | PASS, 98/98 |
| TypeScript typecheck | PASS |
| Frontend production build | PASS |
| `npm audit --audit-level=high` | PASS, 0 vulnerabilities |
| `cargo fmt --all -- --check` | PASS |
| `cargo check` | PASS |
| `git diff --check` | PASS |
| Publisher failure/rollback harness | PASS, 9/9 |
| Governed production Tauri `--no-bundle` publication | PASS |
| Candidate/stable smoke, shortcut target, icon, no terminal/console regression | PASS |

The first two normal parallel full Rust invocations exposed one pre-existing stop-fixture race where `taskkill.exe` returned `There is no running instance of the task` after the child had already exited. The focused test and required full serial regression both passed. No unrelated production process logic was changed.

## Exact scoped files

- `H!veAI/src-tauri/src/codex_adapter.rs`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/codex-logs/M13C_WINDOWS_CODEX_EXECUTABLE_RESOLUTION_NATIVE_REMEDIATION_LOG.md`

No visible UI was redesigned. No installer was created. M09, M10, M11, M12, X01, X02, and canonical opening-video behavior were not changed.

## Commit, push, and governance

- Implementation/tracker commit: `750d777` (`Fix Windows Codex executable resolution`)
- `git push origin H!veAI`: completed, `e42d150..750d777`
- Pushed implementation equality proof: fetched remote `origin/H!veAI` resolved to `750d777`; local implementation `HEAD` was `750d777`; divergence was `0 0` before this log was added.
- M13 remains open pending independent strict re-audit and user native/visual acceptance.
- Strict roadmap progress remains `13 / 20 = 65%`.
- M14 was not started.
- M21 was not started.
- User native acceptance remains pending; this builder log does not claim it.

