#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DESKTOP_DIR="$ROOT_DIR/desktop"
TAURI_CONF="$DESKTOP_DIR/src-tauri/tauri.conf.json"
PACKAGE_JSON="$DESKTOP_DIR/package.json"
DMG_SCRIPT="$DESKTOP_DIR/src-tauri/target/release/bundle/dmg/bundle_dmg.sh"
APP_PATH="$DESKTOP_DIR/src-tauri/target/release/bundle/macos/MePassa.app"

read_version_from_tauri() {
  if [[ -f "$TAURI_CONF" ]]; then
    rg -n "\"version\"" "$TAURI_CONF" | head -n 1 | sed -E 's/.*"version"\s*:\s*"([^"]+)".*/\1/'
  fi
}

read_version_from_package() {
  if [[ -f "$PACKAGE_JSON" ]]; then
    rg -n "\"version\"" "$PACKAGE_JSON" | head -n 1 | sed -E 's/.*"version\"\s*:\s*\"([^\"]+)\".*/\1/'
  fi
}

VERSION=$(read_version_from_tauri)
if [[ -z "$VERSION" ]]; then
  VERSION=$(read_version_from_package)
fi

if [[ -z "$VERSION" ]]; then
  echo "Failed to read version from $TAURI_CONF or $PACKAGE_JSON" >&2
  exit 1
fi

DMG_PATH="$DESKTOP_DIR/src-tauri/target/release/bundle/dmg/MePassa_${VERSION}_x64.dmg"

if [[ ! -x "$DMG_SCRIPT" ]]; then
  echo "DMG script not found. Run 'npm run tauri:build' first." >&2
  exit 1
fi

if [[ ! -d "$APP_PATH" ]]; then
  echo "App bundle not found at $APP_PATH" >&2
  echo "Run 'npm run tauri:build' first." >&2
  exit 1
fi

echo "Building DMG for version $VERSION"
echo "Source: $APP_PATH"
echo "Output: $DMG_PATH"

bash "$DMG_SCRIPT" "$DMG_PATH" "$APP_PATH"

echo "DMG created at: $DMG_PATH"
