# AI Commerce HQ — Getting Started

## For End Users

1. Run the installer: **`AI Commerce HQ_1.0.0_x64-setup.exe`**
2. Follow the one-click installer (no admin rights required)
3. A desktop icon is created automatically
4. Double-click the icon to launch
5. The app opens a setup wizard on first run — enter your API keys
6. The office opens and agents start working

**You never need to open a terminal.**

---

## For Developers — Building the Installer

### Prerequisites

| Tool | Version | Download |
|------|---------|----------|
| Python | 3.11–3.14 | https://python.org |
| Node.js | 18+ | https://nodejs.org |
| Rust + Cargo | latest | https://rustup.rs |

### Build the installer (one command)

```
python build-installer.py
```

This script:
1. Installs Python dependencies
2. Compiles the Python backend to a standalone `backend.exe` (via PyInstaller, ~2–4 min)
3. Installs Node.js dependencies
4. Runs `tauri build` to produce the NSIS installer

Output: `src-tauri/target/release/bundle/nsis/AI Commerce HQ_1.0.0_x64-setup.exe`

### Development mode (live reload, no installer)

```
python dev.py
```

Or double-click `start-dev.bat` on Windows.

This starts the Python backend and Tauri dev server in one step.

---

## First Run (End User Experience)

1. Splash screen appears while the app initializes
2. Setup wizard asks for API keys:
   - **OpenAI API key** — required for AI reasoning and image generation
   - **Etsy API key + Shop ID** — for real Etsy listings (simulated without)
   - **Printify token** — for POD fulfillment (simulated without)
   - **Gemini key** — optional enhancement
3. Office view opens automatically
4. Agents begin working within ~30 seconds

> **Demo mode:** The app runs without any API keys using realistic simulated data.

---

## How It Works

```
AI Commerce HQ
├── Tauri (Rust)          — Desktop shell, launches backend.exe automatically
├── React + TypeScript    — Office UI, real-time agent visualization
├── backend.exe           — Self-contained Python runtime (no Python needed by user)
│   ├── FastAPI           — REST API + WebSocket server on localhost:8765
│   ├── GMO               — Global Master Orchestrator
│   ├── ETMO              — Etsy Master Orchestrator
│   ├── Sub-Agents        — TRD, DES, QA, POD, LST (dynamically created)
│   └── SQLite            — Local persistent storage
└── WebSocket             — Real-time agent → UI updates
```

**Pipeline (runs every 60 seconds):**

1. TRD discovers a trending niche via AI research
2. DES generates a product design concept (DALL-E 3 if key present)
3. QA validates for copyright safety and Etsy compliance
4. POD configures print-on-demand product on Printify
5. LST creates SEO-optimized Etsy draft listing
6. Approval packet assembled → notification badge appears on Approval Desk
7. **You** open the Approval Desk, review, and click Approve / Reject

---

## Office Rooms

| Room | Status | Description |
|------|--------|-------------|
| Global Master Orchestrator | Active | System command center |
| Etsy Operations | **Active** | Full AI pipeline running |
| Amazon Operations | Dormant | Future release |
| eBay Operations | Dormant | Future release |
| TikTok Shop | Dormant | Future release |
| Instagram / Facebook | Dormant | Future release |
| Website Store | Dormant | Future release |

---

## Data Storage

All data is stored locally — nothing leaves your machine without your approval.

```
%APPDATA%\ai-commerce-hq\hq.db   (Windows)
~/.ai-commerce-hq/hq.db          (fallback)
```

---

## Known Limitations (v1)

- Only Etsy platform is operational
- DALL-E image generation requires an OpenAI API key
- Etsy OAuth not implemented — uses API key auth only
- No push notifications outside the app
