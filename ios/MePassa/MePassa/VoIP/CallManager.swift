//
//  CallManager.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import Foundation
import CallKit
import AVFoundation
import Combine

class CallManager: NSObject, ObservableObject {
    // MARK: - Published Properties
    @Published var currentCall: Call?
    @Published var callState: CallState = .idle
    @Published var isMuted = false
    @Published var isSpeakerOn = false

    // MARK: - CallKit
    private let callController = CXCallController()
    private var provider: CXProvider?

    // MARK: - Audio
    private let audioEngine = AVAudioEngine()
    private var audioSession: AVAudioSession {
        AVAudioSession.sharedInstance()
    }

    // MARK: - Configuration
    override init() {
        super.init()
        configureCallKit()
    }

    func configure() {
        // Additional configuration if needed
        print("📞 CallManager configured")
    }

    // MARK: - CallKit Configuration
    private func configureCallKit() {
        let configuration = CXProviderConfiguration(localizedName: "MePassa")
        configuration.supportsVideo = false // TODO: Enable for FASE 14 (video calls)
        configuration.maximumCallGroups = 1
        configuration.maximumCallsPerCallGroup = 1
        configuration.supportedHandleTypes = [.generic]

        // Audio
        configuration.ringtoneSound = "ringtone.caf"

        provider = CXProvider(configuration: configuration)
        provider?.setDelegate(self, queue: nil)

        print("✅ CallKit provider configured")
    }

    // MARK: - Outgoing Call
    func startCall(to peerId: String, displayName: String) {
        let handle = CXHandle(type: .generic, value: peerId)
        let startCallAction = CXStartCallAction(call: UUID(), handle: handle)

        let transaction = CXTransaction(action: startCallAction)

        callController.request(transaction) { [weak self] error in
            if let error = error {
                print("❌ Error requesting start call: \(error)")
                return
            }

            print("✅ Start call requested")
            self?.createCall(id: startCallAction.callUUID, peerId: peerId, displayName: displayName, isOutgoing: true)
        }
    }

    // MARK: - Incoming Call
    func reportIncomingCall(callId: UUID, peerId: String, displayName: String, completion: @escaping (Error?) -> Void) {
        let update = CXCallUpdate()
        update.remoteHandle = CXHandle(type: .generic, value: peerId)
        update.localizedCallerName = displayName
        update.supportsHolding = false
        update.supportsGrouping = false
        update.supportsUngrouping = false
        update.supportsDTMF = false
        update.hasVideo = false

        provider?.reportNewIncomingCall(with: callId, update: update) { [weak self] error in
            if let error = error {
                print("❌ Error reporting incoming call: \(error)")
                completion(error)
                return
            }

            print("✅ Incoming call reported")
            self?.createCall(id: callId, peerId: peerId, displayName: displayName, isOutgoing: false)
            completion(nil)
        }
    }

    // MARK: - Call Management
    private func createCall(id: UUID, peerId: String, displayName: String, isOutgoing: Bool) {
        let call = Call(
            id: id,
            peerId: peerId,
            displayName: displayName,
            isOutgoing: isOutgoing
        )

        DispatchQueue.main.async {
            self.currentCall = call
            self.callState = isOutgoing ? .connecting : .ringing
        }

        // TODO: Connect to VoIP engine via UniFFI
        if isOutgoing {
            initiateWebRTCConnection(peerId: peerId)
        }
    }

    func answerCall() {
        guard let call = currentCall else { return }

        let answerAction = CXAnswerCallAction(call: call.id)
        let transaction = CXTransaction(action: answerAction)

        callController.request(transaction) { error in
            if let error = error {
                print("❌ Error answering call: \(error)")
                return
            }

            print("✅ Call answered")
        }
    }

    func endCall() {
        guard let call = currentCall else { return }

        let endCallAction = CXEndCallAction(call: call.id)
        let transaction = CXTransaction(action: endCallAction)

        callController.request(transaction) { [weak self] error in
            if let error = error {
                print("❌ Error ending call: \(error)")
                return
            }

            print("✅ Call ended")
            self?.cleanupCall()
        }
    }

    private func cleanupCall() {
        // TODO: Disconnect WebRTC via UniFFI
        stopAudio()

        DispatchQueue.main.async {
            self.currentCall = nil
            self.callState = .idle
            self.isMuted = false
            self.isSpeakerOn = false
        }
    }

    // MARK: - Audio Controls
    func toggleMute() {
        isMuted.toggle()

        // TODO: Mute/unmute via UniFFI WebRTC
        print("🔇 Mute: \(isMuted)")
    }

    func toggleSpeaker() {
        isSpeakerOn.toggle()

        do {
            if isSpeakerOn {
                try audioSession.overrideOutputAudioPort(.speaker)
            } else {
                try audioSession.overrideOutputAudioPort(.none)
            }
            print("🔊 Speaker: \(isSpeakerOn)")
        } catch {
            print("❌ Error toggling speaker: \(error)")
        }
    }

    // MARK: - WebRTC Integration (TODO)
    private func initiateWebRTCConnection(peerId: String) {
        // TODO: Call UniFFI to start WebRTC connection
        // This will integrate with core/src/voip/engine.rs

        print("📞 Initiating WebRTC connection to \(peerId)...")

        // Simulate connection delay
        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            self.callState = .connected
            self.startAudio()
        }
    }

    private func startAudio() {
        do {
            try audioSession.setCategory(.playAndRecord, mode: .voiceChat, options: [.allowBluetooth, .defaultToSpeaker])
            try audioSession.setActive(true)

            // TODO: Connect AVAudioEngine to WebRTC audio tracks
            // This will use AVAudioEngine for audio I/O similar to Android's CallAudioManager

            print("🎤 Audio session started")
        } catch {
            print("❌ Error starting audio session: \(error)")
        }
    }

    private func stopAudio() {
        do {
            try audioSession.setActive(false)
            print("🎤 Audio session stopped")
        } catch {
            print("❌ Error stopping audio session: \(error)")
        }
    }
}

// MARK: - CXProviderDelegate
extension CallManager: CXProviderDelegate {
    func providerDidReset(_ provider: CXProvider) {
        print("📞 Provider reset")
        cleanupCall()
    }

    func provider(_ provider: CXProvider, perform action: CXStartCallAction) {
        configureAudioSession()
        action.fulfill()
    }

    func provider(_ provider: CXProvider, perform action: CXAnswerCallAction) {
        guard let call = currentCall else {
            action.fail()
            return
        }

        configureAudioSession()

        // TODO: Accept call via UniFFI WebRTC
        print("✅ Accepting call...")

        DispatchQueue.main.async {
            self.callState = .connected
        }

        startAudio()
        action.fulfill()
    }

    func provider(_ provider: CXProvider, perform action: CXEndCallAction) {
        cleanupCall()
        action.fulfill()
    }

    func provider(_ provider: CXProvider, perform action: CXSetMutedCallAction) {
        isMuted = action.isMuted
        // TODO: Apply mute to WebRTC
        action.fulfill()
    }

    func provider(_ provider: CXProvider, didActivate audioSession: AVAudioSession) {
        print("🎤 Audio session activated")
        // TODO: Start audio I/O with AVAudioEngine
    }

    func provider(_ provider: CXProvider, didDeactivate audioSession: AVAudioSession) {
        print("🎤 Audio session deactivated")
    }

    private func configureAudioSession() {
        do {
            try audioSession.setCategory(.playAndRecord, mode: .voiceChat)
            try audioSession.setActive(true)
        } catch {
            print("❌ Error configuring audio session: \(error)")
        }
    }
}

// MARK: - Models
struct Call: Identifiable {
    let id: UUID
    let peerId: String
    let displayName: String
    let isOutgoing: Bool
    var startTime: Date = Date()
}

enum CallState {
    case idle
    case ringing
    case connecting
    case connected
    case ended
}
