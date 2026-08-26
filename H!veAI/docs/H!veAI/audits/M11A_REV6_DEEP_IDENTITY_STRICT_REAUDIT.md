# M11A REV6 Deep Identity Strict Re-Audit

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_MICRO_CLOSURE_LOG.md`
Audited implementation commit: `a1d3812096fd11881919cf90d231cdd9580f44fc`
Builder-log commit: `08c598504c20a13f9fb3e5bbba01061221dd53ec`
Prior REV6 source audit: `H!veAI/docs/H!veAI/audits/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_STRICT_REAUDIT.md`
Remote `H!veAI` at this audit start: `0303a50853715a4530d03a97ca64b96bb233bf8d`
Authoritative builder prompt: `H!veAI/docs/H!veAI/prompts/M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_MICRO_CLOSURE_PROMPT.md`

## Verdict

**FAIL / M11 NOT CLOSED**

- BLOCKER: 0
- MAJOR: 2
- MINOR: 0
- NOTE: 2
- Confidence: HIGH
- Regression risk: MEDIUM-HIGH because both findings affect operational evidence identity and can silently suppress or merge distinct Project Dashboard facts.

This deeper re-audit supersedes the prior REV6 source-level PASS decision for closure purposes. The prior audit correctly verified removal of the 256-character prefix truncation, but it did not challenge the remaining normalization and display-string reconstruction semantics with non-ASCII and delimiter-bearing evidence.

REV6 fixes the narrow R23 defect from REV5: long ASCII facts that differ after character 256 now remain distinguishable. However, the operational identity path is still lossy in two independent ways:

1. `normalize_attention_source()` discards every non-ASCII character because it tokenizes with `is_ascii_alphanumeric()`.
2. `attention_identity()` reconstructs Quality identity by parsing the human display string `AttentionItem.detail` with `split_once(':')`, which loses label content after the first colon.

Both violate the M11 truthfulness requirement that identity/deduplication may normalize presentation differences but must not erase semantically distinguishing bounded evidence before equivalence is proven.

Builder-reported test and Windows publication results remain builder evidence. This audit independently inspected the pushed implementation, parser bounds, REV6 direct tests, branch lineage, and the immutable builder log.

---

# Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| REV5 256-character prefix truncation | PASS | REV6 removed the explicit 256-character clip. |
| Full bounded ASCII identity | PASS for exercised cases | Long ASCII blockers/activity/Quality checks covered by REV6 tests remain distinct. |
| Unicode identity preservation | **FAIL / R24 MAJOR** | All non-ASCII letters/symbols are treated as separators and discarded by `is_ascii_alphanumeric()`. Distinct valid UTF-8 materialized facts can normalize to the same identity. |
| Structured Quality equivalence | **FAIL / R25 MAJOR** | Quality source identity is reconstructed from display text with `split_once(':')`; labels containing `:` are truncated for dedup identity. |
| Fixed-size emitted IDs | PASS shape / not sufficient for identity safety | SHA-derived IDs remain bounded, but hashing a lossy normalized input still permits semantic collapse before hashing. |
| R19 WAITING truth | PASS / preserved | No reopening found. |
| R20 conservative dedup principle | **PARTIAL / reopened by R24/R25** | The policy is conservative, but the source identity fed into that policy can be lossy. |
| R21 Quality header filtering | PASS / preserved | No reopening found. |
| R15 watcher architecture | PASS source-level / preserved | No new watcher defect found. |
| User native/visual acceptance | PENDING | Still user-owned and cannot close M11 while production findings remain open. |

---

# R24 / MAJOR - Non-ASCII bounded evidence is erased before identity hashing/equality

## Evidence

Current production code uses:

```rust
fn normalize_attention_source(value: &str) -> String {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}
```

This does not merely normalize punctuation/whitespace/case. It removes every character outside ASCII alphanumeric.

The Project Dashboard parser stores ordinary UTF-8 `String` materialized values and bounds them by byte length; it does not restrict materialized facts to ASCII. Therefore valid bounded evidence such as Turkish names, accented words, CJK text, Greek/Cyrillic text, emoji-bearing identifiers, or mixed-language check names can reach the identity layer.

Examples of distinct valid facts that can collapse or lose distinguishing portions include:

```text
"ş blocker"
"ç blocker"
```

Both normalize to `blocker`.

Or:

```text
"build türkiye"
"build çin"
```

which can reduce distinguishing non-ASCII tokens or characters and create false equality depending on the remaining ASCII content.

For blockers, `blocker_keys` uses the normalized value directly. A false equality can silently discard the second real blocker before it is emitted.

The same lossy normalization also feeds:

- WAITING identity;
- Quality occurrence keys and IDs;
- generated Current Work IDs;
- undated materialized activity identity;
- `AttentionIdentity.source` for stronger-evidence suppression.

Thus the issue is systemic, not limited to one ID output.

## Why this is MAJOR

The core M11 contract is operational truthfulness. A real bounded Project Dashboard fact must not disappear because its distinguishing content is non-ASCII. Silent suppression is worse than a display defect because the user can be told there is less attention required than the project actually declares.

## Required closure

Replace ASCII-only tokenization with a Unicode-preserving deterministic identity normalization.

The closure must satisfy all of the following:

- retain the complete already-bounded UTF-8 scalar;
- normalize whitespace deterministically;
- normalize case only in a way that does not discard non-ASCII content;
- normalize punctuation only according to an explicitly documented equivalence rule;
- never drop Unicode letters/numbers merely because they are non-ASCII;
- distinct bounded evidence whose semantic content differs only in non-ASCII characters must remain distinct;
- final public IDs may remain fixed-size SHA-derived values;
- do not introduce random/UUID IDs;
- preserve identical-input determinism.

A safe minimum is to preserve Unicode characters verbatim except for deterministic whitespace folding and a clearly bounded case-normalization strategy. Do not add an unreviewed transliteration layer that maps distinct scripts/characters onto the same ASCII representation.

## Required direct tests

Add tests that fail against REV6 and pass only after the production fix:

1. Two blockers with identical ASCII context but distinct Turkish/non-ASCII content both survive.
2. Their emitted IDs are distinct and fixed-size.
3. An identical Unicode blocker still collapses deterministically.
4. Two undated activity facts differing only by non-ASCII content receive distinct stable IDs.
5. A Project Dashboard Quality check and persisted TEST_RUN/AUDIT whose sources differ only in non-ASCII content do **not** deduplicate.
6. A true exact normalized Unicode match still deduplicates under the accepted structured conditions.
7. Repeated snapshots and unrelated preceding insertion do not change prior Unicode-derived IDs.
8. `needs_attention` equals final post-dedup attention length.

Use bounded fixtures within the existing materialized scalar limits.

---

# R25 / MAJOR - Quality equivalence is reconstructed from human display text and truncates colon-bearing labels

## Evidence

Materialized Quality attention is emitted with a display detail like:

```rust
detail: format!("{}: {}", fact.label, fact.value)
```

Later, `attention_identity()` attempts to recover the Quality source with:

```rust
item.detail
    .split_once(':')
    .map(|(label, _)| label)
    .unwrap_or(&item.detail)
```

This is not a reversible structured identity path.

A valid Quality label can itself contain a colon. For example:

```text
Check label: "build: windows"
Result: "FAIL"
```

The emitted display detail becomes:

```text
build: windows: FAIL
```

but `attention_identity()` recovers only:

```text
build
```

A persisted TEST_RUN/AUDIT source `build` for the same project/task can then appear equal and suppress the materially different dashboard Quality fact `build: windows`.

The same class of defect exists whenever operational identity is reconstructed from presentation-formatted strings rather than preserved as structured evidence.

## Why this is MAJOR

This can silently suppress a real failed check in `Needs Your Attention` despite the accepted R20 rule requiring proven source/check identity. Equality is not proven here; it is manufactured by a lossy display parser.

## Required closure

Do not derive operational equivalence identity from user-facing `detail` strings.

Introduce or preserve an explicit structured identity source for attention evidence. The implementation may use an internal non-serialized field, a dedicated internal identity structure, or another bounded approach that keeps the API stable, but the following must hold:

- Project Dashboard Quality uses the exact bounded `fact.label` (after the approved identity normalization), not a parsed display string.
- TEST_RUN uses the actual persisted `command` source identity.
- AUDIT uses the actual persisted `summary` source identity under the existing accepted semantics.
- WAITING/BLOCKER/PERMISSION/WORKFLOW matching must likewise avoid lossy display parsing where an original structured source exists.
- Human-readable `detail` remains presentation only and must not be the authority for operational equivalence.
- Existing conservative project + task + evidence-class requirements remain mandatory.
- If exact structured equivalence cannot be proven, keep both items.

Do not broaden deduplication while fixing this. The desired failure mode is duplicate visible evidence, not silent removal.

## Required direct tests

At minimum:

1. Dashboard Quality `build: windows` and persisted TEST_RUN `build` for the same task remain distinct.
2. Dashboard Quality `build: windows` and persisted AUDIT `build` remain distinct.
3. A true persisted source `build: windows` suppresses the weaker dashboard duplicate under the existing accepted rules.
4. Labels containing multiple colons and punctuation remain identity-stable.
5. Unicode + colon combined cases remain distinct when not exact matches.
6. Display formatting changes must not change operational identity if the underlying structured source is unchanged.

---

# Evidence notes

## E16 / NOTE - REV6 direct tests were too ASCII-specific

The REV6 tests correctly exercise the former 256-character clipping defect but build their long prefixes from ASCII-only strings (`"x"` and repeated `"quality"`). They therefore cannot detect R24. They also use Quality labels without colons, so they cannot detect R25.

The next closure must include adversarial tests for supported input classes, not only the exact previous reproducer.

## E17 / NOTE - Prior REV6 PASS audit is historical evidence, not current closure authority

`M11A_REV6_FULL_SCALAR_IDENTITY_FINAL_STRICT_REAUDIT.md` remains immutable historical audit evidence. Do not rewrite/delete it. This deeper audit supersedes its closure verdict because it identifies two concrete production defects in the same pushed implementation.

---

# Confirmed REV6 closures to preserve

Do not reopen these unless the next fix exposes a concrete regression:

- explicit 256-character identity clipping is removed;
- long ASCII facts differing after character 256 remain distinguishable;
- fixed-size SHA-derived materialized IDs remain bounded and contain no raw long source content;
- identical blocker collapse remains deterministic for the exercised ASCII path;
- repeated snapshot IDs remain stable for the exercised ASCII path;
- R19 WAITING truth remains corrected;
- Quality table headers remain excluded;
- M10 workflow truth remains stronger than materialized dashboard evidence;
- unknown values remain unknown;
- user-owned timestamps are not fabricated;
- R15 single-dashboard watcher architecture and actual notify-path evidence remain preserved;
- Akilta topbar attribution, footer removal/reclaimed workspace, startup video/audio/replay, terminal suppression, Advanced source inventory, and canonical shell behavior are not reopened by this audit;
- no external registered project repository or Bulk Edit modification is required.

---

# Required next action

Do not start M12.

Run one bounded **M11A REV7 Unicode + Structured Identity closure** addressing R24 and R25 only, plus full regression evidence.

M11 remains NOT CLOSED until:

1. R24 and R25 pass independent re-audit with no BLOCKER/MAJOR production defect;
2. required regression/publication gates are green;
3. user native/visual acceptance is complete.

The next Codex prompt is:

`H!veAI/docs/H!veAI/prompts/M11A_REV7_UNICODE_AND_STRUCTURED_IDENTITY_FINAL_CLOSURE_PROMPT.md`
