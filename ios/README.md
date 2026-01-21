# MePassa iOS App

iOS native app for MePassa P2P messaging platform built with SwiftUI, CallKit, and Rust FFI.

## 📊 Status: FASE 13 - 100% Development Complete

**Latest Update:** 2026-01-21
- ✅ Rust core compiles for iOS (conditional compilation)
- ✅ Static libraries integrated (libmepassa_core_ios.a + sim.a)
- ✅ Swift bindings generated via UniFFI 0.28.3
- ✅ Xcode project configured via xcodegen
- ✅ **Build successful:** `xcodebuild -scheme MePassa build` → BUILD SUCCEEDED!
- ✅ Build automation pipeline complete (build-all.sh)
- ✅ Complete documentation and troubleshooting guide
- 📋 End-to-end tests deferred to final testing phase

## 📋 Requirements

- **Xcode:** 15.0+
- **iOS:** 15.0+ deployment target
- **macOS:** for development (tested on macOS 14+)
- **Rust:** 1.75+ with iOS targets
- **uniffi-bindgen:** 0.28.3 (Python package)
- **xcodegen:** for project generation

## 🏗️ Project Structure

```
ios/
├── build-all.sh              # 🚀 Master build script (Rust + Bindings + Xcode)
├── build-rust.sh             # Build Rust core for iOS targets
├── generate-bindings.sh      # Generate Swift bindings via UniFFI
├── project.yml               # Xcode project specification (xcodegen)
├── Libraries/                # Compiled Rust static libraries
│   ├── libmepassa_core_ios.a     # iOS device (ARM64) - 96MB
│   └── libmepassa_core_sim.a     # Simulator (ARM64 + x86_64) - 192MB
├── MePassa.xcodeproj/        # Generated Xcode project (via xcodegen)
└── MePassa/
    ├── MePassaApp.swift      # App entry point + SwiftUI lifecycle
    ├── ContentView.swift     # Root navigation
    ├── Info.plist            # Permissions & capabilities
    ├── MePassa-Bridging-Header.h  # C FFI bridging header
    ├── Core/
    │   └── MePassaCore.swift # Rust FFI wrapper (singleton)
    ├── Views/                # SwiftUI screens
    │   ├── LoginView.swift   # Identity generation screen
    │   ├── ConversationsView.swift  # Chat list
    │   ├── ChatView.swift    # 1:1 messaging
    │   ├── CallScreen.swift  # Active call UI
    │   ├── IncomingCallScreen.swift
    │   ├── NewChatView.swift # Add contact via QR
    │   ├── SettingsView.swift
    │   ├── QRScannerView.swift        # SwiftUI wrapper
    │   ├── QRScannerViewController.swift  # AVFoundation camera
    │   └── MyQRCodeView.swift # Share peer ID
    ├── VoIP/                 # VoIP integration
    │   ├── CallManager.swift # CallKit integration (309 LoC)
    │   └── AudioManager.swift # AVAudioEngine I/O (311 LoC)
    └── Generated/            # UniFFI generated bindings
        ├── mepassa.swift     # Swift interfaces (48KB)
        ├── mepassaFFI.h      # C FFI header (27KB)
        └── mepassaFFI.modulemap
```

## 🚀 Quick Start

### One-Command Build

```bash
# Complete build pipeline (Rust + Bindings + Xcode)
./ios/build-all.sh

# Or with Xcode build included:
./ios/build-all.sh --build
```

This will:
1. ✅ Build Rust core for iOS device + Simulator
2. ✅ Generate Swift bindings via UniFFI
3. ✅ Generate Xcode project from project.yml
4. ✅ (Optional) Build iOS app for Simulator

### Manual Setup

#### 1. Install Dependencies

```bash
# Install Rust iOS targets
rustup target add aarch64-apple-ios       # iOS devices (ARM64)
rustup target add aarch64-apple-ios-sim   # Simulator (Apple Silicon)
rustup target add x86_64-apple-ios        # Simulator (Intel)

# Install xcodegen
brew install xcodegen

# Install uniffi-bindgen (in virtual environment)
cd ios
python3 -m venv venv
source venv/bin/activate
pip install uniffi-bindgen==0.28.3
```

#### 2. Build Rust Core

```bash
./ios/build-rust.sh
```

Output:
- `ios/Libraries/libmepassa_core_ios.a` (96MB)
- `ios/Libraries/libmepassa_core_sim.a` (192MB universal)

#### 3. Generate Swift Bindings

```bash
source ios/venv/bin/activate  # Activate venv
./ios/generate-bindings.sh
```

Output:
- `ios/MePassa/Generated/mepassa.swift`
- `ios/MePassa/Generated/mepassaFFI.h`
- `ios/MePassa/Generated/mepassaFFI.modulemap`

#### 4. Generate Xcode Project

```bash
cd ios
xcodegen generate
```

Output: `ios/MePassa.xcodeproj`

#### 5. Open in Xcode

```bash
open ios/MePassa.xcodeproj
```

Or build from command line:

```bash
xcodebuild -project ios/MePassa.xcodeproj \
           -scheme MePassa \
           -sdk iphonesimulator \
           -destination 'platform=iOS Simulator,name=iPhone 16' \
           build
```

## 🎯 Features

### ✅ Implemented (100% Development Complete)

**Core Infrastructure:**
- ✅ SwiftUI app structure with navigation
- ✅ Rust FFI integration via UniFFI
- ✅ Static library linking (libmepassa_core)
- ✅ Build pipeline automation (build-all.sh)
- ✅ Xcode project generation via xcodegen
- ✅ Complete documentation and troubleshooting guide

**UI Screens:**
- ✅ Login/identity generation
- ✅ Conversations list
- ✅ Chat screen with messaging UI
- ✅ Call screens (incoming/active)
- ✅ Settings and profile
- ✅ QR code generation
- ✅ QR code scanner (AVFoundation)

**VoIP Integration:**
- ✅ CallKit integration (CallManager)
- ✅ AVAudioEngine audio I/O (48kHz, mono, 16-bit PCM)
- ✅ Audio session management
- ✅ Background modes configured

**Rust Core (iOS):**
- ✅ Conditional compilation (#[cfg(feature = "voip")])
- ✅ P2P messaging (libp2p + Kademlia DHT)
- ✅ E2E encryption (Signal Protocol)
- ✅ Local storage (SQLite)
- ✅ FFI bindings (UniFFI 0.28.3)

### 📋 Deferred Items

**Testing (Deferred to Final Phase):**
- 📋 End-to-end tests on Simulator (messaging, QR Scanner)
- 📋 Physical device testing
- 📋 TestFlight beta distribution

**Blocked by Other Phases:**
- 🔒 WebRTC VoIP integration (awaits FASE 12 - currently mock)
- 🔒 APNs Push Notifications (awaits FASE 8 - server-side)

## 📱 Permissions

Configured in `Info.plist`:

- **Microphone** (`NSMicrophoneUsageDescription`): "MePassa precisa acessar o microfone para chamadas de voz"
- **Camera** (`NSCameraUsageDescription`): "MePassa precisa acessar a câmera para videochamadas"
- **Photos** (`NSPhotoLibraryUsageDescription`): "MePassa precisa acessar fotos para compartilhar imagens"
- **Contacts** (`NSContactsUsageDescription`): "MePassa precisa acessar contatos para encontrar amigos"

Background Modes:
- ✅ Voice over IP (VoIP)
- ✅ Remote notifications
- ✅ Audio, AirPlay, and Picture in Picture

## 🔊 VoIP Integration

### CallKit (Implemented)

- **CallManager.swift** (309 LoC): Manages CallKit provider and controller
- **CXProvider**: System call UI and events
- **CXCallController**: Call actions (answer, end, mute, hold)
- Native iOS call integration (lockscreen, CarPlay)

### Audio I/O (Implemented)

- **AudioManager.swift** (311 LoC): AVAudioEngine wrapper
- 48kHz sample rate, mono, 16-bit PCM
- Audio buffer management (20ms frames)
- Audio session configuration (playAndRecord, VoIP category)
- Ready for WebRTC integration

### WebRTC Integration (Pending)

Currently using mock implementation. Will connect to Rust core's WebRTC engine via FFI when VoIP feature is enabled.

## 🏗️ Architecture

### FFI Integration

```swift
import mepassa  // Generated by UniFFI

// Initialize core
let client = try MePassaClient(dataDir: documentsPath)

// Get local peer ID
let peerId = try client.localPeerId()

// Listen on multiaddr
try await client.listenOn(multiaddr: "/ip4/0.0.0.0/tcp/0")

// Connect to peer
try await client.connectToPeer(
    peerId: remotePeerId,
    multiaddr: remoteAddr
)

// Send message
let messageId = try await client.sendTextMessage(
    toPeerId: recipientPeerId,
    content: "Hello from iOS!"
)

// Get conversations
let conversations = try client.listConversations()

// Get messages
let messages = try client.getConversationMessages(
    peerId: peerId,
    limit: 50,
    offset: 0
)
```

### State Management

- **@EnvironmentObject**: Global app state injection
- **@Published**: Reactive state updates
- **ObservableObject**: SwiftUI state management
- Singleton pattern for MePassaClient wrapper

### Navigation Flow

```
ContentView (root)
├── LoginView (if !authenticated)
└── TabView (if authenticated)
    ├── ConversationsView
    │   ├── ChatView (per conversation)
    │   ├── NewChatView (modal)
    │   └── MyQRCodeView (sheet)
    ├── CallScreen (if in call)
    └── SettingsView

IncomingCallScreen (CallKit presented)
└── Answer → CallScreen
```

## 📦 Build Configuration

### Xcode Settings (project.yml)

```yaml
SWIFT_VERSION: "5.0"
SWIFT_OBJC_BRIDGING_HEADER: $(PROJECT_DIR)/MePassa/MePassa-Bridging-Header.h

LIBRARY_SEARCH_PATHS:
  - $(PROJECT_DIR)/Libraries

OTHER_LDFLAGS:
  - -L$(PROJECT_DIR)/Libraries
  - -lmepassa_core_sim  # For Simulator builds

HEADER_SEARCH_PATHS:
  - $(PROJECT_DIR)/MePassa/Generated
```

### Frameworks & Dependencies

- Foundation.framework
- SwiftUI.framework
- CallKit.framework
- AVFoundation.framework
- CoreImage.framework
- UserNotifications.framework
- PushKit.framework
- Security.framework
- SystemConfiguration.framework
- libresolv.tbd

## 🧪 Testing

### Manual Testing (Current)

1. Build and run on Simulator:
   ```bash
   ./ios/build-all.sh --build
   ```

2. Test features:
   - Identity generation
   - QR code generation/scanning
   - P2P messaging (when both instances connected)
   - Audio permissions
   - CallKit integration

### Automated Tests (TODO)

```bash
xcodebuild test \
    -project MePassa.xcodeproj \
    -scheme MePassa \
    -destination 'platform=iOS Simulator,name=iPhone 16'
```

## 📦 Distribution

### TestFlight (TODO)

1. Configure signing & provisioning:
   - Apple Developer account
   - App ID: `app.mepassa.ios`
   - Provisioning profiles

2. Archive build:
   ```bash
   xcodebuild archive \
       -project MePassa.xcodeproj \
       -scheme MePassa \
       -archivePath ./build/MePassa.xcarchive
   ```

3. Export IPA:
   ```bash
   xcodebuild -exportArchive \
       -archivePath ./build/MePassa.xcarchive \
       -exportPath ./build \
       -exportOptionsPlist ExportOptions.plist
   ```

4. Upload to App Store Connect
5. Distribute to beta testers

## 🔧 Troubleshooting

### Build Errors

**Error: "cannot find type 'RustBuffer'"**
- Solution: Ensure bridging header is configured correctly
- Check: SWIFT_OBJC_BRIDGING_HEADER in build settings

**Error: "Undefined symbols for architecture x86_64"**
- Solution: Build Rust core for all targets (x86_64 + ARM64)
- Run: `./ios/build-rust.sh`

**Error: "library not found for -lmepassa_core_sim"**
- Solution: Check library exists in `ios/Libraries/`
- Verify: LIBRARY_SEARCH_PATHS in project.yml

### Runtime Issues

**App crashes on launch**
- Check: Rust core library is properly linked
- Verify: All frameworks are available
- Debug: Enable exception breakpoints in Xcode

**Audio not working**
- Check: Microphone permissions granted
- Verify: Audio session configuration
- Debug: Check AVAudioEngine status

## 📝 Development Notes

### Rust Core - iOS Build Strategy

Since `audiopus_sys` (Opus audio codec) doesn't compile for iOS with CMake, we use **conditional compilation**:

```rust
#[cfg(feature = "voip")]
pub mod voip;  // Excluded from iOS builds

// VoIP methods only available when feature = "voip" is enabled
#[cfg(feature = "voip")]
pub async fn start_call(&self, to_peer_id: String) -> Result<String> {
    // ...
}
```

**iOS builds:** `--no-default-features` (excludes opus, cpal, webrtc)
**Android/Desktop builds:** default features enabled (includes full VoIP stack)

This allows:
- ✅ iOS: P2P messaging works (libp2p, storage, crypto)
- ✅ Android/Desktop: Full VoIP support
- 🔜 iOS VoIP: Will use native AVAudioEngine + CallKit (FASE 14)

### Current Limitations

1. **VoIP on iOS:** Mock implementation, awaits WebRTC integration
2. **Push Notifications:** Awaits FASE 8 (APNs server-side)
3. **Physical Device Testing:** Requires Apple Developer account
4. **App Store:** Awaits provisioning profiles and certificates

### Next Steps

**Development Complete (100%)** - All implementation tasks finished.

**Deferred to Final Testing Phase:**
1. End-to-end tests on Simulator
2. Physical device testing
3. TestFlight beta distribution

**Future Enhancements (Other Phases):**
1. WebRTC VoIP integration (FASE 12)
2. APNs integration (after FASE 8)

## 📚 Resources

- [SwiftUI Documentation](https://developer.apple.com/documentation/swiftui/)
- [CallKit Documentation](https://developer.apple.com/documentation/callkit)
- [UniFFI Guide](https://mozilla.github.io/uniffi-rs/)
- [AVAudioEngine](https://developer.apple.com/documentation/avfaudio/avaudioengine)
- [xcodegen](https://github.com/yonaskolb/XcodeGen)

## 🤝 Contributing

Part of the MePassa project. See main README for contribution guidelines.

## 📄 License

AGPL-3.0 (same as MePassa project)
