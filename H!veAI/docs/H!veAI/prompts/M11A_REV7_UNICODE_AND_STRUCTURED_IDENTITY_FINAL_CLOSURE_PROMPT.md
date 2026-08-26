# M11A REV7 - Unicode + Structured Identity Final Closure

## Authority

This is the single authoritative Codex prompt for the next H!veAI run.

It is a bounded continuation of M11A and exists only to close the two production findings from:

`H!veAI/docs/H!veAI/audits/M11A_REV6_DEEP_IDENTITY_STRICT_REAUDIT.md`

Open findings:

- R24 / MAJOR - non-ASCII bounded evidence is erased by ASCII-only identity normalization;
- R25 / MAJOR - Quality equivalence is reconstructed from display-formatted `detail` and truncates colon-bearing labels.

Do not create M11B/M11C or another numbered roadmap milestone.
Do not start M12.
Do not begin M21 migration work.

Strict completed roadmap count remains **11 / 20 = 55%** until independent M11 closure and user native/visual acceptance.

Preserve every prior closure not explicitly reopened below.

---

# Mandatory preflight and Task 0

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run first:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, rewrite history, create `H!veAI\.git`, delete user work, or stage unrelated parent-root files.

Preserve user-owned untracked files including:

- `start-demo.bat`
- `task.md`

Read before editing, in this order:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/audits/M11A_REV6_DEEP_IDENTITY_STRICT_REAUDIT.md`
6. `H!veAI/docs/H!veAI/audits/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_STRICT_REAUDIT.md` as historical prior audit only
7. `H!veAI/docs/H!veAI/prompts/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_MICRO_CLOSURE_PROMPT.md`
8. `H!veAI/docs/H!veAI/codex-logs/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_MICRO_CLOSURE_LOG.md`
9. current `H!veAI/.hiveai/PROJECT_DASHBOARD.md`
10. current `H!veAI/src-tauri/src/command_center.rs`
11. current `H!veAI/src-tauri/src/project_dashboard.rs`
12. watcher source and existing M11A focused tests
13. this REV7 prompt in full

Before production edits, synchronize prospective current-status truth only in the existing canonical/current status files normally used by this project:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md` if required by the established dogfood contract

They must state truthfully:

- M00-M10 remain PASS/CLOSED;
- strict completed roadmap count remains 11/20 = 55%;
- original M11 remains historical strict-audit FAIL;
- REV6 implementation remains historical implementation-complete;
- prior REV6 source audit PASS remains historical evidence but is superseded for closure by the deeper REV6 audit;
- deeper REV6 audit = FAIL with R24 and R25 open;
- M11A REV7 = ACTIVE;
- M11 NOT CLOSED;
- M12 BLOCKED;
- user native/visual acceptance pending.

Historical prompts, audits and builder logs are immutable. Do not edit or delete them.

---

# Canonical UI Assets and accepted native behavior

REV7 is not a UI redesign.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve without regeneration/substitution:

- sidebar logo: `H!veAI/src/assets/hiveai-logo.png`;
- background: `H!veAI/src/assets/hiveai-app-background.png`;
- opening video: `H!veAI/src/assets/opening-video.mp4`;
- stable shortcut icon: `H!veAI/dev-bin/H!veAI.ico`;
- tracked Akilta wordmark used by the topbar attribution.

Required unchanged hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve accepted behavior:

- no bottom footer band;
- Akilta attribution remains in the topbar between Workspace/title and Search Workspace;
- visible credit remains `Built with ♥ for maximum productivity by Akilta`;
- whole attribution remains one clickable/focusable target;
- title remains `Developed by Akilta`;
- destination remains `https://www.akilta.com/`;
- native Chrome-only safe external open remains parameterless;
- no Edge fallback;
- no terminal/console flash;
- startup video remains audible and no same-process replay;
- Advanced source inventory remains available;
- current Command Center one-screen layout remains unchanged;
- no installer work;
- no M12 work;
- no M21 migration work.

Do not modify any external registered project repository. Do not touch Bulk Edit.

---

# R24 / MAJOR - Unicode-preserving operational identity normalization

## Defect

REV6 removed the former 256-character clipping but current `normalize_attention_source()` still tokenizes with `is_ascii_alphanumeric()`.

That erases every non-ASCII character before equality, duplicate detection and hashing.

The Project Dashboard parser permits bounded UTF-8 `String` materialized facts. Therefore two real facts can differ in Turkish/accented/CJK/Greek/Cyrillic/other Unicode content and still collapse to the same normalized identity.

Example class:

```text
"ş blocker"
"ç blocker"
```

Both can normalize to the same ASCII remainder.

This can silently drop a blocker through `blocker_keys`, churn/collapse materialized IDs, or falsely suppress Project Dashboard evidence against stronger persisted evidence.

## Required design

Create one clearly named operational identity-normalization path whose contract is documented in code.

Requirements:

1. Consume the complete already-bounded UTF-8 scalar.
2. Never remove a Unicode letter/number merely because it is non-ASCII.
3. Deterministically fold whitespace.
4. Apply case normalization only in a Unicode-preserving way.
5. Treat punctuation according to an explicit documented rule; punctuation normalization must not silently destroy distinguishing semantic content needed by R25.
6. Do not transliterate multiple scripts/characters into a shared ASCII representation.
7. Keep emitted IDs fixed-size and deterministic.
8. Do not use random UUIDs.
9. Distinct supported bounded inputs must stay distinct when they differ in meaningful Unicode content.
10. Truly equivalent normalized inputs must remain deterministic duplicates.

Prefer the least lossy normalization compatible with the already accepted matching semantics. Operational truth has priority over aggressive fuzzy normalization. If equivalence is not provable, preserve both facts.

## Important implementation constraint

Do not use a custom hand-written ASCII transliteration table.
Do not silently strip accents/diacritics unless a pre-existing explicit product contract requires it.
Do not introduce an unbounded normalization dependency without review.

If a dependency is required for Unicode case/normalization, keep it minimal, deterministic, pinned through the existing Rust dependency workflow, and include dependency audit evidence. If standard-library behavior is sufficient for a conservative closure, prefer that.

---

# R25 / MAJOR - Preserve structured evidence identity; never reconstruct it from display text

## Defect

Materialized Quality currently creates human-readable detail:

```text
<label>: <result>
```

Then `attention_identity()` recovers the label using `split_once(':')`.

A valid label such as:

```text
build: windows
```

is therefore reconstructed as only:

```text
build
```

A persisted TEST_RUN/AUDIT source `build` can falsely suppress the distinct Project Dashboard failure `build: windows` for the same task.

This violates the conservative R20 rule because equivalence is inferred from a lossy display parser rather than proven from the original structured fields.

## Required design

Operational evidence identity must be carried from the original structured source to deduplication.

Human-facing fields such as `title`, `detail`, rendered punctuation and display prefixes/suffixes are presentation only.

Use an internal structured identity representation. A suitable architecture is one of:

- an internal non-serialized identity field on the attention model;
- a dedicated internal wrapper used during snapshot assembly/dedup before serialization;
- another bounded typed structure that preserves original project/task/evidence-class/source identity without changing user-facing semantics.

Choose the smallest safe design.

### Required identity sources

At minimum preserve exact original bounded sources for:

- Project Dashboard Quality -> `MaterializedFact.label`;
- TEST_RUN -> persisted test `command`;
- AUDIT -> persisted audit `summary` under the existing accepted R20 semantics;
- Project Dashboard WAITING -> original `waiting_on`;
- Project Dashboard BLOCKER -> original blocker scalar;
- PERMISSION -> persisted permission kind/source used by current accepted matching;
- WORKFLOW -> persisted workflow source/detail used by current accepted matching.

Do not parse these identities back out of display-formatted `detail` if the structured source existed earlier in the pipeline.

### Conservative match rule remains mandatory

Preserve REV5/REV6 policy:

- project IDs must match;
- task IDs must be proven where the accepted rule requires them;
- evidence classes must be one of the explicitly accepted stronger/weaker pairs;
- normalized structured source identity must match;
- if any required identity element is absent or ambiguous, do not suppress the dashboard item.

The safe failure mode is duplicate visible evidence, not silent disappearance.

---

# Required direct adversarial tests

Add tests that fail against current REV6 and pass only after REV7 production fixes.

## Unicode identity tests

1. Two blockers with the same ASCII context and different Turkish/non-ASCII content both survive.
2. Their IDs are distinct, deterministic, fixed-size, and contain no raw source text.
3. A truly identical Unicode blocker still collapses to one logical blocker.
4. Insert an unrelated preceding blocker; prior Unicode-derived IDs remain unchanged.
5. Repeated snapshots produce the same IDs.
6. Two undated activity facts differing only in non-ASCII content receive distinct stable IDs.
7. A long Unicode scalar close to the parser byte bound remains bounded and deterministic without panic/corruption.
8. `needs_attention` equals final post-dedup attention length.

## Structured Quality tests

9. Dashboard Quality label `build: windows` + persisted TEST_RUN `build` for the same task remain distinct.
10. Dashboard Quality label `build: windows` + persisted AUDIT `build` remain distinct.
11. Persisted source `build: windows` exactly matching the dashboard label suppresses the weaker dashboard duplicate under the accepted rule.
12. Label with multiple colons, e.g. `build: windows: release`, remains identity-stable.
13. Display `detail` may contain extra punctuation/result text without becoming identity authority.
14. A test must prove that changing only display formatting while keeping the same structured identity does not change dedup semantics.

## Combined Unicode + delimiter tests

15. Dashboard Quality `dağıtım: türkiye` must remain distinct from persisted `dagitim` or `dağıtım` unless the accepted normalization proves exact identity.
16. A true structured Unicode + colon match must still deduplicate correctly.
17. Distinct Unicode blocker/activity/Quality cases must remain stable under unrelated preceding insertion.

All fixtures must remain within existing parser/materialized limits. Do not create giant or pathological test files.

---

# Preserve all prior closures

Do not regress:

- REV6 removal of 256-character clipping;
- fixed-size SHA-derived materialized IDs;
- long ASCII blocker/activity/Quality collision tests;
- R19 WAITING truth;
- R20 conservative provenance-aware dedup principle;
- R21 Quality header filtering;
- R15 single-dashboard watcher architecture and actual-notify evidence;
- R17 header/front-matter accounting;
- R18 materialized enum validation;
- M10 workflow truth remains stronger than materialized evidence;
- M10 queue duplicate suppression;
- exact single-dashboard event filtering;
- dashboard signal bounded M09 refresh;
- unknown task truth remains unknown instead of fake zero;
- last-good M09 survives refresh failure;
- legacy ABSENT dashboard remains informational;
- malformed/stale/unavailable actionable manifest truth remains visible;
- audit/test actors remain null unless persisted evidence proves them;
- materialized current-work rows do not alter authoritative task totals;
- materialized recent activity remains undated unless a real timestamp exists;
- H!veAI keeps a single `.hiveai/PROJECT_DASHBOARD.md` live contract;
- external project source inventories remain Advanced/internal evidence;
- current accepted UI/native shell behavior listed above.

---

# Testing and verification gates

Rust assertions must actually execute. `cargo test --no-run` is not acceptance.

Use the established shell-local Windows common-controls workaround only if required. Do not change Windows globally.

At minimum execute and record exact results for:

1. focused new R24 Unicode blocker/activity identity tests;
2. focused new R25 structured Quality identity tests;
3. combined Unicode + delimiter adversarial tests;
4. existing REV6 R23 tests;
5. existing R19-R22 tests;
6. focused Project Dashboard parser tests, including bounded UTF-8 scalar behavior;
7. focused watcher/single-dashboard tests, including actual notify path;
8. full Rust native suite with assertions executed;
9. focused frontend Command Center/Task Sources/Akilta shell tests;
10. full frontend suite;
11. TypeScript typecheck;
12. production frontend build;
13. dependency audit at high severity;
14. `cargo fmt --all -- --check`;
15. `cargo check`;
16. `git diff --check`;
17. canonical background/video SHA verification;
18. X01 terminal suppression regression;
19. X02 startup audio/replay regression;
20. governed QA publication;
21. publisher failure harness.

If any required test cannot run, record the exact reason as UNVERIFIED. Never fabricate PASS.

Do not mark user native/visual acceptance PASS yourself.

---

# Git evidence discipline

Use exact Git output only.

Before implementation commit record:

```powershell
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

After implementation commit is pushed and fetched record:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Never synthesize or expand short SHAs.

After the immutable REV7 builder log itself is committed/pushed, run the same commands again and report exact final post-log equality in the final Codex response without rewriting the log.

---

# Builder log

Create exactly one new immutable builder log:

`H!veAI/docs/H!veAI/codex-logs/M11A_REV7_UNICODE_AND_STRUCTURED_IDENTITY_FINAL_CLOSURE_LOG.md`

The log must contain:

- exact starting HEAD/origin SHAs and left/right count;
- exact implementation commit full SHA read from Git;
- exact files changed;
- R24 production design and why Unicode evidence cannot collapse;
- R25 structured identity design and proof display text is no longer identity authority;
- direct failing-before-fix adversarial test descriptions;
- exact focused/full test commands and results;
- preserved R19-R23/R15 evidence;
- publication/failure-harness results;
- canonical asset hashes;
- proof no external registered project repository or Bulk Edit was touched;
- exact post-implementation local HEAD;
- exact fetched `origin/H!veAI` HEAD;
- exact post-implementation left/right count.

Final builder state must remain:

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M11 remains NOT CLOSED.
M12 remains BLOCKED.

Stop after the REV7 log is published and final Git equality is reported. Do not start M12 or M21.
