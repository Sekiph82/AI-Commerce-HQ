# M11A REV7 Unicode + Structured Identity Final Strict Re-Audit

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Audited builder log: `H!veAI/docs/H!veAI/codex-logs/M11A_REV7_UNICODE_AND_STRUCTURED_IDENTITY_FINAL_CLOSURE_LOG.md`
Audited implementation commit: `4a7e6adc29d53ae5f30b321229ecd2b25ec97cfa`
Authoritative REV7 prompt: `H!veAI/docs/H!veAI/prompts/M11A_REV7_UNICODE_AND_STRUCTURED_IDENTITY_FINAL_CLOSURE_PROMPT.md`

## Verdict

**SOURCE-LEVEL PASS / R24 PASS / R25 PASS / M11 PENDING FINAL USER NATIVE-VISUAL ACCEPTANCE**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0 production defects
- NOTE: 2
- Confidence: HIGH
- Regression risk: LOW-MEDIUM

REV7 closes both production findings opened by the deep REV6 audit. No further identity-remediation coding run is justified by the reviewed source.

R24 is closed because operational identity normalization no longer tokenizes through ASCII-only alphanumeric boundaries. The implementation consumes the complete parser-bounded UTF-8 scalar, folds whitespace, applies Unicode-preserving lowercase conversion, preserves punctuation and non-ASCII distinguishing content, and then uses fixed-size SHA-derived public IDs where required.

R25 is closed because operational equivalence is now carried in a private, non-serialized `AttentionIdentity` assembled directly from structured evidence sources. Human-readable `title`/`detail` strings are no longer parsed back into identity authority.

The conservative failure mode remains correct: where structured equivalence cannot be proven, evidence remains visible rather than being silently suppressed.

## Acceptance matrix

| Area | Result | Independent finding |
| --- | --- | --- |
| R24 non-ASCII preservation | PASS | `normalize_operational_identity()` preserves bounded Unicode and punctuation instead of erasing non-ASCII characters. |
| R24 full bounded input | PASS | Identity normalization operates on the complete parser-bounded scalar; no 256-character prefix clipping reappears. |
| R24 fixed-size IDs | PASS | `stable_materialized_id()` still SHA-256-hashes identity material and emits bounded digest-derived IDs. |
| R24 blocker identity | PASS | `blocker_keys` uses the Unicode-preserving normalized value, so Turkish/other non-ASCII distinctions survive duplicate detection. |
| R24 activity/current-work identity | PASS | Materialized activity and generated Current Work identity use the same operational normalization path. |
| R25 Quality structured identity | PASS | Project Dashboard Quality identity comes from the original `MaterializedFact.label`, not from `detail.split_once(':')`. |
| R25 TEST_RUN identity | PASS | Stronger TEST_RUN identity is built directly from persisted `command`. |
| R25 AUDIT identity | PASS | Stronger AUDIT identity is built directly from persisted `summary`. |
| R25 WAITING/BLOCKER identity | PASS | Original materialized wait/blocker scalars are carried directly into structured identity. |
| R25 PERMISSION identity | PASS | Persisted permission kind is carried directly into structured identity. |
| R25 WORKFLOW identity | PASS | Workflow structured source is attached during workflow attention construction instead of being reconstructed from presentation later. |
| Conservative project/task/class matching | PASS | Existing project equality, proven task matching and explicit stronger/weaker evidence-class rules remain in `attention_identities_match()`. |
| Direct adversarial tests | PASS source-present / builder execution evidence | REV7 adds Unicode blocker/activity/current-work coverage and colon-bearing structured Quality coverage; builder reports all focused and full suites passed. |
| Prior R19-R23 closures | PASS / preserved | Reviewed REV7 diff does not reopen the previously closed WAITING, provenance, Quality-header, full-scalar or stable-ID findings. |
| M12 / M21 scope protection | PASS | Neither was started by the REV7 implementation. |

## Independent source findings

### R24

`normalize_operational_identity()` now performs:

```text
full bounded UTF-8 scalar
  -> whitespace folding
  -> Unicode-preserving lowercase
```

It does not transliterate into ASCII and it does not strip punctuation. This is intentionally conservative. Standard-library lowercase is not a complete linguistic/locale-aware Unicode case-folding engine, but that does not constitute a truthfulness defect here: the safe consequence for difficult case-equivalence pairs is failure to deduplicate, not silent merging of distinct evidence. The prompt explicitly requires the safe failure mode to preserve evidence when equivalence is not provable.

### R25

`AttentionItem` now contains a private `#[serde(skip)] operational_identity`. `attention_identity()` simply returns this typed identity instead of reconstructing it from rendered detail. TEST_RUN, AUDIT, PERMISSION, WORKFLOW and Project Dashboard materialized attention rows attach their identity at source assembly time.

This removes the previous delimiter bug where a Quality label such as `build: windows` could be reconstructed as only `build` and incorrectly suppressed against stronger persisted evidence.

## Regression evidence inspected

The pushed source contains direct adversarial tests covering:

- distinct Turkish/non-ASCII blockers surviving as separate facts;
- identical Unicode blockers deduplicating deterministically;
- long bounded Unicode materialized evidence;
- Unicode-generated Current Work IDs;
- Unicode undated activity IDs;
- repeated snapshot stability;
- stability after unrelated preceding-row insertion;
- colon-bearing Quality labels remaining distinct from shorter TEST_RUN/AUDIT sources;
- exact structured Quality matches suppressing only the weaker Project Dashboard duplicate;
- post-dedup `needs_attention` equality.

The builder log reports 278 Rust tests and 87 frontend tests passed, plus typecheck, production frontend build, dependency audit, Rust formatting/checking, publisher failure harness, governed QA publication and native technical probes. Those execution results remain builder evidence rather than independently rerun by this GitHub source audit.

## Git/evidence review

The builder log identifies implementation commit:

`4a7e6adc29d53ae5f30b321229ecd2b25ec97cfa`

The actual GitHub commit exists with message `Close M11A REV7 identity findings` and contains the expected R24/R25 source changes plus canonical status-document updates.

No evidence-bookkeeping defect comparable to REV5 E11 was found in the REV7 log.

## Notes

### NOTE R7-01 - historical opening-video preservation text is now stale product documentation

The REV7 prompt/log still mention `src/assets/opening-video.mp4` as a preserved canonical asset because REV7 was authored under the older asset contract. A separate bounded corrective task subsequently established `src/assets/H!veAI.mp4` as the actual startup asset and fixed native playback/icon behavior. This is historical documentation context, not an R24/R25 production defect. Future prompts must treat `H!veAI.mp4` as the current startup asset and must not instruct Codex to switch production playback back to `opening-video.mp4`.

### NOTE R7-02 - final M11 closure remains user-owned at the native UI gate

Source-level M11 remediation is now clean. M11 should not be marked PASS/CLOSED until the user accepts the current native Command Center/Tasks shell as a whole. The separately reported startup video, audio and native icon acceptance counts for those startup surfaces, but does not by itself silently substitute for the milestone-wide visual acceptance gate.

## Required next action

Do **not** run another R24/R25 remediation.
Do **not** start M12 yet.

Obtain the user's final native/visual acceptance of the current M11 surfaces. If accepted, perform a documentation/status-only M11 closure update:

- M11 -> PASS/CLOSED;
- strict completed roadmap progress -> `12 / 20 = 60%`;
- M12 -> READY;
- preserve historical REV4/REV5/REV6/REV7 audits and logs unchanged;
- preserve `H!veAI.mp4` as the current canonical startup runtime asset;
- do not begin M12 implementation in the same closure update.

## Final decision

**R24: PASS / CLOSED**

**R25: PASS / CLOSED**

**M11 SOURCE AUDIT: PASS**

**M11 MILESTONE: PENDING FINAL USER NATIVE/VISUAL ACCEPTANCE ONLY**
