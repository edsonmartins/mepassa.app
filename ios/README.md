# MePassa iOS App

iOS native app for MePassa P2P messaging platform built with SwiftUI and CallKit.

## 📋 Requirements

- Xcode 15.0+
- iOS 15.0+
- macOS for development
- Rust toolchain (for building core library)
- uniffi-bindgen 0.31.0

## 🏗️ Project Structure

```
ios/MePassa/
├── MePassaApp.swift          # App entry point
├── ContentView.swift          # Main navigation
├── Info.plist                 # App configuration & permissions
├── Views/                     # SwiftUI screens
│   ├── LoginView.swift
│   ├── ConversationsView.swift
│   ├── ChatView.swift
│   ├── CallScreen.swift
│   ├── IncomingCallScreen.swift
│   ├── NewChatView.swift
│   ├── SettingsView.swift
│   ├── QRScannerView.swift
│   └── MyQRCodeView.swift
├── VoIP/                      # VoIP integration
│   └── CallManager.swift      # CallKit integration
└── Generated/                 # UniFFI generated bindings (not in git)
    └── mepassa.swift          # Generated from core/src/mepassa.udl
```

## 🔧 Setup

### 1. Build Core Library

The iOS app depends on the Rust core library compiled for iOS targets.

```bash
# Install iOS targets
rustup target add aarch64-apple-ios      # iOS devices (ARM64)
rustup target add x86_64-apple-ios       # iOS Simulator (Intel)
rustup target add aarch64-apple-ios-sim  # iOS Simulator (Apple Silicon)

# Build for all iOS targets
cd ../core
cargo build --release --target aarch64-apple-ios
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
```

### 2. Generate Swift Bindings

```bash
# Install uniffi-bindgen (if not already installed)
cargo install uniffi-bindgen --version 0.31.0

# Run the binding generation script
cd ios
./generate_bindings.sh
```

This will:
1. Build the core library for macOS
2. Generate Swift bindings from `core/src/mepassa.udl`
3. Output files to `ios/MePassa/Generated/`

Generated files:
- `mepassa.swift` - Swift interfaces and types
- `mepassaFFI.h` - C header for FFI
- `mepassaFFI.modulemap` - Module map

### 3. Configure Xcode Project

1. **Create Xcode Project**:
   - Open Xcode
   - Create new iOS App project
   - Name: "MePassa"
   - Interface: SwiftUI
   - Language: Swift

2. **Add Source Files**:
   - Drag all `.swift` files from `ios/MePassa/` into Xcode
   - Ensure "Copy items if needed" is unchecked (files are already in place)
   - Add `Info.plist` to project

3. **Add Generated Bindings**:
   - Drag `ios/MePassa/Generated/` folder into Xcode
   - Ensure "Create groups" is selected

4. **Add Core Library**:
   - Create "Frameworks" group in Xcode
   - Add `libmepassa_core.a` for each target:
     - iOS Device: `../target/aarch64-apple-ios/release/libmepassa_core.a`
     - iOS Simulator (Intel): `../target/x86_64-apple-ios/release/libmepassa_core.a`
     - iOS Simulator (Apple Silicon): `../target/aarch64-apple-ios-sim/release/libmepassa_core.a`

5. **Configure Build Settings**:
   - In Build Settings, search for "Library Search Paths"
   - Add: `$(PROJECT_DIR)/../target/$(PLATFORM_NAME)/release`
   - In "Other Linker Flags", add: `-lmepassa_core`

6. **Configure Capabilities**:
   - Enable "Background Modes":
     - Voice over IP
     - Remote notifications
     - Audio, AirPlay, and Picture in Picture
   - Enable "Push Notifications"

### 4. Configure Signing & Provisioning

1. Select your Apple Developer account in Xcode
2. Configure Bundle Identifier: `app.mepassa.ios` (or your preference)
3. Enable automatic signing or configure provisioning profiles

## 🎯 Features

### Implemented (FASE 13 - 50%)

- ✅ SwiftUI app structure with navigation
- ✅ Login/identity generation UI
- ✅ Conversations list
- ✅ Chat screen with messaging UI
- ✅ CallKit integration (CallManager)
- ✅ VoIP call screens (incoming/active)
- ✅ Settings and profile screens
- ✅ QR code generation for identity sharing

### TODO (FASE 13 - 50%)

- ⏳ UniFFI bindings integration (in progress)
- ⏳ Audio I/O with AVAudioEngine
- ⏳ WebRTC integration for VoIP
- ⏳ APNs (Push Notifications) integration
- ⏳ QR code scanner implementation
- ⏳ Xcode project file configuration
- ⏳ Build pipeline & TestFlight setup

## 📱 Permissions

The app requests the following permissions (configured in Info.plist):

- **Microphone** (`NSMicrophoneUsageDescription`): For voice calls
- **Camera** (`NSCameraUsageDescription`): For video calls (FASE 14)
- **Photos** (`NSPhotoLibraryUsageDescription`): To share images
- **Contacts** (`NSContactsUsageDescription`): To find friends

## 🔊 VoIP Integration

### CallKit

The app uses CallKit for native iOS call integration:

- **CallManager.swift**: Manages CallKit provider and call controller
- **CXProvider**: Handles system call UI and events
- **CXCallController**: Controls call actions (answer, end, mute)
- **Background Modes**: Configured for VoIP, remote notifications, and audio

### Audio I/O (TODO)

Will use AVAudioEngine for audio capture and playback:
- Capture microphone input
- Process audio through WebRTC
- Playback remote audio
- Handle audio routing (speaker, Bluetooth, earpiece)

## 🏗️ Architecture

### State Management

- **AppState**: Global app state (authentication, user, conversations)
  - Published properties trigger UI updates
  - ObservableObject pattern
  - Injected via @EnvironmentObject

- **CallManager**: VoIP state and CallKit integration
  - Call state management
  - Audio session configuration
  - CallKit delegate implementation

### Navigation Flow

```
ContentView
├── LoginView (if !authenticated)
└── ConversationsView (if authenticated)
    ├── ChatView (per conversation)
    │   ├── Start voice call → CallScreen
    │   └── Start video call → (FASE 14)
    ├── NewChatView (modal)
    └── SettingsView (modal)

IncomingCallScreen (presented by CallKit)
└── Answer → CallScreen
```

### FFI Integration (TODO)

```swift
import mepassa  // Generated by UniFFI

// Initialize core
let client = try MePassaClient(dataDir: documentsPath)

// Get local peer ID
let peerId = try await client.localPeerId()

// Send message
let messageId = try await client.sendTextMessage(
    toPeerId: recipientPeerId,
    content: "Hello!"
)

// Start call
let callId = try await client.startCall(toPeerId: recipientPeerId)
```

## 🧪 Testing

### Unit Tests (TODO)

```bash
xcodebuild test \
    -scheme MePassa \
    -destination 'platform=iOS Simulator,name=iPhone 15'
```

### UI Tests (TODO)

SwiftUI Preview providers are included for all views for rapid UI iteration.

## 📦 Build & Deploy

### Development Build

```bash
xcodebuild \
    -scheme MePassa \
    -configuration Debug \
    -destination 'platform=iOS Simulator,name=iPhone 15'
```

### Release Build

```bash
xcodebuild \
    -scheme MePassa \
    -configuration Release \
    -archivePath ./build/MePassa.xcarchive \
    archive
```

### TestFlight (TODO)

1. Archive the app in Xcode
2. Upload to App Store Connect
3. Configure TestFlight metadata
4. Add internal/external testers
5. Distribute build

## 🔄 Continuous Integration (TODO)

GitHub Actions workflow for:
- Build verification
- Unit tests
- UI tests
- TestFlight beta deployment

## 📝 Development Notes

### Current Status (2026-01-20)

FASE 13 iOS App: ~50% complete

**Completed**:
- ✅ All SwiftUI screens created
- ✅ CallKit integration (CallManager)
- ✅ App structure and navigation
- ✅ Info.plist configuration

**In Progress**:
- 🔧 UniFFI bindings generation
- 🔧 Xcode project configuration

**Pending**:
- ⏳ AVAudioEngine audio I/O
- ⏳ WebRTC VoIP integration
- ⏳ APNs push notifications
- ⏳ QR scanner (AVFoundation)
- ⏳ Build pipeline

### Known Issues

1. **UniFFI Bindings**: Manual generation required until automated in build pipeline
2. **QR Scanner**: Placeholder UI - needs AVFoundation implementation
3. **WebRTC**: Core library ready, needs Swift integration
4. **APNs**: Waiting on FASE 8 completion (server-side)

### Next Steps

1. Generate UniFFI bindings successfully
2. Create Xcode project file (.xcodeproj)
3. Integrate mepassa-core library
4. Implement AVAudioEngine audio I/O
5. Connect VoIP UI to WebRTC engine
6. Test on physical iOS device
7. Configure APNs certificates
8. Set up TestFlight

## 📚 Resources

- [SwiftUI Documentation](https://developer.apple.com/documentation/swiftui/)
- [CallKit Documentation](https://developer.apple.com/documentation/callkit)
- [UniFFI Guide](https://mozilla.github.io/uniffi-rs/)
- [AVAudioEngine](https://developer.apple.com/documentation/avfaudio/avaudioengine)
- [WebRTC iOS](https://webrtc.github.io/webrtc-org/native-code/ios/)

## 🤝 Contributing

This is part of the MePassa project. See main README for contribution guidelines.

## 📄 License

Same as MePassa project license.
