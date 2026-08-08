#!/bin/bash
# Build script for Rust core library for Android
# Compiles libzaplivre_core for Android devices and emulators

set -e

echo "🔨 Building Rust core library for Android..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
ANDROID_DIR="$PROJECT_ROOT/android"
JNILIBS_DIR="$ANDROID_DIR/app/src/main/jniLibs"

echo -e "${BLUE}Project root:${NC} $PROJECT_ROOT"
echo -e "${BLUE}Core directory:${NC} $CORE_DIR"
echo -e "${BLUE}Android directory:${NC} $ANDROID_DIR"
echo ""

# Create jniLibs directories
mkdir -p "$JNILIBS_DIR/armeabi-v7a"
mkdir -p "$JNILIBS_DIR/arm64-v8a"
mkdir -p "$JNILIBS_DIR/x86_64"

# Check if Android targets are installed
echo -e "${YELLOW}Checking Rust Android targets...${NC}"
TARGETS_NEEDED=(
    "armv7-linux-androideabi"
    "aarch64-linux-android"
    "x86_64-linux-android"
)

for target in "${TARGETS_NEEDED[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${YELLOW}Installing $target...${NC}"
        rustup target add "$target"
    else
        echo -e "  ✅ $target already installed"
    fi
done
echo ""

# protoc is required by the core build (libsignal/spqr)
if ! command -v protoc > /dev/null 2>&1; then
    echo -e "${RED}ERROR: protoc not found. Install it first (macOS: brew install protobuf).${NC}"
    exit 1
fi

# Resolve Android SDK location: ANDROID_HOME / ANDROID_SDK_ROOT / local.properties / default
SDK_ROOT="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-}}"
if [ -z "$SDK_ROOT" ] && [ -f "$ANDROID_DIR/local.properties" ]; then
    SDK_ROOT=$(sed -n 's/^sdk\.dir=//p' "$ANDROID_DIR/local.properties")
fi
if [ -z "$SDK_ROOT" ]; then
    case "$(uname -s)" in
        Darwin) SDK_ROOT="$HOME/Library/Android/sdk" ;;
        *)      SDK_ROOT="$HOME/Android/Sdk" ;;
    esac
fi

# Resolve NDK: ANDROID_NDK_HOME or newest NDK installed under the SDK
if [ -n "${ANDROID_NDK_HOME:-}" ]; then
    NDK_PATH="$ANDROID_NDK_HOME"
else
    NDK_PATH=$(ls -d "$SDK_ROOT/ndk/"* 2>/dev/null | sort -V | tail -1)
fi
if [ -z "$NDK_PATH" ] || [ ! -d "$NDK_PATH" ]; then
    echo -e "${RED}ERROR: Android NDK not found.${NC}"
    echo "Set ANDROID_NDK_HOME or install an NDK under $SDK_ROOT/ndk (sdkmanager 'ndk;26.3.11579264')."
    exit 1
fi
echo -e "${BLUE}Using NDK:${NC} $NDK_PATH"

case "$(uname -s)" in
    Darwin) HOST_TAG="darwin-x86_64" ;;
    Linux)  HOST_TAG="linux-x86_64" ;;
    *) echo -e "${RED}Unsupported host OS: $(uname -s)${NC}"; exit 1 ;;
esac
TOOLCHAIN_PATH="$NDK_PATH/toolchains/llvm/prebuilt/$HOST_TAG/bin"
if [ ! -d "$TOOLCHAIN_PATH" ]; then
    echo -e "${RED}ERROR: NDK toolchain not found at $TOOLCHAIN_PATH${NC}"
    exit 1
fi
export AR="$TOOLCHAIN_PATH/llvm-ar"

# NDK 28+ só distribui llvm-ar (não os wrappers <triple>-ar). cc-rs de algumas
# dependências (sha2-asm, ring) procuram por "<triple>-ar" na PATH - criar
# symlinks locais para o llvm-ar evita falha nesses build scripts.
for triple in x86_64-linux-android aarch64-linux-android armv7a-linux-androideabi; do
    if [ ! -e "$TOOLCHAIN_PATH/$triple-ar" ]; then
        ln -sf "$TOOLCHAIN_PATH/llvm-ar" "$TOOLCHAIN_PATH/$triple-ar"
    fi
done

# Build for Android ARM64 (64-bit ARM - most modern devices)
echo -e "${GREEN}Building for Android ARM64 (aarch64-linux-android)...${NC}"
cd "$CORE_DIR"
export CC_aarch64_linux_android="$TOOLCHAIN_PATH/aarch64-linux-android24-clang"
export CXX_aarch64_linux_android="$TOOLCHAIN_PATH/aarch64-linux-android24-clang++"
# Override any machine-specific linker paths from .cargo/config.toml. This keeps
# native builds reproducible when multiple NDK versions are installed.
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$TOOLCHAIN_PATH/aarch64-linux-android24-clang"
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$TOOLCHAIN_PATH/armv7a-linux-androideabi24-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="$TOOLCHAIN_PATH/x86_64-linux-android24-clang"
# audiopus_sys invokes CMake directly; provide the selected NDK explicitly so
# CMake does not depend on a machine-wide Android SDK installation.
export CMAKE_ANDROID_NDK="$NDK_PATH"
export CMAKE_ANDROID_NDK_aarch64_linux_android="$NDK_PATH"
export CMAKE_TOOLCHAIN_FILE_aarch64_linux_android="$NDK_PATH/build/cmake/android.toolchain.cmake"
export CMAKE_TOOLCHAIN_FILE="$NDK_PATH/build/cmake/android.toolchain.cmake"
export CMAKE_aarch64_linux_android="-DCMAKE_TOOLCHAIN_FILE=$NDK_PATH/build/cmake/android.toolchain.cmake -DANDROID_ABI=arm64-v8a -DANDROID_PLATFORM=android-24"
export CMAKE_MAKE_PROGRAM="/usr/bin/make"
export CMAKE_GENERATOR="Unix Makefiles"
cargo build --release --target aarch64-linux-android -p zaplivre-core
mkdir -p "$JNILIBS_DIR/arm64-v8a"
cp "$PROJECT_ROOT/target/aarch64-linux-android/release/libzaplivre_core.so" \
   "$JNILIBS_DIR/arm64-v8a/libzaplivre_core.so"
echo -e "  ✅ arm64-v8a/libzaplivre_core.so"

# The connected test tablet is ARM64. Avoid compiling unused ABIs by default;
# CI can set BUILD_ANDROID_ALL=1 to produce the complete multi-ABI package.
if [ "${BUILD_ANDROID_ALL:-0}" != "1" ]; then
    exit 0
fi
echo ""

# Build for Android ARMv7 (32-bit ARM - older devices)
echo -e "${GREEN}Building for Android ARMv7 (armv7-linux-androideabi)...${NC}"
export CC_armv7_linux_androideabi="$TOOLCHAIN_PATH/armv7a-linux-androideabi24-clang"
export CXX_armv7_linux_androideabi="$TOOLCHAIN_PATH/armv7a-linux-androideabi24-clang++"
cargo build --release --target armv7-linux-androideabi -p zaplivre-core
echo ""

# Build for Android x86_64 (emulators)
echo -e "${GREEN}Building for Android x86_64 (x86_64-linux-android)...${NC}"
export CC_x86_64_linux_android="$TOOLCHAIN_PATH/x86_64-linux-android24-clang"
export CXX_x86_64_linux_android="$TOOLCHAIN_PATH/x86_64-linux-android24-clang++"
cargo build --release --target x86_64-linux-android -p zaplivre-core
echo ""

# Copy libraries to jniLibs
echo -e "${GREEN}Copying libraries to jniLibs...${NC}"

cp "$PROJECT_ROOT/target/aarch64-linux-android/release/libzaplivre_core.so" \
   "$JNILIBS_DIR/arm64-v8a/libzaplivre_core.so"
echo -e "  ✅ arm64-v8a/libzaplivre_core.so"

cp "$PROJECT_ROOT/target/armv7-linux-androideabi/release/libzaplivre_core.so" \
   "$JNILIBS_DIR/armeabi-v7a/libzaplivre_core.so"
echo -e "  ✅ armeabi-v7a/libzaplivre_core.so"

cp "$PROJECT_ROOT/target/x86_64-linux-android/release/libzaplivre_core.so" \
   "$JNILIBS_DIR/x86_64/libzaplivre_core.so"
echo -e "  ✅ x86_64/libzaplivre_core.so"
echo ""

# Show library sizes
echo -e "${GREEN}Library sizes:${NC}"
du -h "$JNILIBS_DIR"/*/*.so
echo ""

echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo -e "${BLUE}Native libraries are ready in:${NC}"
echo "  $JNILIBS_DIR"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "  1. Build Android app: cd android && ./gradlew assembleDebug"
echo "  2. Install on device/emulator: adb install app/build/outputs/apk/debug/app-debug.apk"
