# M09C Bounded Identity Final Strict Re-Audit

Date: 2026-08-25
Branch: `H!veAI`
M09C prompt commit: `fe5356816bbcdeb4b87d77215cbbb0f141858ab2`
Primary implementation commit: `63b73795dcc781f181b21b1cc02199c67f5565f1`
Publication evidence commit: `452d79d41b9d6a8ed874dc30b0273690aa79faaa`
Audited remote HEAD before this audit: `0d70654e8ab218a74a8f1f8901c8a57ce0f494e5`

## 1. VERDICT

`CONDITIONAL`

M09C closes the last known M09 production MAJOR. R02C is fixed in production: duplicate ordinal working state now uses fixed-size SHA-256 keys and task identity hashing is streamed rather than assembled into giant source-derived intermediate strings. E03C stale/retained/legacy/settings/dependency persistence evidence is also complete.

One bounded evidence defect remains in E01C: the named retry-containment test substitutes `../outside.md` but does not create a canonicalizable outside-root target. The production reader calls `canonicalize()` before the explicit `starts_with(root)` containment rejection. Therefore the current test can pass by failing because the outside path does not exist, without proving the refreshed containment branch itself produced the rejection required by the prompt.

This is not a demonstrated production containment defect. The production code visibly re-canonicalizes the refreshed target and rejects canonical paths outside the registered root. It is a direct-test evidence gap only.

Severity summary:

- BLOCKER: 0
- MAJOR: 0
- MINOR: 1
- NOTE: 2

M09 is not yet marked PASS/CLOSED because the final M09C contract explicitly required direct retry-containment evidence. M10 remains blocked. A test-only evidence closure is sufficient; no M09 production redesign is required.

## 2. CONTRACT RECOVERY

M09C was required to:

1. R02C: replace raw duplicate-ordinal identity strings with a fixed-size deterministic key.
2. R02C: stream task-ID hashing or otherwise avoid giant raw identity allocations while preserving ordinary M09B ID semantics where feasible.
3. R02C: add fixed-key, task-ID-stability, and large-heading determinism tests.
4. E01C: make the retry-specific containment test directly exercise refreshed outside-root containment rejection.
5. E03C: complete the stale-source fixture with legacy settings and meaningful exact `SOURCE_EXPLICIT` edge assertions.
6. E05: run focused/full regression, governed publication, preserve scope, and publish truthful final evidence.

No M10, visible UI, X01 terminal-popup, X02 startup-audio, installer, or canonical asset work was permitted.

## 3. BRANCH / HEAD / DIFF SCOPE

Compared prompt base `fe535681...` to remote HEAD `0d70654e...`.

Changed scope is limited to:

- `H!veAI/src-tauri/src/task_intelligence.rs`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/codex-logs/M09C_BOUNDED_IDENTITY_FINAL_MICRO_FIX_LOG.md`

No visible UI production file, StartupIntro, Git Engine, watcher, M10 state machine, canonical visual asset, or installer change is present.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Audit conclusion |
|---|---|---|
| R02C fixed-size duplicate identity key | PASS | `HashMap<[u8; 32], usize>` and `duplicate_identity_key()` remove raw source-derived retained identity strings. |
| R02C streamed task identity hashing | PASS | `identity_digest_bytes()` and `update_normalized_text()` feed SHA-256 incrementally rather than building the prior giant formatted identity string. |
| Preserve ordinary task ID semantics | PASS | Direct test compares explicit and fallback IDs against the prior M09B logical concatenation digest. |
| Fixed-key oversized-heading evidence | PASS WITH NOTE | Key is directly `[u8; 32]`; test proves fixed representation and distinct keys. It does not separately simulate ordinal map counts, but production use is direct and clear. |
| Large-heading repeat determinism | PASS | Production M08 -> M09 parse repeated with 300 tasks under oversized heading and bounded persisted evidence. |
| E01C retry branch direct outside-root containment | PARTIAL | Failpoint substitutes `../outside.md`, but fixture does not create the outside file; `canonicalize()` can fail before explicit containment rejection. |
| E03C stale source/task removal | PASS | Named fixture removes configured stale source and proves stale M09 source/task deletion. |
| E03C retained M09 rows | PASS | Retained `TASKS.md` source and two retained tasks remain. |
| E03C legacy source/task/settings | PASS | Legacy source/task/settings are seeded and directly asserted after reparse. |
| E03C exact dependency edge | PASS | Fixture retains one meaningful explicit dependency and asserts `(COUNT, DISTINCT) == (1,1)`. |
| Full regression/security/build | PASS BY BUILDER CLAIM + SOURCE CONSISTENCY | Log records 53 focused parser tests, 190 Rust, 70 frontend, type/build/audit/fmt/check/build green. No contradictory source evidence found. |
| Governed production no-bundle publication | PASS BY BUILDER CLAIM | Stable EXE/icon/shortcut evidence recorded after verification refresh. |
| Final remote publication | PASS | M09C implementation, publication, verification refresh, and log are visible on remote branch. |
| Exact builder local==origin after final refresh commit | UNVERIFIED / NOTE | Equality is recorded at the pre-refresh HEAD; remote HEAD `0d70654e...` is visible, but independent access to builder-local HEAD after that commit is unavailable. |
| No M10/UI/X01/X02/installer scope creep | PASS | Audited diff does not contain those changes. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

### R02C
Builder claim: PASS.
Repository truth: PASS.

`parse_document()` now retains duplicate identities as fixed `[u8; 32]` keys. `duplicate_identity_key()` delegates to `identity_digest_bytes()`, and `task_id()` uses the same incremental digest helper with the ordinal appended as streamed bytes. No raw heading context is retained in a duplicate-key `String`, and the old giant `format!` identity allocation is gone.

`update_normalized_text()` reproduces the prior `split_whitespace() -> join(" ") -> to_ascii_lowercase()` logical normalization as a stream, so ordinary explicit/fallback ID semantics are preserved without copying the full normalized field into a new giant identity string.

### E01C
Builder claim: PASS.
Repository truth: PARTIAL.

The private `RETRY_RELATIVE_PATH_FAILPOINT` correctly substitutes the refreshed relative path after M08 rediscovery. However the test uses `../outside.md` without creating that file. Production then executes:

1. `refreshed_root.join(current.relative_path)`
2. `fs::canonicalize(refreshed_candidate)`
3. only after successful canonicalization, `if !refreshed_physical.starts_with(&refreshed_root)`

The test asserts only warning code `SOURCE_READ_FAILED`, which is also returned by canonicalization failure. It therefore does not prove the explicit refreshed containment rejection is the cause.

The production containment logic itself is source-level sound and remains accepted. The missing item is exact direct evidence.

### E03C
Builder claim: PASS.
Repository truth: PASS.

The strengthened fixture now contains retained M09 tasks with one explicit dependency, a stale custom source/task, legacy source/task/settings rows, stale-source removal, and exact SQL assertions including a unique `SOURCE_EXPLICIT` edge count.

## 6. FILE / SYMBOL EVIDENCE

Accepted production symbols:

- `parse_document()` fixed-size `HashMap<[u8; 32], usize>`
- `duplicate_identity_key()`
- `identity_digest_bytes()`
- `update_normalized_text()`
- `task_id()` streamed identity hash
- existing bounded persisted display/evidence fields
- selective persistence reconciliation

Evidence-only residual:

- `p01_retry_rechecks_physical_containment` does not create a canonicalizable outside-root target or assert the containment-specific warning message.

## 7. FOCUSED TEST EVIDENCE

Accepted:

- `r02c_duplicate_identity_key_is_fixed_size_for_oversized_heading`
- `r02c_task_ids_remain_stable_after_identity_streaming_refactor`
- `r02c_large_heading_many_tasks_remains_deterministic`
- `p07_removed_task_and_source_reconcile_only_stale_m09_rows`

Partial:

- `p01_retry_rechecks_physical_containment`

Required evidence-only correction:

- create a real outside-root file in a sibling/temp location reachable through the substituted relative path;
- make the substituted refreshed path canonicalize successfully outside the registered root;
- assert both warning code and containment-specific message, e.g. `refreshed source is outside registered root`;
- continue calling the real `read_authoritative_source()` production path.

## 8. REGRESSION EVIDENCE

Builder log reports:

- 53 task-intelligence focused tests PASS;
- 190 Rust tests PASS;
- 70 frontend tests PASS;
- typecheck/build/npm audit/cargo fmt/check/build PASS;
- publisher failure harness 9/9 PASS;
- governed production no-bundle publisher PASS.

No contradictory production regression is visible in M08 discovery authority, M09 source validation, SQLite UPSERT/stale reconciliation, adapters, status parsing, handoff merge, persisted bounds, or task-ID stability.

## 9. SECURITY / SAFETY REVIEW

PASS for production M09C scope.

- no network/AI/parser shell expansion;
- no project-file mutation by parser;
- registered-root canonical containment remains enforced;
- no permission broadening;
- no installer;
- no M10 workflow mutation.

The remaining E01C item strengthens proof of an already visible fail-closed production check.

## 10. ARCHITECTURE CONSISTENCY

PASS.

M08 remains the sole source-discovery authority. M09 consumes M08-owned AVAILABLE sources, performs bounded local parsing, persists neutral parser truth, and does not implement M10 operational state transitions. Stable M09 task IDs remain suitable anchors for future `task_events`.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

The current TASKS ledger truthfully leaves independent M09C re-audit/final M09 closure pending and M10 blocked. Historical M09/M09A/M09B audits remain preserved.

The M09C builder log overstates E01C as direct containment proof. This audit corrects that claim without modifying the historical builder log.

## 12. FINAL REPOSITORY STATE

Audited remote HEAD before this audit: `0d70654e8ab218a74a8f1f8901c8a57ce0f494e5`.

Implementation commit `63b73795...`, publication evidence `452d79d4...`, verification refresh `0d70654e...`, and the M09C log are visible remotely.

No force-push or destructive history rewrite is evident from the audited lineage.

## 13. OPEN CROSS-MILESTONE FINDINGS

Still queued outside M09 parser scope:

- X01: suppress visible Windows console/terminal windows from Git child processes.
- X02: restore startup intro audio while preserving reliable native startup behavior.

Do not mix these into the M09 evidence-only closure.

## 14. DEFECTS BY SEVERITY

### MINOR E01D - retry-containment direct test can pass before the containment branch

The outside relative path is not made canonicalizable. `SOURCE_READ_FAILED` can therefore come from `canonicalize()` rather than the explicit refreshed-root containment rejection.

### NOTE E05 - final builder-local equality after final refresh commit

Remote final commit is visible, but builder-local equality after that exact commit cannot be independently verified from repository state alone.

### NOTE R02C test precision

The fixed-key unit test proves `[u8;32]` representation and unique logical keys; it does not separately assert ordinal increments. Production map use is direct and existing duplicate-sibling tests cover deterministic ordinal behavior, so no additional production work is required.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

- Keep identity hashing centralized in the new digest helpers so future parser fields do not reintroduce raw identity buffers.
- Prefer containment-specific error enums/codes in a future hardening pass if diagnostic granularity becomes important; not required for M09 closure.

## 16. UNVERIFIED ITEMS

- Exact builder-local HEAD equality after `0d70654e...`.
- Real Windows symlink/junction retry escape remains environment-dependent, but the bounded non-symlink test hook can prove the same refreshed containment branch once corrected.
- ScrubBots/FMCG distinct special parser conventions remain intentionally UNVERIFIED with generic-safe fallback.

## 17. REGRESSION RISK

`LOW`

R02C is a focused internal identity representation refactor with direct stability tests. Remaining work is test-only evidence.

## 18. AUDIT CONFIDENCE

`HIGH`

Production source, direct tests, prompt contract, branch diff, builder log, and remote commit lineage were inspected independently.

## 19. FINAL VERDICT

`CONDITIONAL`

The last known M09 production MAJOR is closed. One test-only E01 containment-evidence correction remains before an unconditional M09 PASS/CLOSED verdict.

## 20. REQUIRED REMEDIATION

Perform one evidence-only micro-fix:

1. Do not change M09 production logic unless the stronger test reveals a real defect.
2. Strengthen `p01_retry_rechecks_physical_containment` so the substituted refreshed path points to an existing canonicalizable file physically outside the registered root.
3. Assert `SOURCE_READ_FAILED` and the containment-specific warning message from the refreshed containment branch.
4. Run the focused test and full regression gates.
5. Publish/update log truthfully, verify remote visibility, and stop.
6. Do not start M10 or X01/X02 in this evidence-only run.
