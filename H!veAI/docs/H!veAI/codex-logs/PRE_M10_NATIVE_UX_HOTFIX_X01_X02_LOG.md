# Pre-M10 Native UX Hotfix X01/X02 Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI

## Start evidence

- Mandatory `git fetch origin H!veAI` completed.
- Safe fast-forward completed from `af4dcb424adbc250b8efa0a104cf8b0014117f51` to synchronized prompt/audit HEAD `11ef7d2`.
- Pre-existing untracked user files `start-demo.bat` and `task.md` were preserved.
- Tracker truth was synchronized before hotfix implementation: M00-M09 PASS/CLOSED, strict progress 10/20 = 50%, X01/X02 active, M10 BLOCKED/UNSTARTED.

## X01

Current production cause: shared production `run_git()` spawned `git` without Windows creation flags, allowing child console windows during watcher-driven refreshes.
Production symbol(s) changed: `production_git_command()` and `run_git()` in `src-tauri/src/git_engine/mod.rs`.
Windows no-console mechanism: `std::os::windows::process::CommandExt::creation_flags(0x08000000)` (`CREATE_NO_WINDOW`) is applied to the shared production `git` command; non-Windows behavior is unchanged.
Exact focused test(s): `git_engine::tests::production_git_path_captures_bounded_version_output`; `git_engine::tests::production_git_path_preserves_structured_exit_errors`; existing `status_matrix_detects_staged_unstaged_untracked_and_clean` and snapshot/diff tests.
Watcher/Git regression evidence: focused Git Engine suite 21 passed; focused watcher suite 26 passed, including `watcher_git_category_event_persists_snapshot` and `watcher_git_refresh_failure_preserves_rescan_requirement`.
Automated status: PASS
Manual native acceptance: PENDING

## X02

Current production cause: `StartupIntro.tsx` explicitly rendered the canonical video with `muted`, making native startup silent.
Production symbol(s) changed: `StartupIntro` audible playback preparation and `src-tauri/tauri.conf.json` main-window `additionalBrowserArgs`.
Audible autoplay mechanism: remove the `muted` attribute; set `video.muted = false` and `video.volume = 1` immediately before `play()`; configure WebView2 with `--autoplay-policy=no-user-gesture-required` while preserving WRY's default feature-disable arguments.
Why this mechanism is supported by the installed Tauri/WRY version: local dependency inspection confirms Tauri 2.11.5 exposes window `additionalBrowserArgs`, WRY 0.55.1 applies that field to WebView2, and WRY documents its default autoplay policy plus the default arguments that must be preserved when custom arguments are supplied.
Exact focused test(s): `pre-M10 native UX hotfix` 4 frontend behavior tests; `native WebView2 audio configuration` config test.
Canonical MP4 hash before/after: unchanged, repository `src/assets/opening-video.mp4` and canonical source both SHA-256 `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`.
Automated status: PASS
Manual native acceptance: PENDING

## Focused failure chronology

- Initial X02 focused attempt: 2 test-harness failures, caused by fake timers being enabled before async claim resolution and a remount-based replay model; no production failure was indicated.
- Corrected the tests to wait for claim resolution, use React `act()` for timed dismissal, restore timers after each test, and model same-process rerender behavior.
- Rerun: X02 focused suite PASS, 5 tests passed.

## TRACKER TRUTH

M09 final state: PASS/CLOSED after independent M09D final strict audit.
Roadmap progress: 10/20 = 50%.
M10 state: BLOCKED/UNSTARTED pending independent X01/X02 hotfix audit and required manual native acceptance.

## FULL REGRESSION

- `npm run typecheck`: PASS.
- `npm test -- --run`: PASS, 6 files / 75 tests.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS, 192 tests passed, 0 failed.
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- Publisher failure/rollback harness: PASS, all 9 assertions.
- Governed production Tauri `--no-bundle` QA publisher: PASS; candidate smoke-tested before stable replacement.

## PUBLICATION

stable EXE SHA/size: `H!veAI/dev-bin/H!veAI.exe`, SHA-256 `24B4659F5F119ACC19442C3DF0428067DDE36085BECD2A7B9BB0DFEAFBBBD8A5`, `17716224` bytes.
stable icon SHA/size: `H!veAI/dev-bin/H!veAI.ico`, SHA-256 `D83ED52300040617D1DA2502E35DC25FEC66AF030CDF444DD52B491716B0940E`, `143206` bytes.
shortcut target/icon: target `H!veAI/dev-bin/H!veAI.exe`; icon `H!veAI/dev-bin/H!veAI.ico,0`; arguments empty; working directory `H!veAI/dev-bin`.
installer scan: no `.msi`, `.msix`, `.appx`, `.appxbundle`, `.msixbundle`, or `.wixpdb` files under `H!veAI`.

Canonical asset hashes: opening video `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`; background `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`; H!veAI logo `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`; H!veAI small logo `603869D103E86281988DE87F7E40F18DB1FFB78828525CC66D443A9727C7BAF0`.

## COMMITS / REMOTE

implementation commit: pending.
log commit: pending.
final remote HEAD: pending.
local/origin equality: pending.

## PRE-PUSH SELF-AUDIT

X01
Production defect that existed before this hotfix: unprotected shared Windows Git child-process spawn.
Exact production symbol that now changes behavior: `production_git_command()` used by `run_git()`.
Exact test(s): production-path version/error tests; Git Engine and watcher focused suites.
Why the old code would fail the test / requirement: source inspection would find no `CREATE_NO_WINDOW` before `.spawn()`, and native watcher refreshes could create visible child consoles.
What automated evidence does NOT prove: Rust/js tests cannot prove that no console flashes during a real native watcher session.
Manual acceptance still required: user must run the stable native shortcut for several minutes while causing real watched-repository changes and observe no terminal windows.

X02
Production defect that existed before this hotfix: unconditional `muted` playback in `StartupIntro`.
Exact production symbol that now changes behavior: `StartupIntro` playback preparation and main-window WebView2 arguments.
Exact test(s): audible media preparation, safe dismissal, browser-preview skip, same-process non-replay, and native config tests.
Why the old code would fail the test / requirement: old markup included `muted`, so native intro audio was forcibly disabled.
What automated evidence does NOT prove: browser/jsdom media mocks and config assertions do not prove audible output from a real WebView2 native launch.
Manual acceptance still required: user must cold-launch, restart natively, navigate same-process routes, and confirm video/audio behavior and layout.

## SCOPE

- No M09 production parser changes.
- No M10 workflow code.
- No UI redesign or shell/layout changes.
- No canonical asset byte changes.
- No shell-wrapper workaround for Git.
- No standalone browser or Edge window.
- No installer.
- No fake audible-playback claim from browser/jsdom tests alone.
