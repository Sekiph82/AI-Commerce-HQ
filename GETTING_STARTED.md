# AI Commerce HQ — Getting Started

## Prerequisites

Install these before anything else:

| Tool | Version | Download |
|------|---------|----------|
| Node.js | 18+ | https://nodejs.org |
| Python | 3.11+ | https://python.org |
| Rust + Cargo | latest | https://rustup.rs |

## Quick Start (Development)

### Option A — One command
```powershell
.\scripts\dev.ps1
```

### Option B — Manual (two terminals)

**Terminal 1 — Python backend:**
```powershell
cd backend
pip install -r requirements.txt
python main.py
```

**Terminal 2 — Tauri frontend:**
```powershell
npm install
npm run tauri:dev
```

## First Run

1. App opens with splash screen
2. Setup wizard asks for API keys:
   - **OpenAI API key** — required for AI reasoning and image generation
   - **Etsy API key + Shop ID** — for real Etsy listings (optional, simulated without)
   - **Printify token** — for POD fulfillment (optional, simulated without)
   - **Gemini key** — optional enhancement
3. Office view opens automatically
4. Agents begin working within ~30 seconds

> **Demo mode:** The app works without any API keys using realistic simulated data.

## Build Windows Installer

```powershell
.\scripts\build-windows.ps1
```

Output: `src-tauri/target/release/bundle/msi/AI Commerce HQ_1.0.0_x64_en-US.msi`

## Architecture

```
AI Commerce HQ
├── Tauri (Rust)          — Desktop shell, launches Python backend
├── React + TypeScript    — Office UI, real-time visualization
├── Python + FastAPI      — AI orchestration runtime
│   ├── GMO               — Global Master Orchestrator
│   ├── ETMO              — Etsy Master Orchestrator
│   ├── Sub-Agents        — TRD, DES, QA, POD, LST (dynamically created)
│   └── SQLite            — Local persistent storage
└── WebSocket             — Real-time agent → UI updates
```

## How It Works

1. App launches → Tauri starts Python backend automatically
2. GMO initializes → Creates ETMO
3. ETMO creates sub-agent team (visible as desks appearing in Etsy room)
4. Every ~60 seconds, ETMO runs a pipeline cycle:
   - TRD discovers a trending niche
   - DES generates a product design (DALL-E if key present)
   - QA validates the design
   - POD configures Printify product
   - LST creates Etsy draft listing
   - Approval packet created → notification appears
5. You open the Approval Desk and review the product
6. You click **Publish** (goes live) / **Keep Draft** / **Discard**

## Rooms

| Room | Status | Description |
|------|--------|-------------|
| GMO | Active | Global control center |
| Etsy | Active | Full pipeline running |
| Amazon | Dormant | Future release |
| eBay | Dormant | Future release |
| TikTok | Dormant | Future release |
| Instagram/Facebook | Dormant | Future release |
| Website Store | Dormant | Future release |

## Data Storage

All data stored locally at:
```
%APPDATA%\ai-commerce-hq\hq.db    (Windows)
~/.ai-commerce-hq/hq.db           (fallback)
```

## Known Limitations (v1)

- Only Etsy platform is operational
- Image generation requires OpenAI API key (DALL-E 3)
- Etsy OAuth flow not implemented — uses API key auth only
- Printify integration is simulated without token
- No email notifications
- Windows only (macOS/Linux support requires minor config changes)
