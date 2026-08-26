# M11A REV5 - Final Attention Truth and Identity Micro-Closure

## Authority

This is the single authoritative Codex prompt for the next H!veAI run.

It is a bounded continuation of M11A and exists only to close R19-R22 from:

`H!veAI/docs/H!veAI/audits/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_STRICT_REAUDIT.md`

Do not split this into M11B/M11C or a new numbered milestone.
Do not start M12.

Strict completed roadmap count remains **11 / 20 = 55%** until independent M11 closure.

Preserve every REV3/REV4 closure not explicitly reopened below.

---

# Mandatory preflight and Task 0

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run first:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
git merge --ff-only origin/H!veAI
```

Never reset, rebase, force-push, rewrite history, create `H!veAI\.git`, delete user work, or stage unrelated parent-root files.

Preserve user-owned untracked:

- `start-demo.bat`
- `task.md`

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/prompts/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_CLOSURE_PROMPT.md`
6. `H!veAI/docs/H!veAI/codex-logs/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_CLOSURE_LOG.md`
7. `H!veAI/docs/H!veAI/audits/M11A_REV4_FINAL_SINGLE_DASHBOARD_INTEGRATION_STRICT_REAUDIT.md`
8. current `.hiveai/PROJECT_DASHBOARD.md`
9. current `project_dashboard.rs`, `command_center.rs`, watcher source and focused tests
10. this prompt in full

Before production changes, synchronize prospective current-status truth only in:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`

They must say:

- M00-M10 PASS/CLOSED;
- 11/20 = 55%;
- M11 original historical FAIL;
- REV4 implementation complete but independent REV4 audit = FAIL with R19-R22 open;
- M11A REV5 = ACTIVE;
- M11 NOT CLOSED;
- M12 BLOCKED;
- user native/visual acceptance pending.

Do not rewrite historical prompts/logs/audits.

---

# Canonical UI Assets

Preserve exactly. REV5 is not a UI redesign.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve:

- repo sidebar logo `H!veAI/src/assets/hiveai-logo.png`;
- repo background `H!veAI/src/assets/hiveai-app-background.png`;
- repo opening video `H!veAI/src/assets/opening-video.mp4`;
- stable icon `H!veAI/dev-bin/H!veAI.ico`;
- current tracked Akilta wordmark used by the topbar attribution.

Required unchanged hashes:

- background: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve accepted behavior:

- no bottom footer band;
- Akilta attribution stays in topbar between Workspace/title and Search Workspace;
- exact visible credit stays `Built with ♥ for maximum productivity by Akilta`;
- whole attribution remains one clickable/focusable target;
- title remains `Developed by Akilta`;
- destination remains `https://www.akilta.com/`;
- native Chrome-only safe external open remains parameterless;
- no Edge fallback;
- no terminal/console flash;
- startup video remains audible and no same-process replay;
- Advanced source inventory remains available;
- no installer.

Do not modify any external registered project repository. Do not touch Bulk Edit.

---

# R19 / MAJOR - WAITING must not manufacture attention

## Defect

Current `materialized_operational_evidence()` treats `Project status = WAITING` itself as sufficient to create a `Project Dashboard status requires attention` row when no blocker/wait row already exists.

This violates the REV4 contract.

## Required production behavior

For valid/partial single-dashboard projects:

- `Project status = BLOCKED` may independently create an attention item.
- `Health = BLOCKED` or `Health = ATTENTION` may independently create an attention item.
- `Project status = WAITING` alone must **not** create attention.
- WAITING becomes actionable only when there is at least one meaningful real wait fact:
  - non-empty/non-UNKNOWN/non-NONE `Waiting on`; or
  - a meaningful item in `## Blockers and waiting`.
- `Required actor = HUMAN` or `EXTERNAL` may enrich an existing real wait but must not create attention alone.
- `Waiting on = NONE`, `UNKNOWN`, `NOT_VERIFIED`, empty, or `None verified` must remain non-actionable.

Do not weaken existing BLOCKED/ATTENTION behavior.

## Required tests

Directly prove:

1. WAITING + `Waiting on: NONE` + no blockers -> zero Project Dashboard attention items;
2. WAITING + `Waiting on: UNKNOWN` + no blockers -> zero Project Dashboard attention items;
3. WAITING + real `Waiting on` -> exactly one relevant wait attention item;
4. WAITING + duplicate blockers/waiting text + same `Waiting on` -> exactly one logical item;
5. BLOCKED with no detail still creates one truthful status attention item;
6. Health ATTENTION/BLOCKED remains actionable without inventing detail beyond declared state.

Tests must fail against REV4 behavior before the fix.

---

# R20 / MAJOR - Provenance-aware attention de-duplication

## Defect

Current `deduplicate_materialized_attention()` only compares free-text `detail` overlap. Persisted audit/test/permission rows use generic detail strings, while Project Dashboard quality/blocker text uses project-authored text. Equivalent evidence therefore survives as duplicate attention rows and inflates the global attention KPI.

## Required design

Create a deterministic normalized equivalence key for attention evidence.

The key must be bounded and provenance-aware. Prefer structured identity over prose.

Recommended dimensions, when available:

```text
project identity
+ task identity
+ evidence class
+ normalized source/check identity
```

Evidence classes should remain explicit, for example:

- WORKFLOW
- AGENT / PERMISSION
- TEST_RUN
- AUDIT
- PROJECT_DASHBOARD_QUALITY
- PROJECT_DASHBOARD_WAIT
- PROJECT_DASHBOARD_BLOCKER

Do not collapse unrelated evidence merely because both say `FAIL`, `blocked`, or `waiting`.

### Quality/verification matching

For materialized Quality/verification facts:

- normalize the check label deterministically;
- keep `.hiveai/PROJECT_DASHBOARD.md` provenance;
- when a persisted TEST_RUN/AUDIT can be tied to the same project/task/check identity, stronger persisted evidence suppresses the materialized duplicate;
- if identity cannot be proven, keep both rather than guessing equivalence.

If current persisted schema lacks a safe exact check identity, use only conservative evidence available in existing rows. Do not invent mappings.

### Blocker/wait matching

- project/task/source identity first;
- conservative normalized text only as a fallback;
- do not remove genuinely distinct blockers.

### KPI truth

`needs_attention` must be computed after de-duplication and reflect the final bounded set.

## Required direct tests

At minimum:

1. dashboard failed quality + matching persisted TEST_RUN -> one logical attention item;
2. dashboard failed quality + matching persisted AUDIT where identity is provable -> one logical attention item;
3. dashboard wait + matching stronger workflow/permission wait where identity is provable -> one logical item;
4. unrelated dashboard FAIL and persisted FAIL remain two distinct items;
5. de-duplication is deterministic across repeated snapshots;
6. final KPI count equals de-duplicated attention length.

Do not satisfy this by broad string matching.

---

# R21 / MINOR - Do not materialize Quality table headers as facts

The standard section may use:

```text
## Quality and verification
| Check | Result | Evidence |
| --- | --- | --- |
| Native tests | PASS | ... |
```

Current parser can materialize `Check: Result` as a factual quality row.

Required fix:

- detect and ignore the known table header row case-insensitively;
- preserve legitimate custom facts whose labels are not the header;
- retain the existing item/scalar bounds;
- do not change Source authorities parsing.

Required tests:

- standard `Check | Result | Evidence` header is absent from `quality_verification`;
- first real row is the first materialized fact;
- no fake Engineering Brief `Check: Result` fact appears.

---

# R22 / MINOR - Stable content/source identity for materialized operational rows

Current blocker and undated activity IDs depend on list index. Inserting an unrelated item earlier churns IDs of unchanged later facts.

Required fix:

- derive bounded deterministic IDs from project identity + materialized source class + stable normalized content/source identity;
- use a fixed-size deterministic digest if necessary;
- never expose unbounded raw content in IDs;
- preserve existing row ID preference for Current work when a real dashboard row ID exists;
- quality IDs must handle duplicate labels safely;
- repeated snapshots of unchanged dashboard content must produce identical IDs;
- inserting an unrelated preceding blocker/activity must not change the ID of an unchanged later item.

Do not introduce random UUIDs for materialized evidence identities.

Required tests:

1. unchanged snapshot -> identical IDs;
2. insert unrelated first blocker -> existing later blocker ID unchanged;
3. insert unrelated first activity -> existing later activity ID unchanged;
4. duplicate labels/content are handled deterministically without ID collision inside one project snapshot.

---

# R15 evidence follow-up - Real notify path, bounded only

Do not redesign R15.

The REV4 source-level scope reconciliation logic is accepted. Add the smallest practical production-path integration evidence for actual watcher delivery without directly calling `manager.sender` for the transition signal.

Where platform/runtime permits in tests:

- live manager starts legacy;
- physically create `.hiveai/PROJECT_DASHBOARD.md` on disk;
- wait boundedly for actual notify delivery and scope transition to SINGLE_DASHBOARD;
- physically remove/recreate or atomically replace the dashboard;
- prove the manager reconciles without restart;
- no busy-wait without deadline;
- if a platform limitation prevents deterministic directory-lifecycle proof, record that exact case as UNVERIFIED rather than fabricating PASS.

Do not broaden the watcher to make the test easier.

This is evidence follow-up only. Do not reopen R15 architecture unless the test exposes a real defect.

---

# Preserve REV4 closures

Do not regress:

- live scope comparison/recreation between LEGACY_RECURSIVE and SINGLE_DASHBOARD;
- exact single-dashboard event filtering;
- dashboard signal bounded M09 re-read;
- explicit rescan behavior;
- M10 stronger workflow truth;
- M10 queue duplicate suppression;
- front-matter/header accounting fix;
- enum validation;
- null/unknown task truth semantics;
- last-good M09 preservation on refresh failure;
- legacy ABSENT manifest informational semantics;
- malformed/stale/unavailable attention semantics;
- null audit/test actors unless proven;
- materialized Current work bounds/status mapping;
- materialized undated activity without fake timestamps;
- no materialized rows added to authoritative task totals;
- H!veAI single Project Dashboard contract.

---

# Testing and publication gates

Rust assertions must actually execute. `cargo test --no-run` is not acceptance.

Use the already-established shell-local Windows common-controls workaround only if required. Do not mutate Windows globally.

At minimum run and record exact results for:

1. focused R19 attention truth tests;
2. focused R20 de-duplication tests;
3. focused R21 parser/header test;
4. focused R22 identity-stability tests;
5. bounded actual notify-path R15 evidence test where platform permits;
6. full Rust native suite with assertions executed;
7. focused frontend Command Center/Task Sources/Akilta shell tests;
8. full frontend suite;
9. typecheck;
10. production frontend build;
11. dependency audit;
12. `cargo fmt --all -- --check`;
13. `cargo check`;
14. `git diff --check`;
15. canonical background/video SHA verification;
16. governed QA publication;
17. publisher failure harness.

Do not mark user native visual acceptance PASS.

---

# Builder log and final state

Create a new immutable builder log:

`H!veAI/docs/H!veAI/codex-logs/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_LOG.md`

The log must contain:

- starting HEAD;
- implementation commit SHA;
- exact files changed;
- R19-R22 implementation summary;
- exact focused/full test commands and results;
- actual notify-path evidence status;
- publication and failure-harness results;
- canonical asset hashes;
- proof that no external registered project was touched;
- final local HEAD;
- fetched `origin/H!veAI` HEAD;
- `git rev-list --left-right --count HEAD...origin/H!veAI` after all implementation/evidence commits, or explicitly explain why final post-log equality must be reported separately.

Final builder state must remain:

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M11 remains NOT CLOSED.
M12 remains BLOCKED.

Stop. Do not start M12.
