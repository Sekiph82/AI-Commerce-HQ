# H!veAI MP4 Startup and Native Icon Fix

Date: 2026-08-27
Branch: `H!veAI`

The canonical startup asset is now `H!veAI/src/assets/H!veAI.mp4`. The previous production reference was `../assets/opening-video.mp4`; `opening-video.mp4` is no longer referenced by `StartupIntro.tsx`.

## Media evidence

Original `H!veAI/src/assets/H!veAI.mp4` before compatibility conversion:

- Container: MP4 / QuickTime MOV
- Video: HEVC Main 10, `yuv420p10le`, 1918x1080, 24 fps
- Audio: AAC LC, 32000 Hz, stereo
- Duration: 8.064 seconds
- SHA-256: `DFAC3F85BD8119E5A59D8A9B5F9E8B8749A8B03345FE8455514E0D505A326E38`

The HEVC Main 10 / 10-bit stream was unsuitable for reliable Windows Tauri/WebView2 HTML5 video rendering, explaining audio with a black frame. It was transcoded in place to preserve the canonical filename/path:

- Container: MP4 / QuickTime MOV
- Video: H.264 AVC High, `yuv420p`, 1918x1080, 24 fps
- Audio: AAC LC, 32000 Hz, stereo
- Duration: 8.064 seconds
- Final SHA-256: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`

The production bundle emitted `dist/assets/H!veAI-Cfo21PSp.mp4` with the same final SHA-256 and the same compatible streams.

## Icon evidence

The approved `H!veAI small logo.png` was used without alteration as the icon source. Both `H!veAI/src-tauri/icons/icon.ico` and `H!veAI/dev-bin/H!veAI.ico` now contain square `16x16`, `32x32`, `48x48`, `64x64`, `128x128`, and `256x256` entries and are byte-identical.

- Old `src-tauri/icons/icon.ico` SHA-256: `119CED8B2B393C7785809E2F47045EE44FBE9FA3E65490FA86475E97DD83E923`
- Old `dev-bin/H!veAI.ico` SHA-256: `D83ED52300040617D1DA2502E35DC25FEC66AF030CDF444DD52B491716B0940E`
- New `src-tauri/icons/icon.ico` SHA-256: `F0C1CC62F000C959AB10493B902E1A7B2F4E10E1E4BB9C837BF3841BC194C5AD`
- New `dev-bin/H!veAI.ico` SHA-256: `F0C1CC62F000C959AB10493B902E1A7B2F4E10E1E4BB9C837BF3841BC194C5AD`

Tauri explicitly configures `icons/icon.ico`. The published executable's associated icon extracted as `32x32`, the launch window title was `H!veAI`, and the desktop shortcut points to `dev-bin/H!veAI.exe` with `dev-bin/H!veAI.ico,0`.

## Verification

- `ffprobe.exe` original/final MP4 inspection -> completed; codec and stream metadata recorded above.
- `npm.cmd test -- --run --reporter=dot tests/m08.00-focused.test.tsx tests/pre-m10-native-ux-focused.test.tsx` -> 2 files, 15 passed.
- `cargo test --lib startup_intro_tests -- --nocapture --test-threads=1` -> 4 passed.
- `npm.cmd run typecheck` -> passed.
- `npm.cmd run build` -> passed.
- `powershell.exe -ExecutionPolicy Bypass -File .\scripts\publish-dev-qa.ps1` -> passed; production Tauri `--no-bundle` build, candidate/stable smoke tests, shortcut validation, and console suppression passed.
- Stable executable launch check -> passed technically: startup captures at two times differed, visible video frames rendered, audio-capable unmuted playback path remained configured, window title was `H!veAI`, and no new visible console host appeared.
- Existing startup transition, fullscreen/no-controls behavior, and same-process replay claim logic were preserved.

## Git evidence

Implementation commit: `85dc71e922cf1e8fb7f9c4166a183c66abcd47ca`
Exact post-push implementation HEAD: `85dc71e922cf1e8fb7f9c4166a183c66abcd47ca`
Exact fetched `origin/H!veAI`: `85dc71e922cf1e8fb7f9c4166a183c66abcd47ca`
Exact `HEAD...origin/H!veAI` count: `0 0`

Implementation files changed:

- `H!veAI/src/assets/H!veAI.mp4`
- `H!veAI/src/components/StartupIntro.tsx`
- `H!veAI/tests/m08.00-focused.test.tsx`
- `H!veAI/src-tauri/icons/icon.ico`
- `H!veAI/src-tauri/tauri.conf.json`
- `H!veAI/dev-bin/H!veAI.ico`

No M11A REV7 identity code, Command Center, Task Sources, external registered project, Bulk Edit, M12, or M21 files were changed by this corrective implementation. The pre-existing uncommitted M11A worktree changes were preserved and left unstaged.

Final state: `H!VEAI.MP4 STARTUP + NATIVE ICON FIX COMPLETE / PENDING USER VISUAL-AUDIO ACCEPTANCE`
