# M12B Native Open Cockpit Route Loading Remediation

## Authority

Work only on the `H!veAI` branch.

This is a bounded M12 native-acceptance remediation. Do not start M13 or M21.

The user has now performed the required native acceptance test on `H!veAI/dev-bin/H!veAI.exe` and M12 native acceptance currently FAILS.

Observed native behavior:

1. Command Center renders normally and a registered project (for example `AI-Commerce-HQ`) is selected.
2. Clicking `Open cockpit` navigates to the Project Cockpit route.
3. The native Project Cockpit displays:
   - `Unable to load view`
   - `Registered project was not found or could not be loaded.`
4. Therefore M12 must remain NOT CLOSED until this route/load failure is fixed and user acceptance is repeated.

The current frontend flow is known to navigate with:

`navigate(`/projects/${current.projectId}`)`

and the route is:

`/projects/:id`

The Project Cockpit then executes both the registered-project lookup and the native cockpit snapshot lookup. The current catch path collapses their failures into the same generic message, which is insufficient for diagnosis.

Do not guess the root cause. Reproduce it against the actual governed native QA executable and identify the exact failing boundary.

---

## Required diagnosis

Synchronize safely with `origin/H!veAI`, then inspect at minimum:

- `H!veAI/src/command_center_view.tsx`
- `H!veAI/src/pages.tsx`
- `H!veAI/src/App.tsx`
- `H!veAI/src/projectRegistry.ts`
- `H!veAI/src/projectCockpit.ts`
- `H!veAI/src/registryContext.tsx`
- `H!veAI/src-tauri/src/lib.rs`
- `H!veAI/src-tauri/src/projects.rs`
- `H!veAI/src-tauri/src/project_cockpit.rs`
- any M12/M12A focused tests that exercise Project Cockpit routing

Reproduce the failure with real registered-project data, not fixture-only browser data.

Capture and record separately:

1. selected Command Center `projectId`;
2. route parameter `id` after navigation;
3. result/error from `hiveai_project_get`;
4. result/error from `hiveai_project_cockpit_snapshot`;
5. actual registered project IDs returned by `hiveai_projects_list`;
6. whether the failing project is ACTIVE, MISSING, ARCHIVED, nested, Git/non-Git, or otherwise degraded;
7. the exact native error code/message before the UI maps it to presentation text.

Prove whether the defect is frontend identity/routing, Tauri argument mapping, registry lookup, cockpit snapshot composition, stale selected-project identity, nested-project handling, or another concrete cause.

Do not paper over the defect by removing validation or by redirecting to a different project.

---

## Production acceptance contract

After the fix, for every registered project shown in Command Center or Projects:

- `Open cockpit` must open the cockpit for exactly that registered project ID;
- no fallback to another project is allowed;
- stale/late route responses must not replace a newer selection;
- ACTIVE valid projects must load their native cockpit snapshot;
- MISSING/ARCHIVED/degraded projects must render a truthful project-specific state rather than a false generic not-found when identity is still registered;
- a genuinely unknown registry ID must still fail truthfully;
- cross-project data leakage remains forbidden;
- M12A project-wide workflow-history guarantees remain intact.

If the registered-project identity exists but one cockpit subsystem is degraded (Git, task intelligence, sources, dashboard, etc.), prefer a loaded cockpit with explicit warnings/unknown states where the M12 contract permits degradation. A non-critical subsystem failure must not incorrectly masquerade as `Registered project was not found`.

If cockpit snapshot composition currently treats optional/degraded evidence as fatal contrary to the M12 contract, correct that boundary without weakening real registry/project identity failures.

---

## Error-state quality

Improve the route error boundary enough that native failures remain diagnosable and truthful.

At minimum distinguish:

- genuinely unregistered/unknown project identity;
- registered project that is missing/archived/unavailable;
- registered project whose cockpit snapshot failed for another native reason.

Do not expose secrets or stack traces in the normal UI.

Tests must be able to assert the underlying error classification.

---

## Required tests

Add direct regression coverage for the actual discovered defect.

At minimum include:

1. Command Center selected project ID -> `/projects/:id` -> exact same registered project ID.
2. Projects page Open Cockpit -> exact same registered project ID.
3. Native registered ACTIVE project successfully reaches `hiveai_project_cockpit_snapshot` and renders its identity.
4. Two registered projects with rapid switching cannot cross-load each other's cockpit data.
5. Unknown route ID remains a truthful not-found.
6. Registered MISSING/ARCHIVED/degraded identity gets the correct explicit state rather than accidental cross-project fallback.
7. A degradable optional cockpit subsystem failure does not incorrectly become registry-not-found, if this is part of the discovered root cause.
8. Existing M12A R26 workflow-history starvation/tie-order/project-isolation tests remain PASS.

Prefer direct Rust/native tests for backend ownership/error semantics and mounted frontend tests for routing and presentation.

---

## Verification and publication

Run all relevant focused tests plus the established full gates:

- M12/M12B focused frontend tests;
- M12/M12A focused Rust tests;
- complete frontend test suite;
- complete Rust lib test suite;
- TypeScript typecheck;
- frontend production build;
- npm high-level audit;
- cargo fmt check;
- cargo check;
- `git diff --check`;
- governed publication failure harness;
- governed `publish-dev-qa.ps1` production Tauri `--no-bundle` publication.

Launch the newly published:

`H!veAI/dev-bin/H!veAI.exe`

and technically verify with real registered project data that:

- Command Center `Open cockpit` opens the selected project;
- Projects-page `Open cockpit` opens the selected project;
- at least two different registered projects open with different correct identities;
- no terminal/console flash regression occurs.

Do not claim user visual/native acceptance. The user will repeat that acceptance after publication.

---

## Scope protection

Do not change unrelated M12 visual design.

Do not start or modify:

- M13 Codex Adapter;
- M21 standalone migration;
- external registered project repositories;
- Bulk Edit;
- startup video/icon behavior;
- M11 closed behavior except where a narrowly necessary route identity call must be corrected;
- M12A workflow-history semantics except preserving them.

Do not alter registry identity semantics merely to make the screen open unless the strict diagnosis proves that the identity implementation itself is defective.

---

## Evidence log

Create:

`H!veAI/docs/H!veAI/codex-logs/M12B_NATIVE_OPEN_COCKPIT_ROUTE_LOADING_REMEDIATION_LOG.md`

Include:

- exact reproduced native failure;
- exact root cause;
- selected `projectId`, route `id`, registry lookup result and cockpit snapshot error during reproduction;
- production fix;
- files changed;
- focused adversarial tests;
- all regression/build/publication results;
- implementation commit SHA;
- final local HEAD;
- fetched `origin/H!veAI`;
- final divergence count;
- explicit statement that user native acceptance remains pending.

Commit and push all scoped changes to `origin/H!veAI`.

Final builder state:

`M12B NATIVE OPEN COCKPIT REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

Stop. Do not start M13 or M21.
