# Generate icon files using npm tauri icon command
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot

Write-Host "Generating Tauri icons..." -ForegroundColor Cyan

# Use Tauri's icon generation from the SVG
# Requires an icon source file (png preferred, 1024x1024+)
# We'll use a PowerShell/Python approach to create placeholder PNGs

$pythonCmd = "python"
if (-not (Get-Command python -ErrorAction SilentlyContinue)) { $pythonCmd = "python3" }

$iconScript = @"
from PIL import Image, ImageDraw, ImageFont
import os

def make_icon(size, path):
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Background
    draw.rounded_rectangle([0, 0, size-1, size-1], radius=size//5, fill=(26, 31, 46, 255))
    # Inner
    m = size // 8
    draw.rounded_rectangle([m, m*2, size-m, size-m], radius=m//2, fill=(31, 58, 110, 255))
    # Text
    font_size = size // 4
    try:
        draw.text((size//2, size//2), '🏢', anchor='mm', font_size=font_size)
    except:
        pass
    os.makedirs(os.path.dirname(path), exist_ok=True)
    img.save(path)
    print(f'Created {path}')

icons_dir = 'src-tauri/icons'
make_icon(32, f'{icons_dir}/32x32.png')
make_icon(128, f'{icons_dir}/128x128.png')
make_icon(256, f'{icons_dir}/128x128@2x.png')
make_icon(256, f'{icons_dir}/icon.png')

# Create ico from png
try:
    img = Image.open(f'{icons_dir}/icon.png')
    img.save(f'{icons_dir}/icon.ico', format='ICO', sizes=[(16,16),(32,32),(48,48),(64,64),(128,128),(256,256)])
    print('Created icon.ico')
except Exception as e:
    print(f'ico creation skipped: {e}')

print('Icons generated.')
"@

& $pythonCmd -c $iconScript

Write-Host "Icons generated in src-tauri/icons/" -ForegroundColor Green
