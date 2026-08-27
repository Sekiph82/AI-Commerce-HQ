# M12B Native Open Cockpit Route-Loading Remediation Log

Date: 2026-08-27
Product: H!veAI
Branch: `H!veAI`

## Scope and synchronization

The M12B prompt was fetched and read after the mandatory safe preflight. The
preflight fast-forwarded local `HEAD` from
`0d15aea6cc2cb01d8c92c38758e011a05f056881` to
`24bbeb1dfdc11467357680f8a204ac144b9ec4e9`, with initial divergence
`0 2`. No parent-root user files were staged or changed. M13, M21, external
registered projects, Bulk Edit, startup video/icon behavior, and M12A workflow
history semantics were outside scope.

## Native reproduction and root cause

The governed executable `H!veAI/dev-bin/H!veAI.exe` was launched against the
real local H!veAI database at
`C:\Users\sekip\AppData\Roaming\ai.hiveai.desktop\hiveai.db`.
The selected Command Center project was:

`projectId=ba72f712-c7e6-47ce-af8c-dd9332539310` (`AI-Commerce-HQ`, `ACTIVE`).

The expected route was `/projects/ba72f712-c7e6-47ce-af8c-dd9332539310`.
The real registry list contained these eight IDs, all `ACTIVE`:

- `0b090590-59d5-4206-a59a-77e293631b0a` (`LLM`)
- `19d4a6b4-9c32-46e2-824b-2fa6a135346a` (`PackLab 3D`)
- `267479be-4d69-4b3e-aaf7-418c37203edf` (`FormuLab`)
- `7189068a-7205-4bde-a7ff-52ee81e3b4d4` (`fmcg-erp-system`)
- `977e3cda-25ec-479f-bba9-615d8ebb3bb8` (`Bulk-Edit`)
- `9e6f52d2-5584-4dcf-aa6c-b0a22f68fdd1` (`move-in-range`)
- `ba72f712-c7e6-47ce-af8c-dd9332539310` (`AI-Commerce-HQ`)
- `dc45cfde-88f2-4414-b3a0-3c1f33b8ee72` (`ScrubBots`)

The native `hiveai_project_get` lookup succeeded for the selected ID and
returned the same `ACTIVE` project identity. The native cockpit IPC call was
rejected before the Rust snapshot handler ran with the exact Tauri ACL message:

`Command hiveai_project_cockpit_snapshot not allowed by ACL`

The production `project_cockpit::snapshot` function was separately executed
against every one of the eight real registered records and returned a snapshot
whose project ID and name matched each requested registry record. This rules out
frontend ID routing, Tauri argument mapping, registry lookup, stale selection,
nested-project handling, and snapshot composition as the cause. The exact root
cause was that `hiveai_project_cockpit_snapshot` was registered in
`src-tauri/src/lib.rs` but was absent from the main-window capability and ACL
permission list. The old frontend catch path then incorrectly presented that
ACL failure as a project-not-found message.

## Production correction

- Added least-privilege `allow-project-cockpit` to
  `src-tauri/permissions/foundation.toml`.
- Added that permission to `src-tauri/capabilities/default.json`.
- Kept Command Center and Projects navigation on the exact registered UUID.
- Added route error classification for unknown identity, registered
  missing/archived identity, and registered snapshot failure without exposing
  raw errors, secrets, or stack traces.
- Added direct frontend routing/error tests, a native exact-ID snapshot test,
  and a capability regression test.
- Preserved M12A R26 project-wide workflow history behavior and all existing
  startup, shell, watcher, identity, and permission boundaries.

## Verification and publication

- `npm.cmd test -- --run tests/m12-project-cockpit-focused.test.tsx`: PASS, 8 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib project_cockpit::tests:: -- --nocapture --test-threads=1`: PASS, 8 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib main_window_allows_the_registered_project_cockpit_command -- --nocapture --test-threads=1`: PASS, 1 test.
- `npm.cmd test -- --run`: PASS, 95 tests. Existing React `act(...)` warnings remain non-failing.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib -- --nocapture --test-threads=1`: PASS, 287 tests executed, 0 failed.
- `npm.cmd run typecheck`: PASS.
- `npm.cmd run build`: PASS.
- `npm.cmd audit -- --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS, existing dead-code warnings only.
- `git diff --check`: PASS.
- `scripts/tests/publish-dev-qa-failure-harness.ps1`: PASS, 9/9 cases.
- `scripts/publish-dev-qa.ps1`: PASS. Production Tauri `--no-bundle` build,
  candidate smoke, frontend readiness, no forbidden development port, no new
  visible console host, stable replacement, and shortcut validation passed.

The newly published `H!veAI/dev-bin/H!veAI.exe` launched successfully with
native title `H!veAI`. The publisher's technical smoke verified readiness and
console suppression. Embedded WebView2 remote inspection was unavailable in
this environment, and native click-through of both route entry points could
not be independently instrumented; those native visual/interaction checks
remain explicitly unverified for the user rather than being claimed as
accepted. The real-data native handler probe and mounted frontend tests are the
available route/load evidence.

## Files changed

- `.hiveai/PROJECT_DASHBOARD.md`
- `CODEX_ROADMAP.md`
- `README.md`
- `TASKS.md`
- `docs/H!veAI/README.md`
- `src-tauri/capabilities/default.json`
- `src-tauri/permissions/foundation.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/src/project_cockpit.rs`
- `src/pages.tsx`
- `tests/m12-project-cockpit-focused.test.tsx`
- This immutable log.

## Git evidence

Implementation commit:
`19da7346a400d02f310ead5aed649df565a1c85e`

The implementation commit was pushed to `origin/H!veAI` and verified before
this log was created:

```text
git rev-parse HEAD
19da7346a400d02f310ead5aed649df565a1c85e

git rev-parse origin/H!veAI
19da7346a400d02f310ead5aed649df565a1c85e

git rev-list --left-right --count HEAD...origin/H!veAI
0 0
```

The final log commit is pushed separately. Its exact post-push local/origin
SHA and divergence proof are reported in the completion response and remain
`0 0` after publication.

## Final builder state

**M12B NATIVE OPEN COCKPIT REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE**
