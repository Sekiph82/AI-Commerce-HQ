# M13B Stream-Safe Redaction and Durable Event Truth Remediation Log

Date: 2026-09-02  
Product: H!veAI  
Branch: `H!veAI`  
Authority: `docs/H!veAI/prompts/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_PROMPT.md`  
Findings remediated: `R30`, `R31`

## Final builder state

**M13B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE**

M13 remains open. M14 and M21 were not started.

## Synchronized preflight proof

- Repository root: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`.
- Branch: `H!veAI`.
- Before fetch, local `HEAD` and `origin/H!veAI` were both `0091a24`; `git rev-list --left-right --count HEAD...origin/H!veAI` returned `0 0`.
- `git fetch origin H!veAI` completed and advanced `origin/H!veAI` to `2ef85b5`.
- After fetch, divergence was `0 2` (local behind two commits) and the worktree had only the user-owned untracked files `start-demo.bat` and `task.md`.
- `git merge --ff-only origin/H!veAI` completed from `0091a24` to `2ef85b5`.
- No reset, rebase, force-push, destructive checkout, or discarded user work was used.
- The exact authoritative M13B prompt was read from the synchronized checkout at `H!veAI/docs/H!veAI/prompts/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_PROMPT.md`. The rendered GitHub URL returned a web cache miss, so the fetched checked-in copy was used as the synchronized authority.

## R30 root cause and final architecture

The residual defect was in `Capture::append()`/`read_stream()`: each arbitrary OS pipe read was independently converted with `String::from_utf8_lossy()` and redacted before immediate persistence. A marker such as `api_key` split as `api_` and `key=...` therefore escaped classification.

The final implementation adds `StreamRedactor` in `H!veAI/src-tauri/src/codex_adapter.rs`:

- It carries only an explicitly bounded `4096`-byte uncommitted record suffix across reads.
- It classifies complete newline-delimited records before any record is handed to `Capture` or the persistence queue.
- It flushes an unterminated final record at EOF.
- It preserves UTF-8 bytes across read boundaries and uses lossless conversion for valid UTF-8; invalid sequences are represented by the standard replacement behavior without panics.
- If an unterminated record exceeds the carry bound, it emits only the redaction marker and discards the rest of that record through its newline. Raw overlong content is never persisted.
- Existing marker classes remain protected: `api_key`, `apikey`, `token`, `password`, `secret`, `authorization`, and `sk-`.
- `Capture` now accepts only already-classified/redacted text. Output byte/event caps are applied after redaction and before queueing.

## R31 root cause and final architecture

The residual defect was that independent stdout/stderr reader threads each opened a SQLite connection and ignored `insert_event()` errors after capture accounting advanced. `SESSION_FINISHED` then reported captured counts even when fewer `STREAM_OUTPUT` rows were durable.

The final implementation adds a bounded `EventWriter`/`EventWriterHandle` path:

- stdout and stderr reader threads submit redacted stream events through one bounded `sync_channel` with capacity `32`.
- One writer thread owns the SQLite event connection for incremental stream output, eliminating competing stdout/stderr stream writers and preserving per-channel sequence values.
- Every stream write uses exactly `3` bounded attempts with `10 ms` then `25 ms` backoff between attempts. There are no unbounded retries and the Tauri main thread is not blocked.
- Successful durable writes increment separate per-channel persisted byte/event counters. Capture counters remain separate and are never treated as durable counters.
- A terminal stream persistence failure sets explicit degraded state, records `CODEX_STREAM_OUTPUT_PERSISTENCE_FAILED`, and attempts a `PERSISTENCE_DEGRADED` event. The final `SESSION_FINISHED` payload carries both captured and persisted counts, truncation, degradation, failure count, and bounded diagnostic fields.
- Legacy flat `stdoutBytes`/`stderrBytes` and `stdoutEvents`/`stderrEvents` fields now represent durable counts; explicit `stdoutCaptured*`, `stderrCaptured*`, `stdoutPersisted*`, `stderrPersisted*`, and degradation fields remove ambiguity.
- Session reconstruction orders structured stream rows by their channel-local sequence rather than timestamp/UUID tie order, preserving stdout/stderr channel identity and sequence behavior.
- Lifecycle completion remains truthful for `COMPLETED`, `FAILED`, `STOPPED`, and `CRASHED`, even when output evidence is degraded.

## Capture-vs-persisted truth model

For each channel, final evidence distinguishes:

| Evidence | Meaning |
| --- | --- |
| `*CapturedBytes` / `*CapturedEvents` | Safely redacted output accepted by the bounded capture layer. |
| `*PersistedBytes` / `*PersistedEvents` | `STREAM_OUTPUT` rows confirmed durable by the single writer. |
| `*Truncated` | Configured byte/event caps suppressed additional retained output. |
| `*PersistenceDegraded` / `outputDegraded` | One or more accepted stream events could not be durably written after bounded retries. |
| `persistenceDiagnosticCode` / `persistenceDiagnosticMessage` | Bounded diagnostic for the first persistence degradation. |

No final payload fabricates missing output. In the terminal-failure fixture, captured events were `1`, persisted events were `0`, durable stream rows were `0`, and `PERSISTENCE_DEGRADED` was explicit.

## Adversarial split-marker test matrix

`stream_redaction_handles_every_protected_marker_across_tiny_chunks` feeds each complete sensitive line through a controlled reader returning one byte per read. This necessarily exercises every split position, including marker-internal and value-internal boundaries, and covers more than two reads:

| Marker class | Controlled input |
| --- | --- |
| `api_key` | `api_key=super-secret-value` |
| `apikey` | `apikey=super-secret-value` |
| `token` | `token=super-secret-value` |
| `password` | `password=super-secret-value` |
| `secret` | `secret=super-secret-value` |
| `authorization` | `authorization: Bearer super-secret-value` |
| `sk-` | `sk-super-secret-value` |

Additional direct evidence:

- `stream_redaction_flushes_unterminated_sensitive_lines_without_leaking` covers a secret-bearing final line without a newline.
- `normal_utf8_content_crossing_chunks_reconstructs_from_durable_events` covers Unicode code points split across one-byte reads and reconstructs exact normal content.
- Every `STREAM_OUTPUT` payload is queried directly from `agent_events` and checked for the secret; checks are not limited to reconstructed session output.
- `stateful_redacted_output_keeps_durable_event_and_byte_caps` verifies `128` durable events and `64 KiB` retained-byte limits after stateful redaction.
- `stateful_redaction_remains_bounded_before_capture_caps_are_applied` verifies the `4096`-byte redaction carry bound.

## Forced persistence contention/failure evidence

Controlled `EventStore` injection points in the native test module provide deterministic SQLite-like `database is locked` failures without probabilistic races or user-project access:

- `concurrent_stdout_stderr_writes_use_one_bounded_writer_and_keep_channel_sequences`: simultaneous stdout/stderr readers, one writer, independent `[1, 2]` sequences for each channel.
- `transient_persistence_failure_recovers_with_bounded_retries`: two forced locked failures, third attempt succeeds, no degradation, exactly three attempts.
- `terminal_persistence_failure_is_explicit_and_never_claims_durable_output`: all three stream attempts fail; no stream row is recorded, `PERSISTENCE_DEGRADED` is recorded, captured count is `1`, persisted count is `0`, and final evidence is degraded.
- `durable_stream_rows_match_final_persisted_output_counts`: direct SQLite row count equals final persisted event count during normal execution.
- `final_evidence_preserves_truthful_terminal_states_and_counts`: final state/count model is checked for `COMPLETED`, `FAILED`, `STOPPED`, and `CRASHED`.

## Exact files changed

- `H!veAI/src-tauri/src/codex_adapter.rs`
- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
- `H!veAI/docs/H!veAI/codex-logs/M13B_STREAM_SAFE_REDACTION_AND_DURABLE_EVENT_TRUTH_REMEDIATION_LOG.md` (this immutable log)

The unrelated parent-root untracked files `start-demo.bat` and `task.md` were preserved, not staged, modified, or committed.

## Verification and publication commands

All commands were run from the H!veAI application root unless noted.

| Command | Result |
| --- | --- |
| `cargo test --manifest-path src-tauri/Cargo.toml --lib codex_adapter::tests -- --nocapture --test-threads=1` baseline | PASS, 9 pre-remediation tests |
| `cargo test --manifest-path src-tauri/Cargo.toml --lib codex_adapter::tests -- --nocapture --test-threads=1` M13B | PASS, 19 tests |
| `cargo test --manifest-path src-tauri/Cargo.toml --lib -- --nocapture --test-threads=1` | PASS, 306 tests |
| `npm.cmd test -- --run tests/m13-codex-adapter-focused.test.tsx` | PASS, 3 tests |
| `npm.cmd test -- --run` | PASS, 11 files / 98 tests |
| `npm.cmd run typecheck` | PASS |
| `npm.cmd run build` | PASS, Vite production build |
| `npm.cmd audit --audit-level=high` | PASS, 0 vulnerabilities |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | PASS |
| `cargo check --manifest-path src-tauri/Cargo.toml` | PASS; existing unrelated dead-code warnings only |
| `git diff --check` | PASS |
| `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/tests/publish-dev-qa-failure-harness.ps1` | PASS, all 9 rollback/safety cases |
| `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1` | First two non-elevated attempts failed at candidate smoke with exit 101 after successful production builds; stable executable remained unchanged |
| Elevated unchanged `powershell.exe -NoProfile -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1` | PASS; production Tauri `--no-bundle`, candidate/stable smoke, shortcut target/icon validation, and safe publication passed |
| Safe real Codex readiness probe: `codex.exe --version` | PASS, `codex-cli 0.130.0-alpha.5` |
| Real Codex coding operation against a user project | NOT RUN; deterministic local fixtures used |

The candidate smoke exit `101` was isolated to the non-elevated GUI/process context: the same release binary, including an exact temporary candidate-path copy under the publisher's temporary WebView2 profile, remained alive with title `H!veAI` and emitted `HIVEAI_FRONTEND_READY` when launched with permitted Windows process access. The successful elevated governed publication is the accepted publication result.

## Git proof

- Exact implementation commit: `61493b01d8fc9cce72c5e7d5495df0a1814d6991`.
- Implementation commit was pushed without force to `origin/H!veAI`.
- Post-implementation fetched local `HEAD`: `61493b01d8fc9cce72c5e7d5495df0a1814d6991`.
- Post-implementation fetched `origin/H!veAI`: `61493b01d8fc9cce72c5e7d5495df0a1814d6991`.
- Post-implementation `git rev-list --left-right --count HEAD...origin/H!veAI`: `0 0`.
- This immutable log is committed as a separate documentation commit immediately after the implementation commit and pushed to the same branch. The final post-log fetched SHA and divergence are reported in the delivery handoff because recording a commit's own SHA inside that commit would make the log self-referential.

## Governance truth

- M13 remains not closed and remains pending independent strict re-audit plus user native/visual acceptance.
- M13B is remediation complete pending audit.
- Strict completed roadmap count remains `13 / 20 = 65%`.
- M14 remains `PLANNED/BLOCKED` and was not started.
- M21 remains planned/not started.
- Historical M13/M13A prompts, logs, and audits were not rewritten.
