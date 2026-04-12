#!/usr/bin/env python3
"""
AI Commerce HQ — Cross-Platform Installer Build Script
=======================================================
Builds the Windows installer (.exe) without requiring PowerShell.

Usage:
    python build-installer.py

Requirements: Python 3.11+, Node.js, Rust/Cargo
"""

import sys
import os
import subprocess
import shutil
import platform
from pathlib import Path

ROOT = Path(__file__).parent
BACKEND_DIR = ROOT / "backend"
BINARIES_DIR = ROOT / "src-tauri" / "binaries"
BUNDLE_DIR = ROOT / "src-tauri" / "target" / "release" / "bundle"
TARGET_TRIPLE = "x86_64-pc-windows-msvc"

IS_WINDOWS = platform.system() == "Windows"


def run(cmd, cwd=None, check=True, **kwargs):
    """Run a command, printing it first."""
    print(f"  $ {' '.join(str(c) for c in cmd)}")
    return subprocess.run(cmd, cwd=cwd or ROOT, check=check, **kwargs)


def step(n, total, msg):
    print(f"\n[{n}/{total}] {msg}")
    print("-" * 50)


def check_prereqs():
    """Verify all build tools are available."""
    print("\n=== Checking prerequisites ===")

    # Python version
    v = sys.version_info
    if v < (3, 11):
        print(f"ERROR: Python {v.major}.{v.minor} is too old. Need 3.11+")
        sys.exit(1)
    print(f"  ✓ Python {v.major}.{v.minor}.{v.micro}")

    # Node.js
    if not shutil.which("node"):
        print("  ✗ Node.js not found. Install from https://nodejs.org")
        sys.exit(1)
    r = subprocess.run(["node", "--version"], capture_output=True, text=True)
    print(f"  ✓ Node.js {r.stdout.strip()}")

    # Cargo
    if not shutil.which("cargo"):
        print("  ✗ Rust/Cargo not found. Install from https://rustup.rs")
        sys.exit(1)
    r = subprocess.run(["cargo", "--version"], capture_output=True, text=True)
    print(f"  ✓ {r.stdout.strip()}")

    print()


def install_python_deps():
    step(1, 4, "Installing Python dependencies (including PyInstaller)")
    run([sys.executable, "-m", "pip", "install",
         "-r", str(BACKEND_DIR / "requirements.txt"), "--quiet"])
    run([sys.executable, "-m", "pip", "install", "pyinstaller", "--quiet"])
    print("  ✓ Python dependencies installed")


def compile_backend():
    step(2, 4, "Compiling Python backend → backend.exe  (takes 2–4 minutes)")
    BINARIES_DIR.mkdir(parents=True, exist_ok=True)
    work_dir = ROOT / "build" / "pyinstaller-work"
    work_dir.mkdir(parents=True, exist_ok=True)

    run([
        sys.executable, "-m", "PyInstaller",
        str(BACKEND_DIR / "backend.spec"),
        "--distpath", str(BINARIES_DIR),
        "--workpath", str(work_dir),
        "--noconfirm",
    ], cwd=BACKEND_DIR)

    out = BINARIES_DIR / "backend.exe"
    if not out.exists() or out.stat().st_size == 0:
        print(f"\nERROR: Expected {out} but it was not created or is empty.")
        sys.exit(1)

    size_mb = out.stat().st_size / (1024 * 1024)
    print(f"  ✓ backend.exe compiled ({size_mb:.1f} MB)")


def install_node_deps():
    if not (ROOT / "node_modules").exists():
        step(3, 4, "Installing Node.js dependencies")
        run(["npm", "install", "--silent"])
        print("  ✓ Node.js dependencies installed")
    else:
        step(3, 4, "Node.js dependencies (already present, skipping)")


def build_tauri():
    step(4, 4, "Building Tauri NSIS installer")
    run(["npm", "run", "tauri:build"])


def report_output():
    print("\n" + "=" * 50)
    print("  BUILD COMPLETE")
    print("=" * 50)

    # Find the installer
    installer = None
    for pattern in ["**/*setup*.exe", "**/nsis/*.exe"]:
        matches = list(BUNDLE_DIR.glob(pattern))
        if matches:
            installer = max(matches, key=lambda p: p.stat().st_size)
            break

    if installer:
        size_mb = installer.stat().st_size / (1024 * 1024)
        print(f"\n  Installer : {installer}")
        print(f"  Size      : {size_mb:.1f} MB")
        print("\n  The installer creates a desktop shortcut and Start Menu entry.")
        print("  No Python, Node.js, or Rust required on the target machine.\n")
    else:
        print(f"\n  Installer built — check {BUNDLE_DIR}")


if __name__ == "__main__":
    print("\n==============================================")
    print("  AI Commerce HQ — Windows Installer Builder")
    print("==============================================")

    check_prereqs()
    install_python_deps()
    compile_backend()
    install_node_deps()
    build_tauri()
    report_output()
