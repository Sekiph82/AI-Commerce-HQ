# M11A REV6 - Full-Scalar Identity Final Micro-Closure

## Authority

This is the single authoritative Codex prompt for the next H!veAI run.

It is a bounded continuation of M11A and exists only to close the remaining production finding R23 plus evidence discipline E11 from:

`H!veAI/docs/H!veAI/audits/M11A_REV5_FINAL_ATTENTION_TRUTH_IDENTITY_STRICT_REAUDIT.md`

Do not split this into M11B/M11C or a new numbered milestone.
Do not start M12.

Strict completed roadmap count remains **11 / 20 = 55%** until independent M11 closure.

Preserve every REV3/REV4/REV5 closure not explicitly reopened below.

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

Preserve user-owned untracked:

- `start-demo.bat`
- `task.md`

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/prompts/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_PROMPT.md`
6. `H!veAI/docs/H!veAI/codex-logs/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_LOG.md`
7. `H!veAI/docs/H!veAI/audits/M11A_REV5_FINAL_ATTENTION_TRUTH_IDENTITY_STRICT_REAUDIT.md`
8. current `.hiveai/PROJECT_DASHBOARD.md`
9. current `command_center.rs`, `project_dashboard.rs`, watcher source, and focused tests
10. this prompt in full

Before production edits, synchronize prospective current-status truth only in:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`

They must say:

- M00-M10 PASS/CLOSED;
- strict completed 11/20 = 55%;
- original M11 historical FAIL;
- REV5 implementation complete but independent REV5 strict audit = FAIL with R23 open and E11 evidence defect;
- M11A REV6 = ACTIVE;
- M11 NOT CLOSED;
- M12 BLOCKED;
- user native/visual acceptance pending.

Historical prompts/logs/audits are immutable. Do not edit the REV5 builder log to repair its incorrect full SHA strings.

---

# Canonical UI Assets

This section is mandatory and authoritative for regression protection.

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Preserve without regeneration/substitution:

- sidebar logo: `H!veAI/src/assets/hiveai-logo.png`;
- background: `H!veAI/src/assets/hiveai-app-background.png`;
- opening video: `H!veAI/src/assets/opening-video.mp4`;
- stable shortcut icon: `H!veAI/dev-bin/H!veAI.ico`;
- current tracked Akilta wordmark used by the topbar attribution.

Required unchanged hashes:

- background SHA-256: `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`
- opening video SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Preserve accepted behavior:

- no bottom footer band;
- Akilta attribution remains in the topbar between Workspace/title and Search Workspace;
- exact visible credit remains `Built with ♥ for maximum productivity by Akilta`;
- whole attribution remains one clickable/focusable target;
- title remains `Developed by Akilta`;
- destination remains `https://www.akilta.com/`;
- native Chrome-only safe external open remains parameterless;
- no Edge fallback;
- no terminal/console flash;
- startup video remains audible and no same-process replay;
- Advanced source inventory remains available;
- current Command Center one-screen layout remains unchanged;
- no installer.

REV6 is not a UI redesign. Do not touch UI unless a test-only selector needs no production change.

Do not modify any external registered project repository. Do not touch Bulk Edit.

---

# R23 / MAJOR - Hash the full bounded identity, not a 256-character prefix

## Defect

REV5 correctly moved materialized IDs to fixed-size SHA-256-derived output, but `normalize_attention_source()` truncates normalized input to 256 characters before that value is used for:

- `blocker_keys` duplicate detection;
- materialized blocker IDs;
- waiting IDs;
- Quality identity and occurrence keys;
- generated Current work identity;
- undated materialized activity identity;
- `AttentionIdentity.source` used for stronger-evidence matching.

The Project Dashboard parser allows bounded materialized scalar values longer than 256 characters. Two distinct valid facts can therefore share the same first 256 normalized characters and become indistinguishable before SHA-256 is applied.

This can silently drop a real blocker, create unstable occurrence-based IDs, or falsely suppress an unrelated materialized attention item against stronger persisted evidence.

## Required design

Separate human/display normalization from identity normalization.

### Identity normalization

Create a dedicated identity-normalization/hash path that consumes the complete **already bounded** materialized source scalar.

Requirements:

- normalize deterministically for case/whitespace/punctuation according to the existing matching semantics;
- do **not** truncate to 256 characters before hashing/equality;
- rely on the existing parser/scalar bounds for input boundedness, or apply a clearly documented bounded maximum that is at least the full supported materialized scalar size;
- use a fixed-size SHA-256-derived digest for stored/emitted identity keys where appropriate;
- never emit raw long materialized text inside IDs;
- deterministic identical input must always produce identical identity;
- distinct bounded input that differs after character 256 must remain distinct.

A recommended structure is:

```text
full bounded scalar
  -> deterministic full normalization
  -> SHA-256 fixed-size identity digest
```

Display/search strings may still use their own smaller clipping if needed, but those clipped strings must never decide operational evidence identity.

### Blocker de-duplication

`blocker_keys` must use the full normalized identity or its digest.

Two blockers with a common 256-character prefix but different suffixes must both survive.

Truly identical blockers must still collapse deterministically to one logical item.

### Attention equivalence

`AttentionIdentity.source` must not be a truncated prefix.

Use the full bounded normalized source or a digest of it for equality.

Preserve REV5 conservative rules:

- quality materialized evidence only matches stronger TEST_RUN/AUDIT when project + proven task identity + normalized check identity match;
- wait/blocker materialized evidence only matches stronger WORKFLOW/PERMISSION under the accepted structured conditions;
- unrelated FAIL/BLOCKED/WAITING evidence must remain distinct;
- if equivalence is not proven, keep both.

### Materialized ID stability

Apply the collision-safe full identity to:

- blocker IDs;
- waiting IDs;
- status IDs where relevant;
- Quality IDs and duplicate occurrences;
- generated Current work IDs when dashboard row ID is absent;
- undated materialized activity IDs.

Preserve existing real Current work row IDs when supplied by the dashboard.

Do not use UUID/random IDs.

---

# Required direct tests

Add tests that **fail against REV5** and pass only after the production identity fix.

At minimum:

1. Create two meaningful blockers whose normalized text is identical for the first 256 characters and differs only after that point. Both must appear in Project Dashboard attention.
2. Those blockers must have two distinct fixed-size IDs.
3. Insert an unrelated blocker before them; both previous IDs must remain unchanged.
4. Repeat with two undated materialized activity facts sharing the first 256 normalized characters. They must have distinct stable IDs.
5. Create a long dashboard Quality/check identity and a persisted TEST_RUN/AUDIT check that shares only the first 256 normalized characters but differs later. They must **not** de-duplicate.
6. Create a true full-string match for the same long Quality/check and prove stronger persisted evidence still suppresses the weaker materialized duplicate.
7. A truly identical long blocker remains deterministically collapsed to one logical blocker.
8. Repeated snapshots produce exactly the same materialized IDs.
9. No emitted materialized ID includes raw long source content.
10. `needs_attention` equals the final post-de-duplication attention length.

Keep tests bounded. Do not create multi-megabyte fixtures; use values within the existing manifest/scalar limits.

---

# E11 - Exact Git SHA evidence discipline

The immutable REV5 builder log contains incorrect expanded full SHA strings. Do not rewrite it.

REV6 evidence must use exact Git command output.

Before implementation commit:

```powershell
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

After implementation commit is pushed and fetched:

```powershell
git fetch origin H!veAI
git rev-parse HEAD
git rev-parse origin/H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Persist those exact values in the new builder log. Never expand a short SHA by inference or generated text.

After the immutable builder log itself is committed/pushed, perform the same three commands again and report the exact final post-log equality in the final Codex response. Historical log remains immutable.

---

# R15 bounded evidence preservation

Do not redesign the watcher.

Keep REV5's actual-notify integration test and the previously accepted live scope logic.

If practical without broadening scope, extend the actual-notify test to physically recreate the dashboard after deletion or perform an atomic replacement and prove SINGLE_DASHBOARD recovery. This is desirable release-hardening evidence, but do not make unrelated watcher architecture changes merely for this extension.

If the platform makes that exact event nondeterministic, record the exact case as UNVERIFIED. Never fabricate PASS.

---

# Preserve all prior closures

Do not regress:

- R19 WAITING truth;
- R20 structured/provenance-aware de-dup rules;
- R21 Quality header filtering;
- fixed-size materialized IDs;
- M10 stronger workflow truth;
- M10 queue duplicate suppression;
- R15 live watcher scope reconciliation;
- exact single-dashboard event filtering;
- dashboard signal bounded M09 refresh;
- R17 header/front-matter accounting;
- R18 materialized enum validation;
- unknown task truth stays unknown, not fake zero;
- last-good M09 survives refresh failure;
- legacy ABSENT dashboard remains informational;
- malformed/stale/unavailable actionable manifest truth remains visible;
- audit/test actors remain null unless persisted evidence proves them;
- materialized current-work rows do not alter authoritative task totals;
- materialized recent activity remains undated unless a real timestamp exists;
- H!veAI keeps a single `.hiveai/PROJECT_DASHBOARD.md` live contract;
- external project source inventories stay Advanced/internal evidence.

---

# Testing and verification gates

Rust assertions must actually execute. `cargo test --no-run` is not acceptance.

Use the established shell-local Windows common-controls workaround only if required. Do not change Windows globally.

At minimum execute and record exact results for:

1. focused new R23 long-common-prefix collision tests;
2. focused existing R19-R22 tests;
3. focused Project Dashboard parser tests;
4. focused watcher/single-dashboard tests, including REV5 actual-notify test;
5. full Rust native suite with assertions executed;
6. focused frontend Command Center/Task Sources/Akilta shell tests;
7. full frontend suite;
8. TypeScript typecheck;
9. production frontend build;
10. dependency audit at high severity;
11. `cargo fmt --all -- --check`;
12. `cargo check`;
13. `git diff --check`;
14. canonical background/video SHA verification;
15. X01 terminal suppression regression;
16. X02 startup audio/replay regression;
17. governed QA publication;
18. publisher failure harness.

Do not mark user native/visual acceptance PASS yourself.

---

# Builder log and closure state

Create exactly one new immutable builder log:

`H!veAI/docs/H!veAI/codex-logs/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_MICRO_CLOSURE_LOG.md`

The log must contain:

- exact starting full HEAD from `git rev-parse HEAD`;
- exact starting `origin/H!veAI` SHA;
- exact starting left/right count;
- implementation commit full SHA read from Git;
- exact files changed;
- R23 implementation summary;
- direct long-common-prefix regression evidence;
- preserved R19-R22/R15 evidence;
- exact focused/full test commands and results;
- publication/failure-harness results;
- canonical asset hashes;
- proof no external registered project or Bulk Edit was touched;
- exact post-implementation local HEAD;
- exact fetched `origin/H!veAI` HEAD;
- exact post-implementation left/right count.

After the log is committed and pushed, run the final SHA/equality commands again and report them in the final response without rewriting the log.

Final builder state must remain:

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

M11 remains NOT CLOSED.
M12 remains BLOCKED.

Stop. Do not start M12.
