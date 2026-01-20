# Build Guide - MePassa Desktop

Complete step-by-step build process for MePassa Desktop (Tauri 2.0).

## 📦 Build Process Overview

```
1. Install Prerequisites
   ↓
2. Install Dependencies (npm install)
   ↓
3. Build Frontend (Vite)
   ↓
4. Build Backend (Cargo)
   ↓
5. Create Bundle (Tauri CLI)
   ↓
6. Test Application
```

## 🛠️ Prerequisites

### 1. Node.js & npm

```bash
# Check Node.js version (needs 18+)
node --version
# Should output: v18.x.x or higher

# Check npm version
npm --version
# Should output: 9.x.x or higher
```

**Install if needed:**
- **macOS:** `brew install node`
- **Linux:** `curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash - && sudo apt-get install -y nodejs`
- **Windows:** Download from [nodejs.org](https://nodejs.org/)

### 2. Rust Toolchain

```bash
# Check Rust version (needs 1.70+)
rustc --version
# Should output: rustc 1.70.x or higher

# Check Cargo version
cargo --version
```

**Install if needed:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Tauri CLI

```bash
# Install Tauri CLI globally
npm install -g @tauri-apps/cli@next

# Verify installation
tauri --version
# Should output: 2.x.x
```

### 4. System Dependencies

#### macOS

```bash
# Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Windows

- **WebView2:** Pre-installed on Windows 10/11
- **Visual Studio Build Tools:** [Download here](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - Select "Desktop development with C++"

## 📋 Step-by-Step Build

### Step 1: Clone & Navigate

```bash
cd /Users/edsonmartins/desenvolvimento/mepassa/desktop

# Verify structure
ls -la
# Should see: package.json, src/, src-tauri/
```

### Step 2: Install Dependencies

```bash
# Install Node.js dependencies
npm install

# This installs:
# - React, React DOM, React Router
# - Tauri API packages
# - Vite, TypeScript, TailwindCSS
# - Development tools

# Expected output:
# added 234 packages in 15s
```

**Verify:**
```bash
ls node_modules | wc -l
# Should output: ~200+ packages

npm list --depth=0
# Should show main dependencies without errors
```

### Step 3: Build Frontend (Development)

```bash
# Build React app with Vite
npm run build

# This runs: tsc && vite build

# Expected output:
# vite v5.0.8 building for production...
# ✓ 234 modules transformed.
# dist/index.html                   0.45 kB │ gzip:  0.30 kB
# dist/assets/index-a1b2c3d4.css    5.67 kB │ gzip:  1.89 kB
# dist/assets/index-e5f6g7h8.js   142.34 kB │ gzip: 45.12 kB
# ✓ built in 3.45s
```

**Verify:**
```bash
ls -lh dist/
# Should see: index.html, assets/

du -sh dist/
# Should be: ~500K (minified production build)
```

### Step 4: Build Backend (Rust)

```bash
cd src-tauri

# Build Rust backend (release mode)
cargo build --release

# This compiles:
# - Tauri runtime
# - mepassa-core (local dependency)
# - All Rust dependencies

# Expected output:
#    Compiling mepassa-core v0.1.0 (/path/to/core)
#    Compiling mepassa-desktop v0.1.0 (/path/to/desktop/src-tauri)
#     Finished `release` profile [optimized] target(s) in 2m 34s
```

**Verify:**
```bash
ls -lh target/release/mepassa-desktop
# Should exist and be ~15 MB (macOS)

# Test executable (quick check)
./target/release/mepassa-desktop --version
# Should output: mepassa-desktop 0.1.0
```

### Step 5: Create Bundle

```bash
cd ..  # Back to desktop/ root

# Build complete bundle (includes frontend + backend + installer)
npm run tauri:build

# This runs: tauri build

# Expected output (macOS):
#     Finished 2 bundles at:
#         /path/to/desktop/src-tauri/target/release/bundle/macos/MePassa.app
#         /path/to/desktop/src-tauri/target/release/bundle/dmg/MePassa_0.1.0_aarch64.dmg
#
#     Bundle sizes:
#         MePassa.app: 18.5 MB
#         MePassa_0.1.0_aarch64.dmg: 19.2 MB
```

**Verify bundles:**

#### macOS
```bash
ls -lh src-tauri/target/release/bundle/macos/
# MePassa.app (application bundle)

ls -lh src-tauri/target/release/bundle/dmg/
# MePassa_0.1.0_aarch64.dmg (disk image installer)
```

#### Linux
```bash
ls -lh src-tauri/target/release/bundle/appimage/
# mepassa-desktop_0.1.0_amd64.AppImage

ls -lh src-tauri/target/release/bundle/deb/
# mepassa-desktop_0.1.0_amd64.deb
```

#### Windows
```bash
ls -lh src-tauri/target/release/bundle/msi/
# MePassa_0.1.0_x64_en-US.msi

ls -lh src-tauri/target/release/bundle/nsis/
# MePassa_0.1.0_x64-setup.exe
```

## 🚀 Running the Application

### Development Mode

```bash
npm run tauri:dev

# This will:
# 1. Start Vite dev server (port 5173)
# 2. Compile Rust backend
# 3. Launch desktop app
# 4. Enable hot-reload for frontend changes
```

**Hot Reload:**
- Edit files in `src/` → frontend reloads automatically
- Edit files in `src-tauri/src/` → need to restart (Cmd/Ctrl+C, then `npm run tauri:dev` again)

### Production Build

#### macOS
```bash
# Run .app directly
open src-tauri/target/release/bundle/macos/MePassa.app

# Or install .dmg
open src-tauri/target/release/bundle/dmg/MePassa_0.1.0_aarch64.dmg
# Drag MePassa.app to Applications folder
```

#### Linux (AppImage)
```bash
# Make executable
chmod +x src-tauri/target/release/bundle/appimage/mepassa-desktop_0.1.0_amd64.AppImage

# Run
./src-tauri/target/release/bundle/appimage/mepassa-desktop_0.1.0_amd64.AppImage
```

#### Windows (MSI)
```
Double-click: src-tauri\target\release\bundle\msi\MePassa_0.1.0_x64_en-US.msi
Follow installer wizard
```

## 🔍 Build Verification Checklist

### Pre-Build
- [ ] Node.js 18+ installed
- [ ] Rust 1.70+ installed
- [ ] Tauri CLI installed
- [ ] System dependencies installed
- [ ] `npm install` completed successfully

### Post-Build
- [ ] `dist/` directory exists with index.html
- [ ] `src-tauri/target/release/mepassa-desktop` binary exists
- [ ] Bundle created in `src-tauri/target/release/bundle/`
- [ ] Application launches without errors
- [ ] System tray icon appears
- [ ] Client initializes (check logs)

### Functional Tests
- [ ] Onboarding screen shows peer ID
- [ ] "Get Started" button navigates to Conversations
- [ ] "New Chat" dialog opens
- [ ] Can enter peer ID and start chat
- [ ] Messages can be sent and received
- [ ] System tray left-click shows/hides window
- [ ] System tray right-click shows menu
- [ ] "Quit" menu item closes app

## 🐛 Troubleshooting

### Error: "Failed to resolve module"

**Cause:** Missing Node.js dependencies

**Solution:**
```bash
rm -rf node_modules package-lock.json
npm install
```

### Error: "cargo: command not found"

**Cause:** Rust not in PATH

**Solution:**
```bash
source $HOME/.cargo/env
# Or restart terminal
```

### Error: "webkit2gtk not found" (Linux)

**Cause:** Missing system dependency

**Solution:**
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

### Error: "Tauri build failed" on Windows

**Cause:** Missing Visual Studio Build Tools

**Solution:**
- Install Visual Studio Build Tools
- Select "Desktop development with C++"
- Restart computer

### Bundle size too large

**Optimization:**
```bash
# Enable smaller binary (Cargo.toml)
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
strip = true        # Strip symbols
```

**Result:**
- Binary size: ~15 MB → ~10 MB
- Bundle size: ~18 MB → ~12 MB

## 📊 Build Performance

| Operation | Time (macOS M1) | Time (Linux x64) | Time (Windows x64) |
|-----------|-----------------|------------------|--------------------|
| `npm install` | 15s | 20s | 25s |
| `npm run build` | 3s | 4s | 5s |
| `cargo build --release` | 2m 30s | 3m 20s | 4m 10s |
| `tauri build` (full) | 3m 15s | 4m 30s | 5m 45s |
| **Total (clean build)** | **~3.5 min** | **~5 min** | **~6.5 min** |

**Incremental builds:**
- Frontend only: ~3s
- Backend only (code changes): ~30s
- Full rebuild: Same as above

## 🚀 Build Automation Script

Create `build.sh` in desktop root:

```bash
#!/bin/bash
set -e  # Exit on error

echo "🔨 MePassa Desktop Build Script"
echo "================================"

# 1. Check prerequisites
echo "📋 Checking prerequisites..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js not installed"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust not installed"; exit 1; }
echo "✅ Prerequisites OK"

# 2. Install dependencies
echo ""
echo "📦 Installing dependencies..."
npm install

# 3. Build
echo ""
echo "🔨 Building application..."
npm run tauri:build

# 4. Verify
echo ""
echo "🔍 Verifying build..."
if [ -d "src-tauri/target/release/bundle" ]; then
    echo "✅ Build successful!"
    echo ""
    echo "📦 Bundles created:"
    find src-tauri/target/release/bundle -type f -name "MePassa*" -o -name "mepassa*"
else
    echo "❌ Build failed!"
    exit 1
fi
```

**Usage:**
```bash
chmod +x build.sh
./build.sh
```

## 📝 Build Checklist

Before releasing:

- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Run `npm run tauri:build` on all platforms
- [ ] Test bundles on clean systems
- [ ] Verify code signing (macOS, Windows)
- [ ] Create release notes
- [ ] Tag git commit: `git tag v0.1.0`

---

**Last Updated:** 2025-01-20
**Tested On:** macOS Sonoma 14.4, Ubuntu 22.04, Windows 11
