# PRE-M10 Native UX Hotfix X01/X02 — Independent Strict Audit

Date: 2026-08-25
Branch audited: `H!veAI`
Builder implementation commit: `df74aff0ba04e8035baa308863bc1d9489d853b8`
Current audited branch context: later docs-only Project Dashboard commits are present after the builder hotfix; no hotfix production source was changed by those docs commits.

## Verdict

**CONDITIONAL**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- Manual native acceptance: **PENDING / REQUIRED**
- Confidence: HIGH for source/config correctness, MEDIUM for the two visible/audible native UX outcomes until user acceptance is supplied.
- Regression risk: LOW-MEDIUM.

The automated/source-level hotfix is acceptable. M10 remains BLOCKED only because the prompt explicitly requires real native user acceptance for X01 visible console suppression and X02 audible startup playback.

---

## Scope / change truth

Accepted production changes are bounded to the intended hotfix areas:

- `H!veAI/src-tauri/src/git_engine/mod.rs`
- `H!veAI/src/components/StartupIntro.tsx`
- `H!veAI/src-tauri/tauri.conf.json`
- focused tests
- tracker/log documentation

No M09 parser production change is part of the hotfix. No M10 workflow implementation is present in the hotfix. No installer was introduced by the hotfix log evidence.

Historical/current Project Dashboard documentation commits after the builder session are documentation-only and do not change this hotfix verdict.

---

# X01 — Windows Git child console suppression

## AC-X01.1 — Shared production Git path applies no-console policy

**PASS**

Production source defines one shared command builder:

```rust
fn production_git_command() -> Command {
    let mut command = Command::new("git");
    #[cfg(windows)]
    command.creation_flags(0x08000000);
    command
}
```

`run_git()` uses that builder before `.args(...)`, stdio configuration and `.spawn()`.

This is the correct Windows `CREATE_NO_WINDOW` flag and it is guarded by `#[cfg(windows)]`, leaving non-Windows behavior unchanged.

## AC-X01.2 — Existing process behavior preserved

**PASS**

The production path continues to preserve:

- direct `git` executable invocation;
- repository `current_dir`;
- null stdin;
- piped stdout/stderr;
- the existing spawn/timeout/kill/error/output-bounding flow.

No CMD/PowerShell/shell-string workaround was introduced.

## AC-X01.3 — No separate reachable production Git spawn bypass

**PASS**

The Git mutation module routes production branch/stage/commit/push helpers through `run_git()`. The direct `Command::new("git")` usages visible in that module are inside `#[cfg(test)]` test helpers only.

Project Registry Git metadata detection reads `.git` files directly and does not create a competing production Git process path.

Watcher production integration imports `git_engine::snapshot` and therefore reaches the corrected shared `run_git()` path for watcher-triggered Git snapshots.

## AC-X01.4 — Focused/regression evidence

**PASS** for automated evidence.

Direct tests exercise the production `run_git()` path for successful output and structured errors. Existing real-temp-repo Git and watcher tests continue to exercise snapshot/refresh behavior. Builder log records focused Git Engine and watcher suites plus full Rust regression as green.

This automated evidence does **not** prove that Windows never flashes a console window in a real desktop session. The prompt correctly reserves that observation for the user.

## AC-X01.5 — Native visible-window acceptance

**UNVERIFIED / REQUIRED**

Required user acceptance remains:

1. launch the stable native Desktop shortcut;
2. leave H!veAI open for several minutes;
3. cause real changes in one or more registered repositories;
4. confirm no terminal/console windows flash or remain open;
5. confirm H!veAI stays responsive and watched Git state still refreshes.

X01 cannot be marked CLOSED before this evidence is supplied.

---

# X02 — Audible startup intro

## AC-X02.1 — Explicit mute removed

**PASS**

`StartupIntro.tsx` no longer renders the `<video>` element with a `muted` attribute.

Before `play()` the code explicitly sets:

```ts
video.muted = false;
video.volume = 1;
```

and then attempts `video.play()`.

## AC-X02.2 — Process-scoped startup lifecycle preserved

**PASS**

The existing native startup claim remains the gate for playback. Browser preview still skips the native intro path. The component remains a fullscreen overlay and keeps the existing ended/error/failsafe dismissal lifecycle.

No application-mount or frontend-ready coupling was added by the hotfix.

## AC-X02.3 — WebView2 audible autoplay policy

**PASS** for source/config correctness.

The main Tauri window now configures:

```text
--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --autoplay-policy=no-user-gesture-required
```

The hotfix retains the WRY default feature-disable argument set recorded by the builder while adding the audible autoplay policy. CSP was not weakened by this change.

## AC-X02.4 — Focused frontend/config evidence

**PASS** for automated evidence.

Focused tests prove:

- native claimed playback is prepared with `muted=false` and `volume=1`;
- `play()` is attempted;
- the video DOM element lacks a `muted` attribute;
- ended playback dismisses the overlay;
- browser preview does not invoke/play native intro;
- same mounted process does not re-request/replay the claim on rerender;
- native window config contains the expected WebView2 arguments.

Builder log truthfully records that the first focused attempt had test-harness failures and that the tests were corrected before the passing rerun.

These jsdom/config tests cannot prove actual audible Windows WebView2 output.

## AC-X02.5 — Canonical media preservation

**PASS** based on builder publication evidence and source diff scope.

Builder log records the canonical opening-video SHA-256 unchanged as:

`A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

No MP4 replacement/transcode is present in the hotfix source diff.

## AC-X02.6 — Native audible/restart acceptance

**UNVERIFIED / REQUIRED**

Required user acceptance remains:

1. fully close H!veAI;
2. launch from `Desktop\H!veAI.lnk`;
3. confirm video is visible and audio is audible;
4. use the real native Restart H!veAI flow;
5. confirm video + audio play again in the new process;
6. navigate between routes in the same process and confirm intro does not replay;
7. confirm no new outer-scroll/layout regression during the intro.

X02 cannot be marked CLOSED before this evidence is supplied.

---

# Regression / publication review

Builder log reports the following completed gates:

- `npm run typecheck` PASS
- frontend tests PASS, 75 tests
- frontend production build PASS
- `npm audit --audit-level=high` PASS, 0 vulnerabilities
- cargo fmt/check/test/build PASS
- Rust tests PASS, 192 tests
- publisher failure/rollback harness PASS, 9 assertions
- governed Tauri `--no-bundle` QA publication PASS
- stable EXE published with recorded SHA-256
- stable shortcut/icon preserved
- no installer artifacts

These execution results remain builder claims, but they align with the inspected source/config and no contradictory repository evidence was found.

---

# Tracker truth

Correct current state after this audit:

- M00-M09 = PASS/CLOSED.
- Strict completed roadmap count = **10 / 20 = 50%**.
- Pre-M10 X01/X02 automated/source audit = **CONDITIONAL, 0 BLOCKER / 0 MAJOR / 0 MINOR**, pending native manual acceptance.
- M10 = **BLOCKED/UNSTARTED** until X01 and X02 manual acceptance is supplied and the hotfix is then marked CLOSED.
- M11/M12 remain planned/blocked behind M10.

The later cross-repository Project Dashboard design is documentation/planning only and must not be mistaken for implemented M11/M12 runtime behavior.

---

# Closure condition

No further code remediation prompt is justified by this audit.

Once the user supplies successful native acceptance for both X01 and X02, this hotfix can be marked **PASS/CLOSED** without another builder code run unless the manual test exposes a regression.
