# M11A REV7 Unicode and Structured Identity Final Closure

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`

## Contract and implementation

REV7 closed the two MAJOR findings from the deep REV6 audit: R24 Unicode-erasing operational identity normalization and R25 lossy Quality identity reconstruction from display text.

R24 now uses one documented `normalize_operational_identity` path that consumes the complete parser-bounded UTF-8 scalar, folds whitespace deterministically, applies Unicode-preserving case folding, and retains punctuation and distinguishing script content. Fixed-size SHA-derived IDs remain deterministic; no transliteration, UUID, raw unbounded source, or random identity was introduced. Blockers, waits, Quality identities, generated Current Work IDs, materialized activity IDs, and stronger-evidence matching use this path.

R25 now carries a private non-serialized `AttentionIdentity` from structured source assembly through deduplication. Quality uses the original bounded `MaterializedFact.label`; TEST_RUN uses persisted `command`; AUDIT uses persisted `summary`; WAITING/BLOCKER/PERMISSION/WORKFLOW use their original structured sources. Human-facing `title` and `detail` are presentation only and are never parsed as identity authority.

Direct adversarial coverage proves distinct Unicode blockers/activity/Quality facts survive, identical Unicode facts deduplicate deterministically, long bounded UTF-8 remains safe, unrelated preceding rows preserve IDs, colon-bearing Quality labels remain distinct from shorter persisted sources, exact structured matches suppress only the weaker duplicate, and display formatting does not change deduplication.

## Evidence

- `cargo test --lib command_center::tests::m11a_r24 -- --nocapture --test-threads=1` -> 2 passed.
- `cargo test --lib command_center::tests::m11a_r25 -- --nocapture --test-threads=1` -> 1 passed.
- `cargo test --lib command_center::tests::m11a_r23 -- --nocapture --test-threads=1` -> 2 passed.
- `cargo test --lib command_center::tests::m11a_r19 -- --nocapture --test-threads=1` -> 3 passed.
- `cargo test --lib command_center::tests::m11a_r20 -- --nocapture --test-threads=1` -> 3 passed.
- `cargo test --lib command_center::tests::m11a_r22 -- --nocapture --test-threads=1` -> 1 passed.
- Focused Project Dashboard parser tests for Quality headers, dogfood contract, and UTF-8 bounds -> 3 passed.
- Focused watcher tests for actual notify, live scope transition, and last-good refresh -> 3 passed.
- `cargo test --lib -- --nocapture --test-threads=1` -> 278 passed, 0 failed; assertions executed.
- Focused frontend Command Center/Task Sources/shell/startup tests -> 6 files, 70 passed.
- `npm.cmd test -- --run --reporter=dot` -> 9 files, 87 passed.
- `npm.cmd run typecheck`, `npm.cmd run build`, `npm.cmd audit --audit-level=high`, `cargo fmt --all -- --check`, `cargo check`, and `git diff --check` -> passed; npm audit found 0 vulnerabilities.
- Publisher failure harness -> all 9 scenarios passed.
- `powershell.exe -ExecutionPolicy Bypass -File .\scripts\publish-dev-qa.ps1` -> passed; production Tauri `--no-bundle`, candidate/stable smoke tests, rollback-safe publication, shortcut, and terminal suppression checks passed.
- Stable executable technical probe -> window title `H!veAI`; two startup captures differed (`49370AD731CA8394FA10E3409EA8D771F4ACC9CFB1817B6262EE60EE82A080B9` and `21FD31BC2211BA721B8B2157115FA50463FD80B640D5C3927CD8A586F0B4D5BC`); no new visible console host.

## Preserved assets and closures

- Background SHA-256 remains `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`.
- Canonical `src/assets/opening-video.mp4` SHA-256 remains `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`.
- The separately confirmed `H!veAI.mp4` startup asset and H!veAI ICO remain unchanged by REV7.
- R19-R23, R15 watcher architecture/actual-notify evidence, R17 front-matter accounting, R18 enum validation, M10 precedence, unknown-value truth, last-good refresh, Advanced source inventory, accepted topbar/footer/startup/terminal shell behavior, and external-project protections remain preserved.
- No external registered project repository or Bulk Edit was touched. M12 and M21 were not started.

## Git evidence

Exact pre-implementation state:

- HEAD: `a3e14d6b429e7bcf75c587cf0754d4106328b410`
- origin/H!veAI: `a3e14d6b429e7bcf75c587cf0754d4106328b410`
- `HEAD...origin/H!veAI`: `0 0`

Implementation commit: `4a7e6adc29d53ae5f30b321229ecd2b25ec97cfa`

Exact post-implementation state after push and fetch:

- local HEAD: `4a7e6adc29d53ae5f30b321229ecd2b25ec97cfa`
- fetched origin/H!veAI: `4a7e6adc29d53ae5f30b321229ecd2b25ec97cfa`
- `HEAD...origin/H!veAI`: `0 0`

REV7 implementation files changed:

- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/src-tauri/src/command_center.rs`
- `H!veAI/src-tauri/src/project_dashboard.rs`
- `H!veAI/src/assets/opening-video.mp4` (restored to the required canonical REV7 hash)

Final builder state: `IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M11 remains NOT CLOSED. M12 remains BLOCKED.
