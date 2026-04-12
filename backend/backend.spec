# -*- mode: python ; coding: utf-8 -*-
# PyInstaller spec for AI Commerce HQ backend.
# Run from the project root: pyinstaller backend/backend.spec

import sys
import os
from PyInstaller.utils.hooks import collect_all, collect_submodules

block_cipher = None

# Collect everything PyInstaller might miss with dynamic imports
datas = []
binaries = []
hiddenimports = []

# FastAPI / Starlette
td, tb, th = collect_all('fastapi')
datas += td; binaries += tb; hiddenimports += th

td, tb, th = collect_all('starlette')
datas += td; binaries += tb; hiddenimports += th

# Uvicorn and all its protocol handlers
td, tb, th = collect_all('uvicorn')
datas += td; binaries += tb; hiddenimports += th

hiddenimports += [
    'uvicorn.logging',
    'uvicorn.loops',
    'uvicorn.loops.auto',
    'uvicorn.loops.asyncio',
    'uvicorn.protocols',
    'uvicorn.protocols.http',
    'uvicorn.protocols.http.auto',
    'uvicorn.protocols.http.h11_impl',
    'uvicorn.protocols.websockets',
    'uvicorn.protocols.websockets.auto',
    'uvicorn.protocols.websockets.websockets_impl',
    'uvicorn.lifespan',
    'uvicorn.lifespan.on',
    'uvicorn.lifespan.off',
    'h11',
    'websockets',
    'websockets.legacy',
    'websockets.legacy.server',
]

# SQLAlchemy + aiosqlite
hiddenimports += [
    'sqlalchemy.dialects.sqlite',
    'sqlalchemy.ext.asyncio',
    'aiosqlite',
]

# OpenAI / httpx / pydantic
td, tb, th = collect_all('openai')
datas += td; binaries += tb; hiddenimports += th

hiddenimports += [
    'httpx',
    'pydantic',
    'pydantic.v1',
    'anyio',
    'anyio.from_thread',
    'sniffio',
]

# Our own backend packages
hiddenimports += collect_submodules('api')
hiddenimports += collect_submodules('database')
hiddenimports += collect_submodules('orchestrator')
hiddenimports += collect_submodules('agents')
hiddenimports += collect_submodules('tools')
hiddenimports += collect_submodules('state')

a = Analysis(
    ['main.py'],
    pathex=['.'],          # backend/ directory
    binaries=binaries,
    datas=datas,
    hiddenimports=hiddenimports,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=['tkinter', 'matplotlib', 'numpy', 'pandas', 'scipy', 'pytest'],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
    noarchive=False,
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.zipfiles,
    a.datas,
    [],
    name='backend',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,          # No console window for end users
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
