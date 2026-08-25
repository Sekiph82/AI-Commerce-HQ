# M08 Task Source Discovery — Strict Audit

Date: 2026-08-25

## Verdict

`FAIL`

The M08 implementation is substantial and directionally correct, but the milestone does not yet satisfy its own production/evidence contract. No M09 work is authorized.

Builder logs were treated as claims only. This audit checked the active prompt, branch history, Rust production source, frontend production source, focused tests, ACL/capability wiring, and current tracker truth.

## Audited repository state

Authoritative prompt:

`H!veAI/docs/H!veAI/prompts/M08_TASK_SOURCE_DISCOVERY_PROMPT.md`

Prompt commit / original implementation base:

`ca92320be18cb0a55d10be7d2fc82ca9255b9de8`

Tracker normalization merged during the builder run:

`c636b5491ebaf5d70fa46d71d82d7ebd6b4d3f97`

Primary implementation commit:

`d21bafe556b09b357719006152dd71a9e8ccaaed`

Tracker merge commit:

`bef6ec8384be9de40f7ced44f203480b8e818779`

Focused-evidence follow-up / audited branch HEAD:

`6e638f8cb07e29ee8b7cdd6d31ac351495335d64`

The current branch is ahead of the normalized tracker base by three commits. Canonical PNG/MP4 assets were not changed in the audited diff.

## Accepted implementation

### A01 — M08 domain boundary

`PASS`

The implementation stays at source discovery. It does not parse normalized tasks, create workflow state, start agents, call network services, or populate `tasks` / `task_sources` through the M08 production path.

### A02 — Standard bounded discovery shape

`PARTIAL`

Root task/planning names, approved directories, ignore trees, bounded hashing, metadata collection, and non-Git registered project support exist in production.

However, the root handoff variant rule only matches names that end with `handoff.md`; it does not implement the prompt's general reasonable `*handoff*.md` family.

### A03 — Physical containment

`PASS WITH UNVERIFIED WINDOWS LINK CASE`

Existing custom targets and traversed entries are canonicalized and checked under the registered physical root before reading. Ordinary absolute outside-root and `..` escapes are directly rejected.

The Windows symlink test correctly records `UNVERIFIED` when link creation fails with OS error 1314 instead of substituting an ordinary outside path.

### A04 — Narrow native IPC / ACL

`PASS`

The five M08 commands are registered behind a dedicated `allow-task-source-discovery` permission and main-window capability entry. No unrestricted filesystem or shell API was added.

### A05 — Task Sources UI existence

`PASS`

The native `/tasks` route is a real Task Sources inventory workspace using the selected Registry project, with source rows, loading/empty/error surfaces, rescan, and custom path controls. Browser preview does not invoke native task-source commands.

## MAJOR findings

### F01 — Discovery bounds are not actually closed and limit hits are silent

Severity: `MAJOR`

The prompt requires bounded discovery and says a limit hit must return a structured warning rather than silently stopping.

Production `walk_bounded` stops when `output.len() >= MAX_CANDIDATE_FILES`, but it emits no structured limit warning. More importantly, `output.len()` counts accepted source rows, not filesystem entries visited. A very large approved directory containing mostly non-source files can therefore enumerate an unbounded number of entries before 512 candidates are accumulated.

The recursion bound is also off by one: `if depth > MAX_DISCOVERY_DEPTH` allows processing files while `depth == 4`, which can yield a stored relative depth of 5. The focused test only proves one depth-4 example and does not exercise the actual boundary edge.

Required closure:

- bound visited directory entries/work, not only accepted candidates;
- make depth semantics exact and test the first rejected depth;
- surface structured discovery warnings for depth/candidate/work limits;
- prove the warning through production-path tests.

### F02 — Custom source path contract is incomplete

Severity: `MAJOR`

The prompt explicitly requires add/remove/update custom-path operations and deterministic ordering by configured custom order, then freshness, then normalized path.

Production provides list/add/remove only. There is no update/reorder operation or persisted configured order. All custom sources use priority 0 and are effectively ordered by path.

`custom_path_remove` also compares stored `normalized_path` directly against a lowercased normalized input, so remove-by-path with case-equivalent spelling can fail even though add/dedupe treats paths case-insensitively.

Required closure:

- add a bounded update operation, including deterministic custom order where appropriate;
- persist explicit custom order;
- implement the required tie ordering;
- normalize both sides for remove-by-path equivalence;
- add direct production tests for update/order/remove-by-equivalent-path.

### F03 — Persistence reconciliation is too destructive and metadata lacks schema/version ownership

Severity: `MAJOR`

`reconcile()` executes `DELETE FROM project_sources WHERE project_id = ?1` before reinserting the current M08 inventory. `project_sources` predates M08 as a generic project-source table. This blanket delete can remove pre-existing/non-M08 rows for the project.

The prompt also requires structured `metadata_json` to be explicit enough for M09 to consume safely. Current serialized metadata has no schema/version or M08 ownership marker.

Required closure:

- add explicit M08 metadata schema/version/ownership in `metadata_json`;
- reconcile only M08-owned rows, preserving unrelated/legacy `project_sources` rows;
- adopt/migrate compatible existing M08 rows deterministically rather than blanket-delete all project rows;
- directly inspect `project_sources` in focused tests after repeated scan, update, deletion, and preservation cases.

### F04 — Frontend stale-response and mutation transitions are not proven and contain a real stale-operation risk

Severity: `MAJOR`

The prompt requires a delayed project-A response to be unable to overwrite newer project-B UI.

The focused test named `project_change_cannot_leave_stale_prior_inventory` only changes selection and asserts that a B IPC call occurred. It never delays A, resolves B first, then resolves stale A last and verifies the visible inventory remains B.

The plain list refresh path has a request counter, but custom add/remove creates a real additional race: an add/remove started for project A can finish after selection moved to B, then call `refresh(selectedProjectId)` from the stale A render. That new A refresh receives the newest request id and can overwrite B inventory.

Required closure:

- guard custom mutation completion with the project identity/current request generation that initiated it;
- add one-mounted-app delayed A -> select B -> resolve B -> resolve stale A test;
- add stale custom-add and stale custom-remove completion tests so an old project cannot reclaim the current view.

### F05 — Frontend focused tests overstate transition coverage

Severity: `MAJOR`

Several test names/claims do not exercise what they say:

- `custom_add_and_remove_use_native_commands` tests add only; no remove action is exercised.
- `empty_and_error_states_are_truthful` tests empty only; no rejected native call/error state is exercised.
- rescan tests assert the discover command is called but do not prove the visible inventory changes to the returned rescan result.
- the delayed stale project response required by the prompt is absent.

The tracker currently marks mounted-app stale-response-race evidence complete, which is not supported by the focused suite.

Required closure:

- real remove transition;
- real native error transition;
- rescan visible-row replacement assertion;
- same-mounted-app delayed stale-response assertion;
- truthful TASKS/log wording.

### F06 — Rust evidence matrix is incomplete despite the 22/22 count

Severity: `MAJOR`

The prompt requires persistence tests to directly inspect `project_sources` rows after production discovery. Current idempotency/hash/reconciliation tests mostly inspect returned values, not DB rows.

The deleted-CUSTOM case does not create an available custom target and then delete it. It configures an already-missing path, so the required AVAILABLE -> MISSING transition is not proven.

Unreadable-source isolation is not directly tested and is not recorded with a deterministic `UNVERIFIED` reason.

The candidate-limit test proves only `len <= 512`; it does not prove a structured limit warning or bounded filesystem work.

Required closure:

- direct SQL inspection of `project_sources` after repeated discovery;
- AVAILABLE custom target -> delete -> configured MISSING transition;
- deterministic unreadable-file failure injection/test or exact UNVERIFIED reason if the platform cannot force it;
- direct structured limit warning evidence.

### F07 — Required immutable builder-log evidence is incomplete

Severity: `MAJOR`

The original prompt requires exact focused Rust test names/results, exact focused frontend test names/results, final local HEAD, final `origin/H!veAI` HEAD, and explicit equality proof.

The M08 log records aggregate counts only and explicitly says final local/remote HEAD values will be recorded later. At audited HEAD the immutable log still does not contain those required values.

Historical M08 builder log must not be rewritten now. A remediation log must prospectively close this evidence gap.

## MINOR findings

### N01 — Archived project status is not explicitly rejected

The M08 prompt scopes discovery to ACTIVE/MISSING registered projects. Production resolves any registered project id through `fetch_project` and does not reject `ARCHIVED` status. Native UI normally excludes archived projects, but the native command boundary should enforce the documented scope or document a deliberate broader policy.

### N02 — Existing-path custom status listing does not re-enforce physical containment

`custom_paths_list` computes `CONFIGURED/MISSING` using the stored root/path without physical canonical containment. Discovery itself remains safe because `collect_file` canonicalizes before reading, but a formerly safe configured path changed into an outside symlink can be reported `CONFIGURED` while its source silently disappears from discovery. Prefer a containment-aware status/warning.

## Evidence-integrity summary

Builder log claim: Rust 22/22 and frontend 13/13.

Audit conclusion: counts are not enough. Multiple required cases are combined, weaker than their names, or absent, and several production contracts remain incomplete.

## Security / regression assessment

- No unrestricted filesystem command found.
- No network or shell execution added for M08.
- Physical containment is directionally strong.
- No canonical visual asset byte change found in the audited implementation range.
- M09 remains unstarted.
- Regression risk: `MEDIUM-HIGH` until F01-F06 are closed because source discovery is filesystem-facing and persistence-backed.

## Final gate

`M08 RESULT = FAIL`

M08 must remain the active milestone. M09 is blocked.

Create one bounded M08 strict-closure remediation only. Do not split into M08.01/M08.02/etc. Do not start M09. User visual acceptance of the final Task Sources UI remains pending until the remediated native build is published.