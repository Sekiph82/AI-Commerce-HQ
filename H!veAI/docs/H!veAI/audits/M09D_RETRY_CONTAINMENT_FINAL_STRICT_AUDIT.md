# M09D Retry Containment Final Strict Audit

Date: 2026-08-25
Branch: `H!veAI`
M09D prompt base: `46196151e6566d23461e7385565a227d9ab85675`
Implementation/test/tracker commit: `0e4e7f1f46e01be8b21bd3c9b39fa5340ed840a4`
Builder verification-log commits: `e700e50e0fe5ff45ac9ed8882fce173200898530`, `af4dcb424adbc250b8efa0a104cf8b0014117f51`
Audited remote branch HEAD before this audit: `af4dcb424adbc250b8efa0a104cf8b0014117f51`

## 1. VERDICT

`PASS`

M09D closes the single remaining M09C evidence defect. The strengthened retry-containment test now creates a real canonicalizable file outside the registered project root, drives the real production retry path to that file through a private `cfg(test)` substitution, and asserts both the exact warning code and the containment-specific warning message.

Severity summary:

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 2

This audit closes M09 Task Intelligence Parser at source/test level. M09 is eligible for final `PASS/CLOSED` tracker state. M10 remains blocked only by the separate pre-M10 native UX hotfix queue X01/X02.

## 2. CONTRACT RECOVERY

M09D was intentionally test-only. It was required to:

1. preserve M09C production parser behavior unchanged;
2. create a real file outside the registered project root;
3. make the private retry failpoint resolve to that real outside-root file;
4. execute the real `read_authoritative_source()` retry path;
5. assert warning code exactly `SOURCE_READ_FAILED`;
6. assert warning message exactly `refreshed source is outside registered root`;
7. keep the test hook private to test builds;
8. run focused/full regression and governed no-bundle QA publication;
9. preserve UI/assets/M10/X01/X02 scope boundaries;
10. push the log and verify final branch synchronization.

## 3. BRANCH / HEAD / DIFF SCOPE

Compared prompt base `46196151...` to remote implementation/log HEAD `af4dcb42...`.

Only three repository paths changed:

- `H!veAI/src-tauri/src/task_intelligence.rs`
- `H!veAI/TASKS.md`
- `H!veAI/docs/H!veAI/codex-logs/M09D_RETRY_CONTAINMENT_EVIDENCE_CLOSURE_LOG.md`

The Rust change is test-only: one `#[cfg(test)]` serialization lock and strengthened test bodies. No production parser branch, IPC, persistence, UI, Git Engine, watcher, StartupIntro, canonical asset, M10 state-machine code, or installer code changed.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Audit conclusion |
|---|---|---|
| Real canonicalizable outside-root fixture | PASS | Test writes a sibling file in the project temp directory parent. |
| Retry failpoint points to outside-root file | PASS | Failpoint receives `../<unique-sibling-file>`. |
| Real production retry path used | PASS | Test invokes production `read_authoritative_source()`. |
| Canonicalization can succeed before containment rejection | PASS | Outside file is created before invocation and deleted only after the call returns. |
| Exact warning code | PASS | Asserts `SOURCE_READ_FAILED`. |
| Exact containment-specific warning message | PASS | Asserts `refreshed source is outside registered root`. |
| Test hook remains test-only | PASS | Failpoints/lock are behind `#[cfg(test)]`. |
| Production parser unchanged | PASS | M09D diff changes only test-only symbols/test bodies inside parser source. |
| Parallel focused-test isolation | PASS | `RETRY_TEST_LOCK` serializes the two tests sharing global retry failpoints. |
| Focused parser suite | PASS BY BUILDER CLAIM + SOURCE CONSISTENCY | Log records 53/53 after isolation correction; test body/source are consistent. |
| Full Rust/frontend regression | PASS BY BUILDER CLAIM | Log records 190 Rust, 70 frontend plus type/build/audit/fmt/check/build green. |
| Governed QA publication | PASS BY BUILDER CLAIM | Stable EXE/icon/shortcut hashes and no-bundle publication are recorded. |
| No UI/M10/X01/X02 scope creep | PASS | Diff contains no such production files. |
| Final remote visibility | PASS | Final log commit `af4dcb42...` is current remote `H!veAI` HEAD before this audit. |
| Builder local/origin equality | PASS BY BUILDER CLAIM / REMOTE CONSISTENT | Log records `0 0`; independent auditor can verify remote final commit visibility, not the builder's local working tree after session exit. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The key builder claim is correct.

The strengthened test obtains the registered project temp directory, creates a uniquely named file in its parent, points the retry substitution at `../<name>`, then calls `read_authoritative_source()`. The outside file remains present until after the reader returns, so `canonicalize()` has a real target and the test cannot pass merely because the target is missing.

The test then asserts both:

- `warning.code == "SOURCE_READ_FAILED"`
- `warning.message == "refreshed source is outside registered root"`

This directly distinguishes the intended containment rejection from a generic canonicalization/read failure.

The log also truthfully records a first parallel focused run with shared failpoint interference and the subsequent test-only `RETRY_TEST_LOCK` correction. That historical failure was not erased.

## 6. FILE / SYMBOL EVIDENCE

Accepted M09D symbols:

- `RETRY_TEST_LOCK: OnceLock<Mutex<()>>` under `#[cfg(test)]`.
- `p01_second_change_after_refresh_is_skipped_after_exactly_one_retry` obtains the retry lock before touching the shared failpoint.
- `p01_retry_rechecks_physical_containment` obtains the same lock, creates the outside sibling file, sets `RETRY_RELATIVE_PATH_FAILPOINT`, invokes the real reader, removes the file, and asserts exact code/message.

Production containment remains:

1. rediscover current M08 source;
2. construct refreshed candidate;
3. canonicalize refreshed candidate;
4. reject if `!refreshed_physical.starts_with(&refreshed_root)`;
5. only then read/hash the refreshed source.

No production-path bypass was added.

## 7. FOCUSED TEST EVIDENCE

The strengthened test now proves exactly the previously missing branch condition.

The original M09C fixture could false-pass because `../outside.md` did not exist. M09D removes that ambiguity by creating the outside target first and asserting the containment-specific message.

The shared static failpoints created a real parallel-test race during the builder's first focused run. Serializing only the tests that mutate those failpoints is an appropriate test-only correction and does not change production concurrency behavior.

## 8. REGRESSION EVIDENCE

Builder log records:

- strengthened containment test PASS;
- 53 task-intelligence focused tests PASS;
- 190 Rust tests PASS;
- 70 frontend tests PASS;
- TypeScript typecheck PASS;
- frontend production build PASS;
- npm audit high PASS with 0 vulnerabilities;
- cargo fmt/check/build PASS;
- publisher failure harness PASS, 9 governed assertions;
- governed production `--no-bundle` QA publisher PASS.

No contradictory source-level evidence was found.

## 9. SECURITY / SAFETY REVIEW

PASS.

- No production permission expansion.
- No unrestricted shell/filesystem capability added.
- No network/AI behavior introduced.
- No project file mutation by production parser.
- Outside-root file exists only in a temp test fixture and is removed after the call.
- Test hooks remain `cfg(test)` only.
- No installer.
- X01/X02 intentionally remain separate.

## 10. ARCHITECTURE CONSISTENCY

PASS.

M08 remains the sole source-discovery authority. M09 remains a bounded deterministic parser over M08-owned AVAILABLE sources. M09 still does not implement M10 workflow transitions. Stable task identity/persistence/evidence contracts from M09C remain unchanged.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

PASS for pre-audit state.

`TASKS.md` correctly kept M09 open and M10 blocked while M09D awaited independent audit. The M09D log preserved the initial 51-pass/2-fail focused run and recorded the test-isolation correction rather than rewriting history.

After this audit, trackers should be updated prospectively to M09 `PASS/CLOSED`, strict progress `10 / 20 = 50%`, and pre-M10 UX hotfix X01/X02 as the only gate before M10.

## 12. FINAL REPOSITORY STATE

Before this audit, remote `H!veAI` HEAD was:

`af4dcb424adbc250b8efa0a104cf8b0014117f51`

The implementation/test commit and both M09D log commits are remotely visible in sequence.

Builder-local equality after the final pushed commit is supported by the builder log but cannot be independently reconstructed from GitHub alone. This does not block M09 because remote branch truth and all required committed evidence are present.

## 13. OPEN CROSS-MILESTONE FINDINGS

These are not M09 defects and remain intentionally open:

- X01: visible Windows console/terminal windows created by Git child processes while H!veAI is running.
- X02: startup intro video is muted even though the canonical MP4 contains audio.

Both must close before M10 starts.

## 14. DEFECTS BY SEVERITY

No open M09D or M09 production defects.

Notes:

- NOTE N01: builder-local final equality is supported by builder evidence; remote final commit visibility is independently verified.
- NOTE N02: ScrubBots/FMCG distinct special parser conventions remain intentionally UNVERIFIED with generic-safe fallback and no special confidence bonus. This was accepted by prior M09 audits and is not an open defect.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

No remediation required for M09 closure.

Future parser extensions should continue using fixed-size identity working keys, bounded persisted fields, M08 source authority, explicit evidence locators, and test-isolated global failpoints.

## 16. UNVERIFIED ITEMS

- ScrubBots distinct special convention: UNVERIFIED by design.
- FMCG ERP distinct special convention: UNVERIFIED by design.
- Builder-local worktree equality after session exit cannot be independently inspected through GitHub; remote final commit state is verified.

None block M09.

## 17. REGRESSION RISK

`LOW`

Reason: M09D changes test-only code plus tracker/log documentation. Production parser behavior remains the already audited M09C implementation.

## 18. AUDIT CONFIDENCE

`HIGH`

Reason: the remaining evidence defect was narrow, the exact test body is visible, the production containment branch is visible, the changed-file scope is tiny, and no production code changed.

## 19. FINAL VERDICT

`PASS`

M09D closes E01D. The complete M09 Task Intelligence Parser milestone now has no open BLOCKER, MAJOR, or MINOR acceptance defect and is ready to be marked `PASS/CLOSED`.

## 20. REQUIRED REMEDIATION

None for M09.

Do not start M10 yet. Close the separate pre-M10 native UX hotfix queue X01/X02 first.