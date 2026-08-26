# Opening Video Replacement Log

Date: 2026-08-27
Branch: `H!veAI`

Source video: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI\videos and gifs\H!veAI.mp4`
Destination tracked asset: `H!veAI/src/assets/opening-video.mp4`

Old SHA-256: `A438404A19CE53C45D1385BA1F1009E9AEA110C7361C42B278844EBCF76C6686`
New source SHA-256: `DFAC3F85BD8119E5A59D8A9B5F9E8B8749A8B03345FE8455514E0D505A326E38`
Resulting tracked SHA-256: `DFAC3F85BD8119E5A59D8A9B5F9E8B8749A8B03345FE8455514E0D505A326E38`
Proof: source, tracked asset, and production bundle `dist/assets/opening-video-CJnzggXz.mp4` hashes match exactly.

Verification:

- `cargo test --lib startup_intro_tests -- --nocapture --test-threads=1` -> 4 passed.
- `npm.cmd test -- --run --reporter=dot tests/m08.00-focused.test.tsx tests/pre-m10-native-ux-focused.test.tsx` -> 2 files, 14 passed.
- `npm.cmd run typecheck` -> passed.
- `npm.cmd run build` -> passed.
- `npm.cmd audit --audit-level=high` -> 0 vulnerabilities.
- `cargo fmt --all -- --check` and `cargo check` -> passed.
- `powershell.exe -ExecutionPolicy Bypass -File .\scripts\publish-dev-qa.ps1` -> passed; stable `H!veAI.exe` published and shortcut validation passed.

Implementation commit: `d0bc7323955d910a630cf3a5a8b7d3a51c84e86c`
Post-push local SHA: `d0bc7323955d910a630cf3a5a8b7d3a51c84e86c`
Post-push origin SHA: `d0bc7323955d910a630cf3a5a8b7d3a51c84e86c`
Post-push divergence: `0 0`

Startup playback logic, audible playback, single-process replay suppression, transition behavior, UI layout, and unrelated assets were unchanged. No external registered project, Bulk Edit, M12, M21, or M11A REV7 source was changed.

Final state: `OPENING VIDEO REPLACEMENT COMPLETE / PENDING USER NATIVE VISUAL-AUDIO ACCEPTANCE`
