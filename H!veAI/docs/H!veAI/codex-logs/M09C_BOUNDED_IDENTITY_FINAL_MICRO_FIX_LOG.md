# M09C Bounded Identity Final Micro-Fix Log

Date: 2026-08-25
Product: H!veAI
Branch: H!veAI

## Start evidence

- Mandatory `git fetch origin H!veAI` completed.
- `git rev-list --left-right --count HEAD...origin/H!veAI` was `0 6` before safe fast-forward.
- Synchronized starting HEAD: `fe53568`.
- Pre-existing untracked user files `start-demo.bat` and `task.md` were preserved.
- No UI, canonical asset, installer, M10, X01, or X02 changes were made.

R02C
Production symbol(s): `duplicate_identity_key`, `identity_digest_bytes`, `update_normalized_text`, `task_id`, and the fixed-size `HashMap<[u8; 32], usize>` in `parse_document`.
Exact test(s): `r02c_duplicate_identity_key_is_fixed_size_for_oversized_heading`, `r02c_task_ids_remain_stable_after_identity_streaming_refactor`, `r02c_large_heading_many_tasks_remains_deterministic`.
Why f919fb66 fails: M09B retained raw heading context in `HashMap<String, usize>` duplicate keys and built large formatted task identity strings before hashing.
Fixed-size working identity representation: duplicate ordinal keys are `[u8; 32]` SHA-256 digests; normalized identity components are streamed into SHA-256 and no giant formatted source-derived identity string is retained.
Task-ID stability proof: the representative explicit and fallback test compares the new digest output with the prior M09B logical concatenation digest; ordinary task IDs remain unchanged.
Status: PASS

E01C
Exact retry-containment evidence: `p01_retry_rechecks_physical_containment` mutates the refreshed M08 source row's relative path to `../outside.md` through a private `cfg(test)` substitution immediately after rediscovery and before refreshed canonicalization; the real `read_authoritative_source()` path returns `SOURCE_READ_FAILED` from the refreshed root-containment check.
Status: PASS

E03C
Exact stale/retained/legacy/settings/dependency SQL evidence: `p07_removed_task_and_source_reconcile_only_stale_m09_rows` retains two M09 tasks and one dependency edge, removes a configured stale source/task, seeds and verifies unchanged legacy source/task/settings rows, verifies retained `TASKS.md` source, and asserts `SOURCE_EXPLICIT` count equals its distinct-edge count `(1,1)`.
Status: PASS

## Focused evidence

- 53 `task_intelligence::tests` passed, including all prior M09A/M09B tests and the three required R02C tests.
- `p01_retry_rechecks_physical_containment`: PASS.
- `p07_removed_task_and_source_reconcile_only_stale_m09_rows`: PASS.
- No visible UI production files changed.

## Regression and security gates

- `npm run typecheck`: PASS.
- `npm test -- --run`: PASS, 5 files / 70 tests.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS, 0 vulnerabilities.
- `cargo fmt --manifest-path H!veAI/src-tauri/Cargo.toml -- --check`: PASS.
- `cargo check --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- `cargo test --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS, 190 tests passed, 0 failed.
- `cargo build --manifest-path H!veAI/src-tauri/Cargo.toml`: PASS.
- Publisher failure harness: PASS, all 9 governed failure/rollback assertions.
- Governed production publisher: PASS, production `--no-bundle`, candidate/stable smoke, frontend-ready marker, no forbidden dev ports, and no visible console host.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`, SHA-256 `3DE0029C908876CEEB377ABC9FC7F2EB335EF57B50FB432C9E3AB05E3929C430`, size `17715200` bytes.
- Shortcut target: `H!veAI/dev-bin/H!veAI.exe`; icon: `H!veAI/dev-bin/H!veAI.ico,0`.

## Truth and scope

- Canonical asset hashes remain unchanged: background `7997ADD4EE7417B5818C1E2B7789B4C20D7B5F7EEC3215B9C0A136A5B9791C23`; opening video `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`; H!veAI logo `C773839E949222ED787972964AB3EEF27DF0F7D885AF78E7A0E6E340EF6E726C`.
- No installer was created.
- X01 terminal-popup and X02 startup-audio remain intentionally deferred.
- M09 remains open pending independent M09C re-audit; M10 remains BLOCKED/UNSTARTED.

## Commit/publication evidence

- Implementation commit: `63b73795dcc781f181b21b1cc02199c67f5565f1`.
- Stable executable SHA/size/shortcut: recorded above after governed publication.
- Final pushed M09C log commit: `SELF / verified after push in session`.
- Final remote branch HEAD: recorded in terminal/session evidence and final response.
- Local/origin equality: verified after the final pushed log commit without force-push.

Stop after M09C. Do not begin M10 or the X01/X02 hotfixes.
