#!/bin/bash
# Generate Swift bindings from Rust core using UniFFI
# Requires: uniffi-bindgen (pip install uniffi-bindgen==0.28.3)

set -e

echo "🔨 Generating Swift bindings for MePassa Core..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Directories
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
IOS_DIR="$PROJECT_ROOT/ios"
GENERATED_DIR="$IOS_DIR/MePassa/Generated"

# Files
UDL_FILE="$CORE_DIR/src/mepassa.udl"
LIB_FILE="$PROJECT_ROOT/target/release/libmepassa_core.dylib"

echo -e "${BLUE}Project root:${NC} $PROJECT_ROOT"
echo -e "${BLUE}UDL file:${NC} $UDL_FILE"
echo -e "${BLUE}Library:${NC} $LIB_FILE"
echo -e "${BLUE}Output directory:${NC} $GENERATED_DIR"
echo ""

# Check if UDL exists
if [ ! -f "$UDL_FILE" ]; then
    echo -e "${RED}❌ Error: UDL file not found at $UDL_FILE${NC}"
    exit 1
fi

# Check if library exists
if [ ! -f "$LIB_FILE" ]; then
    echo -e "${YELLOW}⚠️  Library not found, building...${NC}"
    cd "$CORE_DIR"
    cargo build --release --features video -p mepassa-core
    echo ""
fi

# Create output directory
mkdir -p "$GENERATED_DIR"

# Activate virtual environment if it exists
if [ -d "$IOS_DIR/venv" ]; then
    echo -e "${GREEN}Activating Python virtual environment...${NC}"
    source "$IOS_DIR/venv/bin/activate"
fi

# Check if uniffi-bindgen is available
if ! command -v uniffi-bindgen &> /dev/null; then
    echo -e "${RED}❌ Error: uniffi-bindgen not found${NC}"
    echo ""
    echo "Install with:"
    echo "  pip install uniffi-bindgen==0.28.3"
    echo ""
    echo "Or create a virtual environment:"
    echo "  cd ios && python3 -m venv venv"
    echo "  source venv/bin/activate"
    echo "  pip install uniffi-bindgen==0.28.3"
    exit 1
fi

# Generate bindings
echo -e "${GREEN}Generating Swift bindings...${NC}"
uniffi-bindgen generate "$UDL_FILE" \
  --language swift \
  --out-dir "$GENERATED_DIR" \
  --lib-file "$LIB_FILE"

# Check if generation was successful
if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ Swift bindings generated successfully!${NC}"
    echo ""
    echo -e "${BLUE}Generated files:${NC}"
    ls -lh "$GENERATED_DIR"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  1. Build iOS app: xcodegen generate && xcodebuild -scheme MePassa build"
    echo "  2. Run on simulator: open ios/MePassa.xcodeproj"
else
    echo ""
    echo -e "${RED}❌ Failed to generate Swift bindings${NC}"
    exit 1
fi
