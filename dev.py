#!/usr/bin/env python3
"""
AI Commerce HQ — Development Launcher
======================================
Single-command development startup.

Usage:
    python dev.py           # install deps + start dev server
    python dev.py --check   # check environment only
    python dev.py --backend # start Python backend only
"""

import sys
import os
import subprocess
import platform
import time
import shutil
import argparse
from pathlib import Path

ROOT = Path(__file__).parent
BACKEND = ROOT / "backend"
MIN_PYTHON = (3, 11)
BACKEND_PORT = 8765

IS_WINDOWS = platform.system() == "Windows"

# ── On Windows, npm/npx are .cmd files and need shell=True ───────────────────
def run_cmd(args, **kwargs):
    """Run a command, using shell=True on Windows so .cmd files work."""
    return subprocess.run(args, shell=IS_WINDOWS, **kwargs)

def popen_cmd(args, **kwargs):
    """Popen a command, using shell=True on Windows so .cmd files work."""
    return subprocess.Popen(args, shell=IS_WINDOWS, **kwargs)

# ── Colour helpers ────────────────────────────────────────────────────────────
def ok(msg):   print(f"  OK  {msg}")
def warn(msg): print(f"  !   {msg}")
def err(msg):  print(f"  ERR {msg}")
def info(msg): print(f"  --> {msg}")

# ── Environment checks ────────────────────────────────────────────────────────

def check_python_version():
    v = sys.version_info
    if v < MIN_PYTHON:
        err(f"Python {v.major}.{v.minor} is too old. Need {MIN_PYTHON[0]}.{MIN_PYTHON[1]}+")
        sys.exit(1)
    ok(f"Python {v.major}.{v.minor}.{v.micro}")

def check_node():
    if shutil.which("node"):
        r = run_cmd(["node", "--version"], capture_output=True, text=True)
        ok(f"Node.js {r.stdout.strip()}")
    else:
        err("Node.js not found. Install from https://nodejs.org")
        sys.exit(1)

def check_npm():
    # npm is npm.cmd on Windows — find the real path first
    npm_path = shutil.which("npm") or shutil.which("npm.cmd")
    if npm_path:
        r = run_cmd(["npm", "--version"], capture_output=True, text=True)
        ok(f"npm {r.stdout.strip()}")
    else:
        err("npm not found. Reinstall Node.js from https://nodejs.org")
        sys.exit(1)

def check_cargo():
    if shutil.which("cargo"):
        r = run_cmd(["cargo", "--version"], capture_output=True, text=True)
        ok(f"Cargo: {r.stdout.strip()}")
    else:
        warn("Rust/Cargo not found — needed to compile the app. Install from https://rustup.rs")
        warn("After installing Rust, close this window and open a new one.")

def check_environment():
    print("\n=== AI Commerce HQ — Environment Check ===")
    check_python_version()
    check_node()
    check_npm()
    check_cargo()
    print()

# ── Dependency installation ───────────────────────────────────────────────────

def install_python_deps():
    req = BACKEND / "requirements.txt"
    if not req.exists():
        warn("backend/requirements.txt not found — skipping")
        return
    info("Installing Python backend dependencies...")
    subprocess.run(
        [sys.executable, "-m", "pip", "install", "-r", str(req), "--quiet"],
        check=True
    )
    ok("Python dependencies installed")

def install_node_deps():
    if (ROOT / "node_modules").exists():
        ok("Node modules already installed")
        return
    info("Installing Node.js dependencies (this may take a minute)...")
    run_cmd(["npm", "install", "--silent"], cwd=ROOT, check=True)
    ok("Node.js dependencies installed")

# ── Backend startup ───────────────────────────────────────────────────────────

def backend_ready(timeout=30) -> bool:
    import urllib.request
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            urllib.request.urlopen(
                f"http://127.0.0.1:{BACKEND_PORT}/api/health", timeout=1
            )
            return True
        except Exception:
            time.sleep(1)
    return False

def start_backend() -> subprocess.Popen:
    info("Starting Python backend...")
    env = os.environ.copy()
    data_dir = Path.home() / "AppData" / "Roaming" / "ai-commerce-hq"
    env["HQ_DATA_DIR"] = str(data_dir)

    # Keep console visible on Windows so errors are readable in dev mode
    proc = subprocess.Popen(
        [sys.executable, str(BACKEND / "main.py")],
        cwd=str(BACKEND),
        env=env,
    )
    return proc

# ── Main launch ───────────────────────────────────────────────────────────────

def run_dev():
    check_environment()

    try:
        install_python_deps()
    except subprocess.CalledProcessError:
        warn("Python dep install had errors — attempting to continue")

    try:
        install_node_deps()
    except subprocess.CalledProcessError:
        err("npm install failed.")
        input("Press Enter to exit...")
        sys.exit(1)

    backend_proc = start_backend()
    info(f"Waiting for backend on port {BACKEND_PORT}...")
    if backend_ready(timeout=30):
        ok(f"Backend ready at http://127.0.0.1:{BACKEND_PORT}")
    else:
        warn("Backend did not respond in 30s — app will retry automatically")

    info("Starting app window (first launch compiles — this takes 10-20 min)...")
    print("  Press Ctrl+C to stop\n")

    try:
        run_cmd(["npm", "run", "tauri:dev"], cwd=ROOT, check=True)
    except KeyboardInterrupt:
        print()
    finally:
        info("Shutting down backend...")
        backend_proc.terminate()
        try:
            backend_proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            backend_proc.kill()

def run_backend_only():
    check_python_version()
    install_python_deps()
    info(f"Backend starting on port {BACKEND_PORT} — press Ctrl+C to stop")
    try:
        subprocess.run(
            [sys.executable, str(BACKEND / "main.py")],
            cwd=str(BACKEND),
            check=True,
        )
    except KeyboardInterrupt:
        print()

# ── Entry point ───────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="AI Commerce HQ dev launcher")
    parser.add_argument("--check", action="store_true", help="check environment and exit")
    parser.add_argument("--backend", action="store_true", help="start backend only")
    args = parser.parse_args()

    print("\nAI Commerce HQ — Dev Launcher")

    if args.check:
        check_environment()
        sys.exit(0)

    if args.backend:
        run_backend_only()
        return

    run_dev()

if __name__ == "__main__":
    main()
