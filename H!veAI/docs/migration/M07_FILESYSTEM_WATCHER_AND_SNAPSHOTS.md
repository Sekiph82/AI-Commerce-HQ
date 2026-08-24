# M07 Filesystem Watcher and Snapshots

## Technology and lifecycle

M07 uses the Rust `notify` crate with the Windows `RecommendedWatcher`. One H!veAI `WatcherManager` owns project watchers and a named worker thread. Startup loads active projects from the M05 registry after SQLite initialization. A single unavailable or failed root becomes `MISSING` or `DEGRADED`; it does not prevent other watchers or H!veAI startup. Dropping the manager stops watcher resources and joins the worker.

## Scope and exclusions

Watch roots are taken only from active registered project records. No arbitrary frontend path is accepted. Paths are normalized relative to the registered root before event handling or frontend payloads. Default noisy paths include `.git/objects`, `.git/logs`, `node_modules`, `target`, `dist`, `build`, `.next`, cache/temp directories. `.git/HEAD`, refs, and config remain eligible Git metadata hints. M07 never reads changed file contents.

## Event model

Raw notify events become bounded internal `NormalizedEvent` values with project ID, event ID, `CREATE`, `MODIFY`, `REMOVE`, `RENAME`, or `RESCAN_REQUIRED`, relative path, optional old path, timestamp, source, and category hint. Hints distinguish Git metadata, task candidates, source, config, and other files; M07 does not parse task files.

## Debounce, queue, and overflow

The watcher callback sends through a bounded `sync_channel` of 512 inputs. A per-project/path pending map coalesces rapid changes with a 250 ms debounce window. Refreshes are rate-limited to one per project per 750 ms. Queue overflow, notify errors, or an overfull pending map mark the project `DEGRADED` with `OVERFLOW` health and `rescanRequired=true`; correctness is recovered through explicit rescan rather than silent dropping.

## Snapshot refresh and persistence

Accepted changes update project availability and evidence timestamps. Git metadata events trigger the M06 Git snapshot with explicit SQLite persistence; non-Git changes still create bounded project evidence. M07 adds migration v4 and `project_snapshots`, storing availability, optional Git snapshot identity, filesystem/refresh/evidence timestamps, changed-path count, rescan flag, and watcher health. It does not store raw file contents or automatically persist full diffs.

## Missing and repaired projects

When a registered root disappears, the watcher is removed, the registry row remains, and the project is represented as `MISSING`. Explicit M05 path repair followed by watcher-set refresh or project rescan reattaches a watcher to the repaired registered root. M07 never searches the machine for replacements and never deletes registry data.

## IPC and privacy boundary

IPC exposes watcher summary, project watcher status, refresh watcher set, and project rescan by project ID only. It does not expose arbitrary watch paths, arbitrary file reads, generic OS event streams, network actions, project execution, package installation, or mutation. Errors are returned as bounded safe strings; file contents and secrets are not logged.

## Large repositories and future boundaries

Generated-directory exclusions, bounded queue/pending state, debounce, refresh throttling, and overflow-to-rescan semantics protect large or noisy repositories. Very large trees, network drives, symlink-heavy layouts, and platform-specific notify behavior may require later tuning. M08 may consume evidence and task-candidate hints, but task discovery and parsing remain outside M07. M06 Git mutation remains default-denied.
