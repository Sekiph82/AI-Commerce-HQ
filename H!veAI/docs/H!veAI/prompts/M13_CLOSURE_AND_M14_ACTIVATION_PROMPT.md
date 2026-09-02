# M13 Closure and M14 Activation Prompt

Date: 2026-09-02
Product: H!veAI
Branch: `H!veAI`
Transition: Close M13 and activate M14 only

## Authority

This prompt is the authoritative status-transition instruction after successful completion of M13, all M13A-M13E remediations, independent strict re-audits, and explicit user native/visual acceptance.

Do not reinterpret this as an implementation prompt for M14. This run must close M13 truthfully, activate M14 as the next milestone, update canonical tracking, create immutable transition evidence, run the required transition gates, commit, and push. Stop before implementing M14.

## Accepted M13 evidence

Treat the following as accepted historical and current evidence. Do not rewrite or delete it:

- M13 initial Codex Adapter implementation
- M13 strict audit findings and all remediation history
- M13A R27-R29 closure: provider-neutral adapter contract, incremental bounded streaming, clean-stop-first and owned-tree escalation
- M13B R30-R31 closure: stateful pre-persistence redaction and durable event truth
- M13C R32 closure: deterministic native Windows `codex.exe` resolution
- M13D R33-R34 closure: no-visible-console process policy and successful real Codex operation compatibility with truthful diagnostics
- M13E native UX closure: persisted session output vertical reader with wrapped full-width downward reading
- independent strict re-audits for the accepted remediation chain
- user native/visual acceptance of:
  - Codex readiness/version in native H!veAI
  - zero unacceptable terminal flashing in the accepted flow
  - real ScrubBots Codex operation completing successfully with exit code 0
  - persisted session output readable vertically without required horizontal scrolling

The accepted real ScrubBots operation produced a `COMPLETED` session with exit code `0` and readable persisted evidence.

## Required status transition

Perform the following and nothing beyond it:

1. Mark M13 as `PASS/CLOSED` in every canonical live/current tracking surface.
2. Record that M13 native/visual acceptance was explicitly supplied by the user on 2026-09-02.
3. Preserve all historical FAIL audits and remediation records as immutable evidence.
4. Update strict completed roadmap progress from `13 / 20 = 65%` to `14 / 20 = 70%`.
5. Activate M14 as the current milestone.
6. Set M14 state to a truthful pre-implementation state such as `READY_FOR_IMPLEMENTATION` / `ACTIVE` according to the canonical workflow vocabulary already used by H!veAI.
7. Keep M15-M20 planned/blocked behind M14.
8. Keep M21 planned/not started and outside this roadmap run.
9. Do not modify accepted M13 runtime behavior merely as part of closure.
10. Do not implement any M14 feature in this run.

## M14 activation identity

Activate the existing roadmap milestone:

`M14 - Agent Session Center`

Use the existing canonical M14 scope from `TASKS.md` / `CODEX_ROADMAP.md`. Do not invent a competing milestone definition in this transition run.

The next action after this transition must be preparation/execution of the dedicated M14 implementation prompt, not implementation inside this closure run.

## Canonical tracking surfaces

Inspect and update only the current status/roadmap tracking surfaces that truthfully require this transition, including as applicable:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md`
- `H!veAI/.hiveai/PROJECT_DASHBOARD.md`

Do not rewrite old prompts, logs, or audits.

## Required immutable transition log

Create:

`H!veAI/docs/H!veAI/codex-logs/M13_CLOSURE_AND_M14_ACTIVATION_LOG.md`

The log must include:

- synchronized starting SHA
- M13 closure evidence inventory
- explicit user native/visual acceptance record
- exact status changes
- strict progress change `13/20 = 65%` -> `14/20 = 70%`
- confirmation M14 is activated but not implemented
- confirmation M15-M20 remain blocked/planned
- confirmation M21 remains planned/not started
- exact files changed
- every execution gate below with result
- implementation/status-transition commit SHA
- push/equality evidence

## Explicit execution gates

Run and record every gate separately.

1. `git fetch origin H!veAI`.
2. Synchronize by fast-forward only. No reset, rebase, force-push, or user-file discard.
3. Confirm exact branch is `H!veAI`.
4. Confirm unrelated user/untracked files are preserved and not staged.
5. Read the current canonical M13/M14 status from `TASKS.md`, `CODEX_ROADMAP.md`, and `.hiveai/PROJECT_DASHBOARD.md` before editing.
6. Verify the accepted M13E strict re-audit exists.
7. Verify M13D and earlier accepted remediation/audit evidence remains present and unchanged.
8. Update M13 to `PASS/CLOSED` only after confirming the accepted audit + user acceptance chain.
9. Update strict progress exactly to `14 / 20 = 70%`.
10. Activate M14 only. Do not activate M15 or any later roadmap milestone.
11. Confirm M14 implementation source is untouched in this run except status/tracking text that is strictly required for activation.
12. Confirm M21 is still planned/not started.
13. Run the focused tracking/status consistency checks available in the repository.
14. Run `git diff --check`.
15. Inspect the final diff and prove it contains only scoped status-transition/tracking/log changes.
16. Confirm no accepted startup video/icon/native UX assets changed.
17. Confirm no Codex adapter/process/security/session runtime source changed.
18. Confirm no project outside H!veAI was modified.
19. Commit the scoped transition.
20. Push to `origin/H!veAI`.
21. Fetch remote and prove local HEAD equals `origin/H!veAI` with divergence `0 0`.
22. Record the exact transition commit SHA in the immutable log.

## Final canonical state

After this run the truthful state must be:

- M00-M13: `PASS/CLOSED`
- Strict progress: `14 / 20 = 70%`
- M14: activated / ready for implementation
- M15-M20: planned/blocked
- M21: planned/not started

Final builder state must be exactly:

`M13 PASS/CLOSED / M14 ACTIVATED / PENDING M14 IMPLEMENTATION`

Stop after the transition. Do not implement M14 or M21.