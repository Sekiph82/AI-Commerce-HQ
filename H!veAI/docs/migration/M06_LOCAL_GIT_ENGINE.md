# M06 Local Git Engine

## Technology choice

M06 uses a Rust-owned, narrowly executed Git CLI boundary. `std::process::Command` invokes the fixed `git` executable with structured argument arrays, a fixed repository working directory, null stdin, an eight-second timeout, and bounded output. No shell, command string, executable path, or arbitrary subcommand reaches the IPC surface. This approach preserves compatibility with ordinary Windows repositories, worktrees, remotes, upstream tracking, status fidelity, and later agent worktree integration without adding a native library compatibility burden.

## Read/write separation

`src-tauri/src/git_engine/mod.rs` contains read-only snapshot, diff, commit, remote, and worktree inspection. `mutation.rs` is a separate write module. Its gate defaults to denied, validates branch names and repository-relative paths, disallows force/reset/checkout-discard/stash operations, and is not exposed as an active UI workflow in M06.

## Snapshot and status model

Snapshots resolve a registered M05 project ID to its persisted repository ID and normalized path. They return branch or detached HEAD, HEAD SHA, staged/unstaged/untracked/conflicted files, repository health, remotes, bounded recent commits, worktrees, upstream, and ahead/behind counts. Health is `CLEAN`, `DIRTY`, `CONFLICTED`, `DETACHED`, or `UNBORN`; missing and non-Git registrations return structured safe errors. Status is parsed from `git status --porcelain=v1 -z --branch`, so clean state is not inferred from one category alone.

Snapshots are returned live. SQLite `git_snapshots` rows are written only when an IPC request explicitly sets `persist: true`. Raw diffs are never stored.

## Diffs and limits

Diff scope is explicit: `STAGED` or `WORKING_TREE`. Output is capped at 96 KiB and 1,200 lines, with a truncation flag. Binary files are represented as metadata and are not exposed as raw bytes. Paths are passed only after registry resolution and as data after `--`.

## Remotes and network behavior

Remote URLs are sanitized before response, persistence, or logging; embedded credentials are removed. Ordinary snapshots do not fetch, pull, push, or contact a remote. Ahead/behind is compared only against an already configured upstream. Without one, counts are `null` and the UI says unavailable.

## Mutation test strategy

Mutation tests create temporary repositories only. They cover default denial, branch creation, explicit-path staging, commit behavior, dangerous path/branch/remote rejection, and isolation. A local bare remote is reserved for explicit push tests; no GitHub or user repository is used by tests.

## Windows limitations and future boundaries

Git must be installed and available as `git.exe` on the desktop process PATH. Large repositories may hit bounded output or timeout errors and report a structured failure. M07 may add filesystem-triggered refreshes, but M06 has no polling loop. M13/M16 can consume snapshots and bounded diffs for agent/audit context. M18 can add separately permission-gated remote workflows; it must not broaden this engine into an unrestricted shell.
