# M05 Project Registry

## Scope

M05 adds an explicit, local-first Project Registry on top of the M04 Rust SQLite layer. A user supplies a folder path through the H!veAI UI; H!veAI reads identity and Git metadata without mutating that folder.

## Registration flow

1. The user selects **Add project** in the `/projects` registry view.
2. The user provides a folder path and optional display name.
3. H!veAI verifies that the path exists and is a directory, canonicalizes it, detects Git metadata, sanitizes remotes, and checks normalized-path duplicates.
4. H!veAI transactionally writes one `projects` row and, when applicable, one `repositories` row.
5. The persisted record appears in search, sort, status filters, and Project Cockpit.

H!veAI never recursively scans Desktop, Documents, home directories, or arbitrary disks. Registration does not run project code, install packages, edit files, create `.hiveai/`, modify `.git/config`, change remotes, checkout branches, create worktrees, commit, or push.

## Schema usage

Migration version 3 adds registry fields to M04 `projects` and `repositories` tables. Project records store a stable UUID, display path, canonical normalized path, registration and validation timestamps, status, priority, preferred builder/auditor, and task-source policy. Repository records store read-only repository root, Git flag, branch, HEAD, sanitized remote list, default branch, and GitHub owner/repository identity.

The M04 SQLite database remains the only datastore. No secrets, credential-bearing URLs, tokens, private keys, `.env` content, or raw credentials are persisted.

## Path normalization and duplicates

The selected folder must exist and be a directory. `std::fs::canonicalize` resolves the path for identity; separators are normalized and Windows comparison is case-insensitive. The original user-selected string remains available for display/audit. A normalized path duplicate is rejected deterministically. The path is never used as a primary key.

Missing paths are represented as `MISSING` at read time. This does not automatically repair or rewrite the project record. Repair requires an explicit user action and a new valid folder.

## Git metadata detection

Detection walks only the selected folder and its ancestors until a `.git` marker is found. It reads `HEAD`, refs/packed refs, and `.git/config` without issuing Git commands or mutating metadata. It tolerates non-Git folders, detached HEAD, absent remotes, multiple remotes, malformed URLs, and missing refs. `origin` is preferred when present; otherwise the first remote is used.

Supported GitHub URL forms include HTTPS, SSH, and SCP-style remotes. HTTP credentials are removed before storage or display. GitHub owner/repository identity is derived only from sanitized supported URLs.

## Archive, remove, and repair

- **Archive:** sets registry status to `ARCHIVED` and retains historical metadata. It never touches the project folder.
- **Remove from registry:** deletes H!veAI project/repository rows only. It never deletes, moves, or edits the project folder.
- **Repair path:** requires an explicit path, validates the destination, rejects duplicate normalized paths, compares an existing remote identity when available, and updates registry rows only. It never moves content.

## UI integration and Canonical UI Assets

The `/projects` surface uses real persisted records with search, sort, status filters, priority control, Git/non-Git state, branch/remote summaries, builder/auditor settings, missing/archived states, cockpit navigation, archive, removal, and repair controls. The dark layout follows the canonical dashboard’s dense cards, navigation, hierarchy, and right-side status column.

The canonical H!veAI logo and Akilta wordmark were copied unchanged from `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI` into the child app’s `src/assets/` directory. The shell displays H!veAI branding and the footer text exactly: `Built with ♥ for maximum productivity by Akilta`.

## Security and containment

IPC is typed and allowlisted. There is no arbitrary filesystem browser, shell, Git command, SQL executor, or process surface. Automated tests use temporary folders and repositories only. The parent application and managed external projects are not modified.

## M06 boundary

M06 owns live Git status, diffs, staged/unstaged files, commits, remotes, ahead/behind state, worktrees, and safe Git writes. M05 stores only the read-only metadata captured at registration time and labels it as cached metadata in Project Cockpit.
