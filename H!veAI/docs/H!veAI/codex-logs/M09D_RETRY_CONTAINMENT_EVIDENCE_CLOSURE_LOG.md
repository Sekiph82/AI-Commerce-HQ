# M09D Retry Containment Evidence Closure Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI

## Start evidence

- Mandatory `git fetch origin H!veAI` completed.
- Safe fast-forward completed from `0d70654e8ab218a74a8f1f8901c8a57ce0f494e5` to synchronized M09D prompt/audit HEAD `4619615`.
- Pre-existing untracked user files `start-demo.bat` and `task.md` were preserved.
- No canonical asset or visible UI production file was modified.

## E01D

Production source changed: NO
Test/test-only symbol(s) changed: `RETRY_TEST_LOCK`; `task_intelligence::tests::p01_second_change_after_refresh_is_skipped_after_exactly_one_retry`; `task_intelligence::tests::p01_retry_rechecks_physical_containment`
Exact test: `p01_retry_rechecks_physical_containment`
Why M09C test could false-pass: it substituted `../outside.md` without creating that outside file, so refreshed `canonicalize()` could fail with file-not-found before the explicit containment rejection; it asserted only the warning code.
How M09D proves canonicalizable outside-root target: it creates `.m09d-outside-<project-temp-directory-name>.md` in the project temp directory's parent, routes the private retry failpoint to `../<that-file>`, calls the real `read_authoritative_source()` retry path, and removes the sibling file after the call.
Exact asserted warning code: `SOURCE_READ_FAILED`
Exact asserted warning message: `refreshed source is outside registered root`
Additional test-isolation correction: the first normal-parallel parser-focused run exposed shared retry failpoint interference between the two retry tests (`51 passed, 2 failed`). A test-only `RETRY_TEST_LOCK` now serializes those failpoint users; no production behavior changed.
Status: PASS

## Focused evidence

- Strengthened focused test: PASS, `p01_retry_rechecks_physical_containment`.
- Full task-intelligence focused suite after test-only isolation correction: PASS, 53 passed, 0 failed.

## Regression and publication

- `npm run typecheck`: PASS.
- `npm test -- --run`: PASS, 5 files / 70 tests.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS, 190 tests passed, 0 failed.
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- Publisher failure harness: PASS, all 9 governed failure/rollback assertions.
- Governed production `--no-bundle` QA publisher: PASS; candidate was smoke-tested before stable replacement.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`, SHA-256 `C45079C227B1A78CE1A012DAF0A4AAF74E17D02C651C561EF17023651FA69A70`, size `17715200` bytes.
- Stable icon: `H!veAI/dev-bin/H!veAI.ico`, SHA-256 `D83ED52300040617D1DA2502E35DC25FEC66AF030CDF444DD52B491716B0940E`, size `143206` bytes.
- Shortcut target: `H!veAI/dev-bin/H!veAI.exe`; icon: `H!veAI/dev-bin/H!veAI.ico,0`.

## Scope and truth

- No M09 production parser logic changed.
- No visible UI, canonical asset, M10, Git Engine, watcher, StartupIntro, X01, or X02 changes.
- No installer was created.
- Installer scan under `H!veAI` found no `.msi`, `.msix`, `.appx`, `.appxbundle`, `.msixbundle`, or `.wixpdb` files.
- Canonical asset hashes remained unchanged: background `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`; opening video `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`; H!veAI logo `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`; H!veAI small logo `603869D103E86281988DE87F7E40F18DB1FFB78828525CC66D443A9727C7BAF0`.
- M09 remains ACTIVE / NOT CLOSED pending independent M09D audit.
- M10 remains BLOCKED / UNSTARTED.
- X01 terminal-popup and X02 startup-audio remain queued after M09 closure and before M10.

## Commit and publication evidence

- Implementation/test/tracker commit: `0e4e7f1f46e01be8b21bd3c9b39fa5340ed840a4` (`Close M09D retry containment evidence`).
- M09D log commit: `SELF / verified after push in session`.
- Final remote branch HEAD: `SELF / verified after push in session`.
- Final local/origin equality after the pushed log commit: `HEAD == origin/H!veAI; 0 0`, verified after the final push in session.
