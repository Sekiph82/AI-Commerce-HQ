# M12B Native Open Cockpit Route-Loading Strict Re-Audit

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`
Audited log: `H!veAI/docs/H!veAI/codex-logs/M12B_NATIVE_OPEN_COCKPIT_ROUTE_LOADING_REMEDIATION_LOG.md`
Audited implementation commit: `19da7346a400d02f310ead5aed649df565a1c85e`

## Verdict

**PASS / SOURCE + BUILD EVIDENCE ACCEPTED / USER NATIVE INTERACTION ACCEPTANCE STILL REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 1
- Confidence: HIGH

## Root cause validation

The remediation log identifies the native failure as an ACL/capability rejection of `hiveai_project_cockpit_snapshot` before the Rust handler ran. This matches the production correction: the main-window capability now includes `allow-project-cockpit`, and `foundation.toml` defines that permission narrowly for only `hiveai_project_cockpit_snapshot`.

The existing command remains registered in `src-tauri/src/lib.rs`, so the missing ACL edge was the relevant boundary rather than a missing handler.

## Production correction review

PASS:

1. `src-tauri/capabilities/default.json` now includes `allow-project-cockpit` in the main-window permission set.
2. `src-tauri/permissions/foundation.toml` defines a least-privilege permission whose only allowed command is `hiveai_project_cockpit_snapshot`.
3. Command Center navigation continues to route with the exact registered `projectId`.
4. Project Cockpit still performs an explicit registry lookup before the snapshot request.
5. The frontend now distinguishes an unknown project ID, a registered unavailable project, and a registered-project snapshot failure instead of collapsing all native failures into a false project-not-found message.
6. No raw native error or stack trace is exposed to the user.
7. M12A R26 project-wide workflow history behavior is preserved.

## Regression evidence

Builder evidence reports:

- focused M12 frontend tests: 8 PASS;
- focused Project Cockpit native tests: 8 PASS;
- direct capability test: PASS;
- full frontend regression: 95 PASS;
- full Rust regression: 287 PASS, 0 failed;
- typecheck/build/npm audit/cargo fmt/cargo check/git diff check: PASS;
- publication failure harness: 9/9 PASS;
- governed dev QA publication: PASS.

The remediation also exercised the real H!veAI registry database and proved that the selected registered project ID resolves through `hiveai_project_get`, while the pre-fix cockpit IPC was rejected specifically by Tauri ACL. It separately exercised the production snapshot composition for the real registered records. This is materially stronger evidence than the original M12 route tests.

## Scope discipline

PASS. The implementation is bounded to M12 native cockpit route loading, ACL/capability exposure, truthful route-error handling, supporting tests, and canonical status documentation. M13 and M21 were not started.

## Remaining acceptance gate

### NOTE N01 - User native click-through remains required

The builder could not instrument a complete WebView2 click-through after publication. Therefore final M12 native acceptance must still be performed by the user on the newly published `H!veAI/dev-bin/H!veAI.exe`.

Required check:

1. Launch the newly published executable.
2. From Command Center select a registered ACTIVE project and click `Open cockpit`.
3. Confirm the Project Cockpit loads for that exact selected project.
4. From Projects, open at least one registered ACTIVE project cockpit as well.
5. Confirm the prior `Unable to load view / Registered project was not found or could not be loaded` failure no longer occurs.
6. Confirm the cockpit tabs render and can be switched without route failure.

If these pass, M12 native/visual acceptance can proceed. If the same failure remains, capture the exact selected project and visible error state before any additional remediation.

## Closure

**M12B STRICT RE-AUDIT: PASS**

**M12 overall remains open only for repeat user native/visual acceptance.**

Strict roadmap progress remains `12/20 = 60%` until M12 is canonically closed.
