# M05 — Project Registry Independent Audit

## Result

APPROVED WITH NON-BLOCKING FOLLOW-UP

## Audited evidence

- `H!veAI/docs/H!veAI/codex-logs/M05_PROJECT_REGISTRY_CODEX_LOG.md`
- `H!veAI/src-tauri/src/projects/registry.rs`
- branch `H!veAI` publication state
- M04 persistence foundation and M05 registry integration

## Findings

### PASS — explicit user-driven registration
Project registration is path-driven and explicit. There is no machine-wide automatic scan in M05.

### PASS — read-only external project handling
Registry operations inspect paths and Git metadata but do not write into registered project folders. Archive/remove operate on H!veAI registry rows only. Tests cover folder preservation.

### PASS — duplicate/path safety
Normalized path duplicate rejection exists. Repair validates a replacement path and rejects a conflicting repository identity when both old/new remotes are available.

### PASS — persistence boundary
M05 extends the existing M04 SQLite persistence layer rather than introducing another datastore.

### PASS — Git metadata containment
Git metadata detection is read-only and remote credential sanitization is documented/tested. No generic shell or Git mutation surface is introduced.

### PASS — UI/product branding
M05 log records canonical H!veAI and Akilta assets copied unchanged into `src/assets/`, H!veAI branding, and footer text `Built with ♥ for maximum productivity by Akilta`.

### PASS — regression / verification
Codex reports frontend typecheck, 10 tests, production build, 22 Rust tests, Rust format/check/build, bounded Windows smoke, zero legacy process/port-8765 listeners, and preserved M00-M04 historical logs.

## Non-blocking follow-up

1. M06 must keep Git inspection and Git mutation APIs separate.
2. All Git writes must be narrowly allowlisted and permission-gated; no arbitrary shell command surface.
3. M06 should use temporary repositories for mutation tests and must not use registered user repositories as write-test targets.
4. Continue Canonical UI Assets governance from `H!veAI/AGENTS.md` and milestone prompts.
5. Existing CSP production-hardening debt remains deferred unless the milestone requires a safe change.

## Decision

M05 is accepted. M06 may begin.

Exact next milestone: `M06 — Local Git Engine`.
