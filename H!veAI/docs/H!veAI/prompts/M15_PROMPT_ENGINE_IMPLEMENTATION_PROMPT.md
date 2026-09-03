# M15 Prompt Engine Implementation Prompt

Date: 2026-09-03
Product: H!veAI
Branch: `H!veAI`
Milestone: M15 - Prompt Engine
Authority: authoritative implementation prompt

## 0. Milestone transition authority

M14 is now authorized to close.

The independent M14E strict re-audit is PASS with 0 BLOCKER / 0 MAJOR / 0 MINOR, and the user has now supplied native acceptance proving both required gates:

1. a new real ScrubBots + Claude session shows the complete final assistant answer in `Current conversation`, not only intermediate progress text;
2. normal use presents the user prompt and final assistant answer first, while technical details, Timeline, Raw events, and Git evidence remain secondary/collapsed unless explicitly opened.

Therefore this run is authorized to:

1. update canonical trackers so M14 is `PASS/CLOSED`;
2. advance strict completed roadmap progress from `14 / 20 = 70%` to `15 / 20 = 75%`;
3. activate and implement M15 only;
4. leave M16-M20 planned/blocked;
5. leave M21 planned/not started.

Do not rewrite or delete historical M14/M14A/M14B/M14C/M14D/M14E prompts, logs, audits, failure evidence, or remediation history.

## 1. Product goal

Implement the H!veAI Prompt Engine described by `TASKS.md` and `CODEX_ROADMAP.md`.

M15 must make builder prompts first-class, versioned, reviewable, reproducible objects with immutable provenance from:

`project/task/context -> prompt version -> explicit human approval -> selected provider -> agent session`

The resulting system must support implementation prompts, remediation prompts, and audit-support prompts without turning H!veAI into an unrestricted arbitrary-prompt launcher.

Every dispatched builder prompt must be reproducible and traceable.

## 2. Canonical M15 scope

Implement all packages M15.01 through M15.08:

- M15.01 Prompt kinds/schemas/types.
- M15.02 Versioning and immutability.
- M15.03 Bounded context collector.
- M15.04 Implementation prompt generation.
- M15.05 Audit-driven remediation prompt generation.
- M15.06 Human review/edit/approve.
- M15.07 Provider dispatch + prompt/session provenance.
- M15.08 Version/context/provenance tests, regression, publication, and independent-audit readiness.

Do not implement M16+ functionality in advance.

## 3. Inherited non-negotiable boundaries

Preserve all accepted boundaries from M04-M14, including:

- Registry-backed ACTIVE project authority;
- exact project/task identity validation;
- M08 source-discovery authority;
- M09 parsed task/evidence authority;
- M10 workflow truth;
- M11/M12 Project Dashboard authority/provenance rules;
- M13/M14 provider/process confinement;
- Codex + Claude first-class provider support;
- no arbitrary executable, shell, argument-vector, or PID control;
- prompt content must not be placed in provider argv;
- bounded stdin prompt transport;
- secret-safe persistence and output sanitization;
- durable agent/session/final-response truth;
- no visible background console windows;
- safe Git Engine authority;
- governed stable EXE publication;
- historical prompt/log/audit immutability.

M15 may add narrow schema/migrations and IPC required for prompts, context, approval and provenance, but must not weaken any accepted process or filesystem security boundary.

## 4. M15.01 Prompt kinds, schemas and native types

Define a canonical prompt-domain model.

Required prompt kinds at minimum:

- `IMPLEMENTATION`
- `REMEDIATION`
- `AUDIT_SUPPORT`

A prompt record/version must be able to represent at minimum:

- stable prompt identity;
- project ID;
- optional task ID;
- kind;
- title/summary;
- immutable prompt body/version body;
- version number or monotonic version identity;
- creation timestamp;
- creator/origin metadata;
- context snapshot/reference;
- approval state;
- approval timestamp when approved;
- approved body hash;
- selected provider when dispatched;
- dispatched session ID when applicable;
- superseded/current-version relationship;
- provenance references to source task, audit finding(s), or user edit as applicable.

Do not conflate mutable prompt identity with immutable prompt versions.

Use explicit enums/state values rather than loose strings where practical.

## 5. M15.02 Versioning and immutability

The existing `prompts` / `prompt_versions` foundation from M04 must be inspected before adding schema.

Required behavior:

- editing a draft creates or updates only a not-yet-used editable draft representation according to a clearly documented model;
- once a prompt version is approved and/or used by an agent session, its exact body must never be mutated in place;
- any later edit creates a new version;
- current version is tracked separately from historical versions;
- historical versions remain queryable;
- a session references one exact immutable prompt version;
- prompt/version hashes are deterministic and reproducible;
- migration remains additive, ordered, idempotent, corruption-safe, and compatible with historical rows.

Add adversarial tests proving an already-dispatched version cannot be silently altered.

## 6. M15.03 Bounded context collector

Build a native bounded context collector that constructs explainable prompt context from existing H!veAI authorities rather than recursively scraping arbitrary project content.

Allowed context classes include:

- current project identity/status/path metadata;
- selected task requirements/state/dependencies;
- M08-approved source evidence;
- M09 parsed evidence/locators;
- Project Dashboard authority roles/provenance;
- relevant architecture/governance source(s);
- relevant Git snapshot/diff metadata;
- relevant test evidence;
- relevant audit findings for remediation prompts;
- relevant recent agent/session outcome metadata where directly useful.

Requirements:

- enforce project containment;
- use existing source/authority APIs rather than duplicating filesystem discovery;
- apply explicit byte/item/source-count bounds;
- deterministic ordering;
- record what was included, omitted, truncated, stale, unavailable, or excluded;
- never silently claim missing context was included;
- never load secrets, `.env`, credential stores, local app databases, build caches, or unapproved arbitrary files into generated prompts;
- avoid dumping huge governance documents when a narrow relevant excerpt/reference is enough;
- output an explainable context manifest that can be inspected by the user.

The context collector is not an autonomous web researcher and is not a replacement for M08/M09.

## 7. M15.04 Implementation prompt generation

Generate builder-ready implementation prompts from current project/task/context truth.

A generated implementation prompt should contain only useful execution contract material, including where applicable:

- task/product goal;
- project/task identity;
- authoritative source/context references;
- current behavior/current state;
- required target behavior;
- exact scope boundary;
- inherited security/architecture constraints;
- acceptance criteria;
- focused tests expected;
- relevant regression gates;
- required logging/provenance;
- explicit prohibited shortcuts.

Do not inject pages of irrelevant repeated governance text merely because files exist.

Generated prompt content must be deterministic for the same frozen context snapshot, excluding clearly documented volatile fields such as generation timestamp if included.

## 8. M15.05 Remediation prompt generation

Support audit-driven remediation prompts.

Input must be real persisted/selected audit findings or explicit bounded finding data, not invented defects.

For each included finding preserve at minimum:

- originating milestone/finding ID;
- severity;
- exact subsystem/file/symbol if known;
- observed incorrect behavior;
- target behavior;
- focused regression that must fail pre-fix and pass post-fix where feasible;
- security/safety constraints;
- closure criteria.

Remediation generation must stay defect-focused and must not silently expand into unrelated cleanup or the next milestone.

If required audit evidence is missing, show that truthfully instead of fabricating remediation detail.

## 9. M15.06 Review, edit and explicit human approval

Add a user-facing Prompt Engine workflow suitable for the native H!veAI design system.

The user must be able to:

- choose project;
- optionally choose task;
- choose prompt kind;
- generate/refresh bounded context;
- inspect context/provenance;
- generate a draft prompt;
- review the prompt in a large readable editor;
- edit before approval;
- see whether the edit will create a new version;
- explicitly approve the exact version/body;
- choose provider `CODEX` or `CLAUDE` for dispatch;
- dispatch only after approval;
- inspect version history/provenance.

Critical UX rule:

Do not auto-dispatch generated prompts. Approval and dispatch must be explicit user actions.

Do not make technical hashes/IDs the primary visual surface. Keep them available as secondary details.

Do not regress the chat-first Agents experience accepted in M14E.

## 10. M15.07 Dispatch and provenance

Dispatch must reuse the existing M14 Agent Session Center/provider contract.

Do not create a second provider launcher.

On dispatch:

- validate ACTIVE project again;
- validate task/project relationship again when a task is present;
- validate the prompt version is approved;
- validate its body/hash still matches the approved immutable version;
- create a NEW owned agent session through the selected Codex/Claude adapter;
- attach exact prompt ID + version ID + version hash to session provenance;
- preserve provider/project/task association;
- preserve final assistant response capture;
- preserve stop/recovery/security behavior;
- make dispatched session navigable/inspectable in Agents;
- preserve immutable historical prompt provenance even after later prompt versions exist.

A session must never be ambiguously linked to "latest prompt". It must reference the exact version actually dispatched.

If dispatch fails before process start, persist truthful failure/provenance without pretending a provider session ran.

## 11. Prompt Engine UI placement

Use the existing application shell and UI governance.

Prefer a dedicated Prompt Engine workspace/route only if this matches current routing architecture cleanly; otherwise integrate it into the most appropriate existing task/project workflow without overloading Agents.

Regardless of placement, the primary workflow should feel like:

`Context -> Draft -> Review/Edit -> Approve -> Dispatch -> Session`

Keep context evidence and version metadata inspectable but secondary.

No browser-hosted shell, no redesign of the global sidebar/logo/footer language, no global zoom hacks, and no page-level horizontal scrolling.

## 12. Required direct acceptance scenarios

### Scenario A: implementation prompt

Using a disposable or safe registered ACTIVE project fixture:

1. choose a real task;
2. collect bounded context;
3. generate an IMPLEMENTATION draft;
4. inspect provenance/context manifest;
5. edit one visible sentence;
6. approve exact edited version;
7. dispatch to a selected provider;
8. verify the created agent session references the exact approved prompt version/hash;
9. verify later prompt edits create a new version without modifying the dispatched one.

### Scenario B: remediation prompt

Using persisted test audit findings/fixture:

1. select finding(s);
2. generate a REMEDIATION draft;
3. prove finding IDs/severity/behavior/closure criteria are preserved;
4. approve version;
5. prove unrelated findings/milestones are not silently included.

### Scenario C: unapproved dispatch denial

Attempt to dispatch:

- an unapproved draft;
- a superseded-but-unapproved edited body;
- a version whose persisted body/hash no longer matches approval evidence.

All must fail safely and truthfully before provider launch.

### Scenario D: historical immutability

Dispatch prompt version N, create version N+1, then reload both and the session. Prove the old session still points to N with original body/hash and N+1 remains separate.

## 13. Security and privacy tests

Add adversarial tests for at least:

- cross-project prompt access rejection;
- task/project mismatch rejection;
- inactive/missing/archived project rejection;
- arbitrary source-path injection rejection;
- containment/symlink escape rejection where applicable;
- secret/.env exclusion from context;
- local H!veAI database exclusion from context;
- context byte/item/source bounds;
- prompt body size bounds;
- unauthorized/unapproved dispatch rejection;
- approval hash mismatch rejection;
- immutable used-version mutation rejection;
- session provenance tamper rejection;
- provider mismatch handling;
- prompt absent from provider argv;
- no arbitrary executable/args/shell/PID surface;
- historical prompt/session isolation;
- sanitizer/final-response regressions from M14 remain green.

## 14. Canonical tracker transition

At the beginning of this run, after synchronization and evidence verification:

- update `H!veAI/TASKS.md` so M14 is PASS/CLOSED and M15 is active/implementing;
- update `H!veAI/CODEX_ROADMAP.md` consistently;
- update `H!veAI/docs/H!veAI/README.md` high-level truth;
- set strict progress to `15 / 20 = 75%` only because M14 has now been accepted/closed;
- do not mark M15 PASS/CLOSED from builder work alone.

At builder completion, M15 may become:

`IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Do not activate M16.

## 15. Explicit execution gates

Record every gate individually in the builder log.

1. `git fetch origin H!veAI`.
2. Compare local HEAD vs `origin/H!veAI`.
3. Fast-forward only if safe.
4. Confirm exact Git root.
5. Confirm exact `H!veAI` branch.
6. Record starting HEAD/status/worktrees/remotes.
7. Preserve unrelated user-owned files and unstaged work.
8. Read `H!veAI/AGENTS.md`.
9. Read `H!veAI/CONSTITUTION.md`.
10. Read `H!veAI/ARCHITECTURE.md`.
11. Read `H!veAI/TASKS.md` M15 section.
12. Read `H!veAI/CODEX_ROADMAP.md` M15 section.
13. Read the accepted M14E strict re-audit.
14. Verify M14 native acceptance authority from this prompt before closing M14 trackers.
15. Update M14 PASS/CLOSED, M15 ACTIVE, progress 15/20=75% in canonical trackers.
16. Inspect current `prompts` and `prompt_versions` schema before migration/design.
17. Inspect current agent/session provenance fields before adding any new link.
18. Inspect M08/M09/M10/M11/M12 APIs available for bounded context rather than duplicating authority.
19. Implement canonical prompt kinds/types.
20. Implement immutable version model.
21. Implement deterministic prompt/version hashing.
22. Implement current-version relationship.
23. Implement used-version mutation denial.
24. Implement bounded context manifest/model.
25. Implement project/task authority validation in context collection.
26. Implement context byte/item/source-count bounds.
27. Implement context secret/exclusion rules.
28. Implement deterministic context ordering.
29. Implement implementation-prompt generation.
30. Implement remediation-prompt generation.
31. Implement truthful audit-finding provenance.
32. Implement review/edit workflow.
33. Implement explicit approval workflow.
34. Implement approval hash/body verification.
35. Implement provider selection for dispatch.
36. Reuse M14 Agent Session Center dispatch path.
37. Attach exact prompt/version provenance to session.
38. Ensure prompt remains absent from provider argv.
39. Ensure dispatch cannot create arbitrary executable/args/shell/PID control.
40. Implement version/history retrieval.
41. Implement context/provenance inspection UI.
42. Implement readable prompt editor UI.
43. Ensure approval and dispatch are separate explicit actions.
44. Ensure no auto-dispatch occurs after generation.
45. Add implementation-prompt focused backend tests.
46. Add remediation-prompt focused backend tests.
47. Add version immutability tests.
48. Add approval/hash mismatch tests.
49. Add session provenance tests.
50. Add context-bound tests.
51. Add source/path containment tests.
52. Add secret/.env/database exclusion tests.
53. Add cross-project isolation tests.
54. Add inactive project rejection tests.
55. Add task/project mismatch tests.
56. Add unapproved dispatch denial tests.
57. Add historical N/N+1 version tests.
58. Add provider-dispatch integration fixture tests for Codex + Claude.
59. Verify final-response capture still works through Prompt Engine dispatch.
60. Run focused M13/M14 process/security regression tests.
61. Run focused M15 Rust tests.
62. Run full serial Rust regression.
63. Run focused Prompt Engine frontend tests.
64. Run existing Agents frontend tests.
65. Run full frontend tests.
66. Run TypeScript typecheck.
67. Run frontend production build.
68. Run `npm audit --audit-level=high`.
69. Run Rust fmt check.
70. Run Rust all-targets check.
71. Run Rust `pty-support` check if feature remains present.
72. Run `git diff --check`.
73. Run migration apply/reapply/history compatibility tests.
74. Run publisher failure/rollback harness.
75. Run governed production Tauri `--no-bundle` publication.
76. Verify candidate/stable EXE SHA equality.
77. Verify stable EXE PE/startup/readiness smoke.
78. Verify stable Desktop shortcut target and accepted icon.
79. Verify no visible background console popup regression.
80. Execute Scenario A end-to-end with safe fixture/provider path.
81. Execute Scenario B remediation-generation fixture.
82. Execute Scenario C unapproved dispatch denial.
83. Execute Scenario D historical immutability.
84. If safe, execute one real read-only approved prompt through Claude or Codex against a disposable/explicitly safe registered project and verify exact prompt-version provenance in the resulting session.
85. Verify no tracked mutation occurs in a read-only acceptance target.
86. Verify primary Prompt Engine UI has no page-level horizontal scrolling at normal laptop viewport.
87. Verify historical prompt versions are readable but not editable in place after use.
88. Verify technical hashes/provenance are available without dominating primary UX.
89. Re-read changed production files for security/self-audit.
90. Confirm no M16 code/trackers were activated.
91. Confirm M21 remains not started.
92. Create immutable builder log at `H!veAI/docs/H!veAI/codex-logs/M15_PROMPT_ENGINE_IMPLEMENTATION_LOG.md`.
93. Log exact commands, failures, retries, test counts, migrations, publication evidence, and acceptance limitations truthfully.
94. Update trackers only to M15 implementation-complete/pending-audit state, never builder-declared PASS/CLOSED.
95. Commit only scoped changes.
96. Push to `origin/H!veAI` without force.
97. Verify final local HEAD equals remote branch HEAD.
98. Record final commit SHA and changed-file scope.

## 16. Required builder log

Create:

`H!veAI/docs/H!veAI/codex-logs/M15_PROMPT_ENGINE_IMPLEMENTATION_LOG.md`

The log must include:

- synchronization/start HEAD;
- M14 closure tracker updates;
- schema/migration decisions;
- prompt/version model;
- context collector bounds and exclusions;
- generation rules;
- approval semantics;
- dispatch/provenance wiring;
- focused tests;
- full regression counts;
- publication evidence;
- native/manual items not independently accepted;
- implementation commit SHA;
- final branch equality.

Builder log claims are not independent acceptance.

## 17. Completion boundary

Success means M15 implementation is complete and auditable, not automatically closed.

At the end:

- M14 = PASS/CLOSED;
- strict roadmap progress = `15 / 20 = 75%`;
- M15 = IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE;
- M16-M20 remain planned/blocked;
- M21 remains planned/not started.

Do not start M16 or M21.
