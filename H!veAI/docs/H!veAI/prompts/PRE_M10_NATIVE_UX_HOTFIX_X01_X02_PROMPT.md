# PRE-M10 Native UX Hotfix — X01 Terminal Popups + X02 Startup Audio

## Mission

Fix exactly two known native UX defects before M10:

- X01: H!veAI can spawn visible Windows console/terminal windows while the application remains open and watcher-driven Git refreshes run.
- X02: the canonical opening video contains audio, but `StartupIntro.tsx` explicitly renders the video with `muted`, so native startup is silent.

This is a pre-M10 hotfix, not M10.

Do not implement Workflow State Machine behavior.
Do not redesign the Command Center.
Do not change the canonical video bytes.
Do not create an installer.
Do not change unrelated permissions, Registry behavior, M08 discovery, or M09 parser behavior.

---

## Start

Work from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

Run:

```powershell
git fetch origin H!veAI
git rev-list --left-right --count HEAD...origin/H!veAI
```

Fast-forward only if safe. Never reset/rebase/force-push/overwrite user work.

Read before editing:

1. `H!veAI/AGENTS.md`
2. `H!veAI/TASKS.md`
3. `H!veAI/CODEX_ROADMAP.md`
4. `H!veAI/docs/H!veAI/audits/M09D_RETRY_CONTAINMENT_FINAL_STRICT_AUDIT.md`
5. `H!veAI/docs/H!veAI/audits/M09_TASK_INTELLIGENCE_FINAL_CLOSURE_AUDIT.md`
6. `H!veAI/src-tauri/src/git_engine/mod.rs`
7. `H!veAI/src-tauri/src/watcher.rs`
8. `H!veAI/src/components/StartupIntro.tsx`
9. `H!veAI/src-tauri/tauri.conf.json`
10. this prompt

Preserve pre-existing untracked user files.

---

# Canonical UI Assets

Canonical user-owned asset root:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`

Canonical assets:

- `scene 3 starting point.png`
- `videos and gifs\opening video.mp4`
- `H!veAI logo.png`
- `H!veAI small logo.png`

Repository canonical opening video:

`H!veAI/src/assets/opening-video.mp4`

Known canonical opening-video SHA-256 before this hotfix:

`A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`

Do not replace, transcode, edit, normalize, re-export, trim, re-encode, regenerate, or otherwise modify the canonical MP4. The fix is playback/runtime behavior only.

Preserve accepted sidebar logo, post-sidebar background positioning, glass styling, app-shell geometry, startup overlay geometry, footer, routes, and navigation.

---

# TRACKER PREFLIGHT — RECORD M09 CLOSURE TRUTH

Before hotfix implementation, update current tracking documents prospectively so repository truth reflects the independent closure already present on the branch.

At minimum update:

- `H!veAI/TASKS.md`
- `H!veAI/CODEX_ROADMAP.md`
- `H!veAI/README.md`
- `H!veAI/docs/H!veAI/README.md` if its current-status section requires correction

Required current truth:

- M00-M09 = `PASS/CLOSED`.
- M09D independent final strict audit = PASS.
- M09 Task Intelligence Parser final closure = PASS/CLOSED.
- strict completed progress = `10 / 20 = 50%`.
- X01/X02 = active pre-M10 hotfix.
- M10 = `BLOCKED/UNSTARTED` until this hotfix is independently audited and any required manual acceptance is complete.

Do not rewrite historical M09/M09A/M09B/M09C logs or audits.

---

# X01 — STOP VISIBLE WINDOWS GIT CONSOLE WINDOWS

## Current production defect

`H!veAI/src-tauri/src/git_engine/mod.rs` currently spawns Git through the shared production path approximately as:

```rust
Command::new("git")
    .args(args)
    .current_dir(path)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
```

The watcher calls the Git Engine for refresh/snapshot work, so one filesystem event burst can result in several Git child processes. The shared Git child process path does not currently apply the Windows no-console-window creation policy.

## Required production behavior

On Windows, every production Git child process launched through the H!veAI Git Engine MUST use Windows process creation flags that suppress creation of a visible console window.

Preferred implementation:

- use `std::os::windows::process::CommandExt` behind `#[cfg(windows)]`;
- apply Windows `CREATE_NO_WINDOW` (`0x08000000`) to the production Git command before `.spawn()`;
- keep non-Windows behavior unchanged/no-op;
- centralize this in a small helper or command-construction path so future Git Engine calls cannot accidentally omit it.

The fix MUST preserve:

- `git` executable invocation directly, without `cmd.exe`, PowerShell, shell strings, batch wrappers, or browser processes;
- current working directory behavior;
- stdin nulling;
- stdout/stderr piping;
- timeout/kill behavior;
- exit-code/error mapping;
- output bounds;
- safe diff flags;
- read-only/default-denied Git mutation architecture;
- watcher-triggered refresh behavior.

Do not suppress H!veAI's own native window. Only child-process console windows are targeted.

## Production-path proof

Inspect all production Rust code under `H!veAI/src-tauri/src/` for Git process spawning.

PASS only if:

- production Git invocations flow through the corrected shared Git Engine process path, or every separate production Git spawn is equivalently protected;
- no unprotected production `Command::new("git")` path remains that can be reached by H!veAI runtime behavior;
- test-only Git helper commands may remain test-only but must not be confused with production coverage.

## Required direct tests/evidence

Add focused tests around the shared Git process-construction/execution path.

At minimum prove:

1. `git --version`/equivalent bounded production-path command still executes and captures output successfully;
2. an actual temp-repository snapshot still returns branch/status data;
3. Git timeout/error behavior remains intact;
4. watcher-triggered Git refresh integration remains green;
5. Windows-specific no-console configuration is attached to the same production command builder/path used by `run_git()`.

The final source-level audit must be able to see the Windows flag applied before `.spawn()` in the production path. A test name alone is not proof.

## Manual native acceptance required

Builder cannot self-approve this item.

After publication, record `PENDING MANUAL ACCEPTANCE` for the following user test:

- launch the stable native `H!veAI.exe` from the Desktop shortcut;
- keep H!veAI open for at least several minutes;
- cause real changes in one or more registered watched repositories so watcher-driven Git refreshes occur;
- observe that no terminal/console windows flash/open repeatedly;
- confirm H!veAI itself remains responsive and Git/watcher status still updates.

Do not mark X01 fully closed before user acceptance unless direct native evidence conclusively proves the visible-window behavior.

---

# X02 — RESTORE AUDIBLE STARTUP VIDEO

## Current production defect

`H!veAI/src/components/StartupIntro.tsx` currently renders the canonical startup video with:

```tsx
autoPlay
muted
playsInline
```

and also calls `video.play()` programmatically.

Because `muted` is explicit, the canonical MP4 is always silent even when it contains an audio track.

## Required production behavior

Native H!veAI startup must play the canonical opening video WITH its original audio.

Required behavior:

- remove unconditional muted playback;
- before programmatic play, ensure the media element is not muted and uses normal full playback volume unless the OS/app-level audio system says otherwise;
- keep `autoPlay`, `playsInline`, `preload="auto"`, no controls, `onEnded`, `onError`, fixed fullscreen overlay, and current fade/dismiss behavior unless a minimal change is required for reliable audible playback;
- preserve process-scoped startup claim semantics;
- cold native process: intro plays once with audio;
- real native restart/new process: intro plays once with audio;
- SPA navigation within the same process: intro does not replay;
- browser preview remains skip/non-native behavior;
- application mounts immediately behind intro as before;
- frontend-ready remains independent of intro duration.

## Reliable WebView2 audible autoplay

Do not assume removing `muted` alone is sufficient.

Inspect the actual installed Tauri 2 / WRY / WebView2 configuration supported by this repository version and implement a supported native-window policy that allows audible autoplay for the startup intro.

Preferred direction if supported by the current Tauri 2 schema/runtime:

- add the WebView2 autoplay policy equivalent of `--autoplay-policy=no-user-gesture-required` to the main native window through the supported `additionalBrowserArgs`/native builder mechanism.

Safety rules:

- first verify whether the mechanism appends to or replaces existing/default WebView2 arguments in the actual installed Tauri/WRY version;
- do not accidentally drop existing runtime defaults;
- do not disable unrelated security features just to make audio work;
- do not weaken CSP;
- do not enable arbitrary remote origins/network access;
- do not add browser-window fallbacks;
- do not change the user's Windows default browser;
- do not create a standalone Edge window.

If the preferred configuration field is unsupported in the installed version, use the supported Tauri/WRY native mechanism for that exact version and document why.

## Playback failure behavior

An audible autoplay failure must not silently masquerade as success.

Keep the current failsafe/dismiss safety, but add truthful bounded diagnostics where appropriate. Do not introduce a permanent click-through splash redesign unless absolutely required by the runtime after the supported native autoplay policy is attempted.

## Required direct tests/evidence

Add/strengthen frontend tests proving:

1. startup video is not rendered with `muted=true`;
2. when native startup claim resolves to play, the media element is explicitly prepared for audible playback before `play()`;
3. `play()` is attempted for the claimed native intro;
4. ended/error/failsafe behavior still dismisses safely;
5. same-process navigation does not replay the intro;
6. browser preview still skips native intro behavior.

Add config/native verification proving the supported WebView2 audible-autoplay policy is present in the actual production native-window configuration.

Do not claim that jsdom/media mocks prove real audible output. Native audio remains a manual acceptance item.

## Manual native acceptance required

Builder must leave X02 `PENDING MANUAL ACCEPTANCE` after publication.

User acceptance sequence:

1. fully close H!veAI;
2. launch from `Desktop\H!veAI.lnk`;
3. confirm the opening video is visibly playing AND its audio is audible;
4. use the real native `Restart H!veAI` flow;
5. confirm video + audio play again in the new process;
6. navigate between H!veAI routes in the same process and confirm the intro does not replay;
7. confirm no new outer scrollbar/layout regression appears during intro.

Do not mark X02 closed without this manual native evidence.

---

# Scope protection

Allowed production areas are limited to what X01/X02 require, likely:

- `H!veAI/src-tauri/src/git_engine/mod.rs`
- minimal directly related Rust test/support code;
- `H!veAI/src/components/StartupIntro.tsx`
- directly related frontend tests;
- `H!veAI/src-tauri/tauri.conf.json` or the exact supported native-window configuration point;
- task/roadmap/readme tracker updates;
- hotfix log.

Do not touch M09 parser production logic.
Do not implement M10 workflow logic.
Do not redesign the app shell.
Do not modify canonical asset bytes.
Do not create an installer.

If fixing either defect requires broader architecture changes, stop and report instead of expanding scope silently.

---

# Regression gates

Run focused X01/X02 tests first, then:

```powershell
npm run typecheck
npm test -- --run
npm run build
npm audit --audit-level=high
cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check
cargo check --manifest-path H!veAI/src-tauri/Cargo.toml
cargo test --manifest-path H!veAI/src-tauri/Cargo.toml
cargo build --manifest-path H!veAI/src-tauri/Cargo.toml
```

Run:

- existing watcher/Git Engine focused tests;
- publisher failure/rollback harness;
- governed production Tauri `--no-bundle` QA publisher.

Verify after publication:

- stable `H!veAI/dev-bin/H!veAI.exe` exists;
- Desktop shortcut still targets the stable EXE directly;
- stable icon unchanged unless legitimately regenerated from the unchanged canonical small-logo source;
- canonical opening-video SHA-256 remains exactly `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`;
- background/logo hashes unchanged;
- no installer artifacts;
- M09 tests remain green;
- M10 code remains absent.

---

# Required hotfix log

Create:

`H!veAI/docs/H!veAI/codex-logs/PRE_M10_NATIVE_UX_HOTFIX_X01_X02_LOG.md`

Required structure:

```text
X01
Current production cause:
Production symbol(s) changed:
Windows no-console mechanism:
Exact focused test(s):
Watcher/Git regression evidence:
Automated status: PASS / FAIL
Manual native acceptance: PENDING

X02
Current production cause:
Production symbol(s) changed:
Audible autoplay mechanism:
Why this mechanism is supported by the installed Tauri/WRY version:
Exact focused test(s):
Canonical MP4 hash before/after:
Automated status: PASS / FAIL
Manual native acceptance: PENDING

TRACKER TRUTH
M09 final state:
Roadmap progress:
M10 state:

FULL REGRESSION
...

PUBLICATION
stable EXE SHA/size:
shortcut target/icon:
installer scan:

COMMITS / REMOTE
implementation commit:
log commit:
final remote HEAD:
local/origin equality:
```

Record failed attempts chronologically. Do not erase failures after fixing them.

---

# Pre-push self-audit

Before final push, provide in the log for each hotfix:

```text
X0N
Production defect that existed before this hotfix:
Exact production symbol that now changes behavior:
Exact test(s):
Why the old code would fail the test / requirement:
What automated evidence does NOT prove:
Manual acceptance still required:
```

Also explicitly verify:

- no M10 code;
- no M09 production changes;
- no canonical asset byte changes;
- no installer;
- no unrelated UI redesign;
- no shell-wrapper workaround for Git;
- no fake audible-playback claim from browser/jsdom tests alone.

---

# Stop condition

Stop after and only after:

1. X01 production fix is implemented and automated Git/watcher regression is green;
2. X02 production fix is implemented and automated frontend/native-config regression is green;
3. M09 closure truth is reflected in tracking documents;
4. full regression/security/build gates pass;
5. governed no-bundle QA publication succeeds;
6. canonical MP4 and other canonical asset hashes remain unchanged;
7. hotfix log is committed and pushed;
8. final local HEAD == `origin/H!veAI` is verified after the final pushed log commit;
9. X01/X02 remain `PENDING MANUAL ACCEPTANCE` unless the user has separately supplied native acceptance evidence;
10. M10 remains `BLOCKED/UNSTARTED`.

Then stop and wait for independent audit + user native acceptance.