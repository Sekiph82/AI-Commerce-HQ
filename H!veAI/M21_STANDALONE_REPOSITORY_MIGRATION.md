# M21 - Standalone Repository Migration

Purpose: after H!veAI v1.0 product development, hardening, release acceptance, and all prior roadmap work are complete, separate H!veAI from the `AI-Commerce-HQ` parent repository and make H!veAI an independent repository/product root.

Timing rule: this milestone is intentionally last. Do not begin it before M20 is PASS/CLOSED and the H!veAI v1.0 release/hardening gates are satisfied. Until then, H!veAI remains developed as the `H!veAI` branch/subtree of `Sekiph82/AI-Commerce-HQ` so reusable AI-Commerce-HQ modules can still be evaluated and reused safely.

## Packages

- M21.01 Verify all prior H!veAI milestones and M20 release/hardening gates are PASS/CLOSED.
- M21.02 Verify the `H!veAI` branch is clean, synchronized, backed up, and tagged with a pre-migration reference point.
- M21.03 Create a dedicated standalone H!veAI GitHub repository.
- M21.04 Promote the current `AI-Commerce-HQ/H!veAI` application tree to the new repository root.
- M21.05 Preserve relevant Git history where practical and document any history filtering/splitting performed.
- M21.06 Establish the standalone repository's production branch strategy, including `main`.
- M21.07 Point `origin` to the standalone H!veAI repository and remove obsolete parent-repository coupling from H!veAI.
- M21.08 Rebase all repo-relative paths, build scripts, package paths, Tauri configuration, native/Rust paths, tests, documentation, and automation on the new root.
- M21.09 Move/repair GitHub Actions, CI/CD, release configuration, issue/PR templates, tags, and release automation as applicable.
- M21.10 Update Codex/Claude/agent instructions and repository-root assumptions for standalone operation.
- M21.11 Revalidate `.hiveai` project-intelligence boundaries and self-project behavior from the standalone root.
- M21.12 Register AI-Commerce-HQ and H!veAI as two separate projects/repositories in H!veAI and verify identity/source isolation.
- M21.13 Confirm AI-Commerce-HQ no longer discovers or treats H!veAI internal files such as `H!veAI/TASKS.md` as AI-Commerce-HQ canonical task authority merely because H!veAI lives beneath the parent repository.
- M21.14 Run full clean build, automated regression, Rust/native tests, frontend tests, packaging, and Windows clean-machine smoke tests from the standalone repository.
- M21.15 Perform final Git/remote/history/release/security/documentation audit.
- M21.16 Preserve or archive the old `H!veAI` branch in AI-Commerce-HQ with an explicit migration tag/reference rather than deleting history blindly.
- M21.17 Record migration commit/tag/repository identity and close the migration only after independent audit and user acceptance.

## Exit criteria

M21 is complete only when H!veAI is no longer dependent on being a subdirectory or development branch of `AI-Commerce-HQ`; it has its own repository root, remote, branch strategy, build/test/release system, documentation, project-intelligence boundaries, and verified standalone Windows build/release path.

## Known temporary condition before M21

While H!veAI remains under the AI-Commerce-HQ parent repository, AI-Commerce-HQ source discovery can surface nested H!veAI planning files such as `H!veAI/TASKS.md`. This is a known consequence of the current development topology and is not, by itself, a reason to split the repository early. The structural separation is intentionally deferred to M21 after M20.

Status: PLANNED/BLOCKED behind M20.
