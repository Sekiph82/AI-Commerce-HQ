# M11A REV5 Final Attention Truth and Identity Strict Re-Audit

Date: 2026-08-26
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_LOG.md`
Audited implementation commit: actual remote commit `5162341fc6b10b1ab07e8087e1cfc35d7a1f2aac`
Builder-log commit / remote HEAD at audit start: `fb2fda1b82b6c0f7cf178670187fb58c7403c061`
Authoritative builder prompt: `H!veAI/docs/H!veAI/prompts/M11A_REV5_FINAL_ATTENTION_TRUTH_AND_IDENTITY_MICRO_CLOSURE_PROMPT.md`

## Verdict

**FAIL / M11 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 1
- MINOR: 1
- NOTE: 2
- Confidence: HIGH
- Regression risk: MEDIUM because the remaining production defect is isolated to materialized evidence identity, but it can silently hide or churn valid Project Dashboard evidence.

REV5 closes the substantive R19 WAITING false-positive defect, the basic provenance-aware R20 suppression cases, the R21 Quality header defect, and the previously noted actual-notify evidence gap for the tested create/delete path. However, the R22 identity implementation is not collision-safe over the full bounded materialized scalar. Distinct valid evidence with a common first 256 normalized characters can be treated as the same evidence before SHA-256 is applied. That can silently drop a real blocker or churn IDs after unrelated edits.

Builder-reported Windows test/publication totals remain builder evidence. This audit independently inspected the pushed implementation, current branch lineage, focused tests, parser/watcher/Command Center source, and the immutable builder log.

## Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| R19 WAITING truth | PASS | WAITING alone no longer creates status attention; BLOCKED and Health ATTENTION/BLOCKED remain independently actionable. |
| R20 provenance-aware attention de-dup | PASS for exercised structured cases | Matching project + task + normalized check/source suppresses weaker dashboard TEST/AUDIT/PERMISSION duplicates; unrelated failures remain distinct. |
| R21 Quality header filtering | PASS | Standard `Check | Result | Evidence` and `Check | Result` header shapes are excluded. |
| R22 stable materialized identity | **PARTIAL / FAIL** | SHA-256 output is fixed-size, but the identity input has already been truncated to 256 normalized characters. Distinct bounded facts can collide before hashing. |
| R15 actual notify evidence | PASS for legacy -> single -> legacy delete path / NOTE | The new test uses actual notify delivery instead of direct sender injection and passed per builder log. Recreate/atomic-replace after deletion is not directly exercised by this new test. |
| REV3/REV4 UI and single-dashboard architecture | PASS source-level / user native visual acceptance pending | No reviewed REV5 diff reopens the topbar Akilta attribution, footer removal, single-dashboard watcher design, or Advanced source inventory separation. |

---

# Open finding

## R23 / MAJOR - Identity normalization truncates before hashing, so distinct valid materialized evidence can collapse

`stable_materialized_id()` itself hashes its identity input and emits a fixed-size digest, which is the correct output shape. The defect is upstream: `normalize_attention_source()` returns only the first 256 normalized characters.

That truncated value is used as the source identity for blockers, waits, Quality checks, materialized activity and attention equivalence. It is also used directly in `blocker_keys` before IDs are generated.

The Project Dashboard parser allows bounded materialized values substantially longer than 256 characters. Therefore two distinct valid facts such as:

```text
<same normalized first 256 characters> + " dependency A"
<same normalized first 256 characters> + " dependency B"
```

can produce the same normalized identity.

For blockers the effect is worse than an ID collision: `blocker_keys.insert(blocker_key)` treats the second fact as a duplicate and drops it before it reaches `Needs Your Attention`.

For activity/Quality/work generated identity the same prefix collision can produce unstable occurrence-based identities or duplicate IDs when otherwise distinct source facts are inserted/reordered.

For R20 de-duplication, a truncated source identity can also make two unrelated long sources appear equal, allowing a stronger persisted row to suppress a distinct Project Dashboard item when task/evidence class also happen to match.

This violates the central M11 truthfulness contract: bounded storage/output is required, but bounded output must not be achieved by discarding distinguishing input before identity hashing.

### Required closure

- Separate display/search normalization from identity normalization.
- Identity hashing/equality must consume the full **already bounded** materialized scalar/source identity, not the first 256 characters.
- It is acceptable and preferred for the final public ID to remain a fixed-size SHA-256-derived digest.
- Do not place raw long content into IDs.
- For attention equivalence, use either the full bounded normalized source or a fixed-size digest of the full bounded normalized source.
- `blocker_keys` must not collapse two distinct facts that differ after character 256.
- Current work generated IDs, Quality IDs and activity IDs must remain stable under unrelated preceding-row insertion.
- Preserve deterministic duplicate handling for genuinely identical facts.

### Required direct tests

Add failing-before-fix tests that prove:

1. two blocker facts with identical normalized first 256 characters but different later content both survive as two distinct attention items;
2. those two blockers receive distinct fixed-size IDs;
3. inserting an unrelated earlier blocker does not change either existing ID;
4. two long undated activity facts with the same first 256 normalized characters receive distinct stable IDs;
5. long Quality/check identities do not falsely de-duplicate against a different persisted check that shares only the first 256 normalized characters;
6. a truly identical long Project Dashboard fact remains deterministically de-duplicated;
7. no raw unbounded content appears in emitted IDs.

---

# Evidence finding

## E11 / MINOR - Builder log records full SHAs that do not match the pushed commit graph

The REV5 builder log records:

- starting full SHA `83ee210d6ed7bff8e6f7fcd802fb76561609cf8b`;
- implementation full SHA `5162341d4a343c11ad9f57d9493f3aa9aa8fb1df`.

The actual pushed graph is:

- authoritative REV5 prompt/start commit `83ee21039392148de686fa8e09396e2ec4f626c6`;
- implementation commit `5162341fc6b10b1ab07e8087e1cfc35d7a1f2aac`;
- immutable log commit `fb2fda1b82b6c0f7cf178670187fb58c7403c061`.

The short prefixes happen to match, but the persisted full-SHA/equality claims in the builder log are not valid GitHub object identities. This is an evidence-bookkeeping defect, not a production-code defect, and does not by itself reopen the completed functional corrections.

### Required closure evidence in next builder log

- Read SHA values from Git commands; do not synthesize/expand a short SHA.
- Persist exact `git rev-parse HEAD`, exact `git rev-parse origin/H!veAI`, and `git rev-list --left-right --count HEAD...origin/H!veAI` after the implementation commit is pushed/fetched.
- After the immutable log is pushed, report the exact post-log local/origin equality in the final builder response. Do not rewrite the historical REV5 log.

---

# Evidence notes

## E12 / NOTE - Actual notify follow-up does not exercise post-delete recreate/atomic replacement

The added `actual_notify_path_reconciles_dashboard_scope_without_restart` test physically creates the dashboard, proves transition to SINGLE_DASHBOARD, physically removes it, and proves fallback to LEGACY_RECURSIVE. This is materially better evidence than direct queue injection. It does not physically recreate the dashboard after deletion or perform an atomic replacement in the same test. Source-level R15 scope reconciliation remains accepted; keep this as release-hardening evidence rather than reopening architecture unless a real native failure appears.

## E13 / NOTE - Native visual acceptance remains user-owned

REV5 did not materially redesign the shell. The current topbar/footer/Command Center geometry still needs final acceptance from the published native build before M11 is closed, as required by the milestone contract.

---

# Confirmed REV5 closures

Do not reopen these unless the R23 fix exposes a concrete regression:

- WAITING without a real wait fact does not manufacture Project Dashboard attention.
- BLOCKED and Health ATTENTION/BLOCKED remain actionable.
- Matching persisted TEST_RUN/AUDIT/permission evidence can suppress the exercised weaker materialized duplicate using structured project/task/source identity.
- Unproven unrelated failures remain separate.
- `needs_attention` is calculated after the current de-duplication stage.
- Quality table headers are not materialized as Quality facts.
- Fixed-size SHA-derived materialized IDs are used rather than raw content or random UUIDs.
- Actual notify delivery is exercised for a live legacy -> single -> legacy dashboard create/delete transition.
- R15-R18 and the REV3 shell/Task Sources corrections remain preserved.

# Required next action

Do not start M12.

Run one final bounded M11A REV6 identity-input micro-closure for R23 plus the E11 evidence discipline. Do not redesign the Command Center or watcher. M11 may close only after independent REV6 re-audit finds no BLOCKER/MAJOR production defect and the user completes native visual acceptance of the currently published shell.
