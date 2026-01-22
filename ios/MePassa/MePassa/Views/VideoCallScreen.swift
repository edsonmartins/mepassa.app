//
//  VideoCallScreen.swift
//  MePassa
//
//  Created on 2026-01-21
//  FASE 14: Videochamadas - Video call UI with SwiftUI
//

import SwiftUI
import AVFoundation

/// VideoCallScreen - Main UI for active video call
///
/// Displays:
/// - Remote video (fullscreen)
/// - Local camera preview (PiP in top-right corner)
/// - Call duration timer
/// - Control buttons: video toggle, mute, camera switch, hangup
struct VideoCallScreen: View {

    // MARK: - Properties

    let callId: String
    let peerName: String
    let onCallEnded: () -> Void

    @StateObject private var cameraManager = CameraManager()

    @State private var videoEnabled = true
    @State private var isMuted = false
    @State private var callDuration = 0
    @State private var isCallActive = true

    private let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    // MARK: - Body

    var body: some View {
        ZStack {
            // Remote video (fullscreen background)
            RemoteVideoView(callId: callId)
                .edgesIgnoringSafeArea(.all)

            // Local video preview (PiP - top right corner)
            if videoEnabled {
                LocalVideoPreview(cameraManager: cameraManager)
                    .frame(width: 120, height: 160)
                    .cornerRadius(12)
                    .overlay(
                        RoundedRectangle(cornerRadius: 12)
                            .stroke(Color.white.opacity(0.3), lineWidth: 2)
                    )
                    .shadow(color: Color.black.opacity(0.3), radius: 8, x: 0, y: 4)
                    .padding()
                    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topTrailing)
            }

            // Controls overlay
            VStack {
                Spacer()

                // Call info
                VStack(spacing: 8) {
                    // Peer name
                    Text(peerName.prefix(20))
                        .font(.title2)
                        .fontWeight(.medium)
                        .foregroundColor(.white)
                        .shadow(color: .black.opacity(0.5), radius: 4, x: 0, y: 2)

                    // Call duration
                    Text(formatDuration(callDuration))
                        .font(.body)
                        .foregroundColor(.white.opacity(0.9))
                        .shadow(color: .black.opacity(0.5), radius: 4, x: 0, y: 2)
                }
                .padding(.bottom, 24)

                // Control buttons row
                HStack(spacing: 20) {
                    // Video toggle button
                    Button(action: toggleVideo) {
                        Image(systemName: videoEnabled ? "video.fill" : "video.slash.fill")
                            .font(.system(size: 24))
                            .frame(width: 56, height: 56)
                            .background(videoEnabled ? Color.blue : Color.red)
                            .foregroundColor(.white)
                            .clipShape(Circle())
                            .shadow(color: .black.opacity(0.3), radius: 4, x: 0, y: 2)
                    }

                    // Mute toggle button
                    Button(action: toggleMute) {
                        Image(systemName: isMuted ? "mic.slash.fill" : "mic.fill")
                            .font(.system(size: 24))
                            .frame(width: 56, height: 56)
                            .background(isMuted ? Color.red : Color.blue)
                            .foregroundColor(.white)
                            .clipShape(Circle())
                            .shadow(color: .black.opacity(0.3), radius: 4, x: 0, y: 2)
                    }

                    // Switch camera button
                    Button(action: switchCamera) {
                        Image(systemName: "arrow.triangle.2.circlepath.camera.fill")
                            .font(.system(size: 24))
                            .frame(width: 56, height: 56)
                            .background(Color.blue)
                            .foregroundColor(.white)
                            .clipShape(Circle())
                            .shadow(color: .black.opacity(0.3), radius: 4, x: 0, y: 2)
                    }

                    // Hangup button (larger, more prominent)
                    Button(action: hangup) {
                        Image(systemName: "phone.down.fill")
                            .font(.system(size: 28))
                            .frame(width: 72, height: 72)
                            .background(Color(red: 0.9, green: 0.22, blue: 0.21)) // Red #E53935
                            .foregroundColor(.white)
                            .clipShape(Circle())
                            .shadow(color: .black.opacity(0.3), radius: 4, x: 0, y: 2)
                    }
                }
                .padding(.bottom, 40)
            }
            .frame(maxWidth: .infinity)
            .background(
                LinearGradient(
                    gradient: Gradient(colors: [
                        Color.black.opacity(0),
                        Color.black.opacity(0.7)
                    ]),
                    startPoint: .top,
                    endPoint: .bottom
                )
            )
        }
        .onAppear {
            startVideo()
        }
        .onDisappear {
            stopVideo()
        }
        .onReceive(timer) { _ in
            if isCallActive {
                callDuration += 1
            }
        }
    }

    // MARK: - Private Methods

    private func startVideo() {
        guard videoEnabled else { return }

        cameraManager.startCapture { sampleBuffer in
            // Extract frame data from CMSampleBuffer
            guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }

            // Convert CVPixelBuffer to byte array
            CVPixelBufferLockBaseAddress(pixelBuffer, .readOnly)
            defer { CVPixelBufferUnlockBaseAddress(pixelBuffer, .readOnly) }

            let width = CVPixelBufferGetWidth(pixelBuffer)
            let height = CVPixelBufferGetHeight(pixelBuffer)

            // Get Y plane (luminance) for YUV420
            guard let baseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 0) else { return }
            let bytesPerRow = CVPixelBufferGetBytesPerRowOfPlane(pixelBuffer, 0)
            let bufferSize = bytesPerRow * height

            let data = Data(bytes: baseAddress, count: bufferSize)

            // Send frame to MePassaCore via FFI
            Task {
                do {
                    try await MePassaCore.shared.sendVideoFrame(
                        callId: callId,
                        frameData: [UInt8](data),
                        width: UInt32(width),
                        height: UInt32(height)
                    )
                } catch {
                    // Log error but don't crash
                    print("❌ Failed to send video frame: \(error)")
                }
            }
        }
    }

    private func stopVideo() {
        cameraManager.stopCapture()
    }

    private func toggleVideo() {
        videoEnabled.toggle()

        Task {
            do {
                if videoEnabled {
                    try await MePassaCore.shared.enableVideo(callId: callId, codec: .h264)
                    startVideo()
                } else {
                    try await MePassaCore.shared.disableVideo(callId: callId)
                    stopVideo()
                }
            } catch {
                print("❌ Failed to toggle video: \(error)")
            }
        }
    }

    private func toggleMute() {
        isMuted.toggle()

        Task {
            do {
                try await MePassaCore.shared.toggleMute(callId: callId)
            } catch {
                print("❌ Failed to toggle mute: \(error)")
            }
        }
    }

    private func switchCamera() {
        cameraManager.switchCamera()
    }

    private func hangup() {
        isCallActive = false

        Task {
            do {
                try await MePassaCore.shared.hangupCall(callId: callId)
                stopVideo()
                onCallEnded()
            } catch {
                print("❌ Failed to hangup call: \(error)")
            }
        }
    }

    private func formatDuration(_ seconds: Int) -> String {
        let minutes = seconds / 60
        let secs = seconds % 60
        return String(format: "%02d:%02d", minutes, secs)
    }
}

// MARK: - Local Video Preview Component

/// LocalVideoPreview - Displays local camera feed using AVCaptureVideoPreviewLayer
struct LocalVideoPreview: UIViewRepresentable {

    @ObservedObject var cameraManager: CameraManager

    func makeUIView(context: Context) -> UIView {
        let view = UIView()
        view.backgroundColor = .black

        let previewLayer = cameraManager.getPreviewLayer()
        previewLayer.frame = view.bounds
        view.layer.addSublayer(previewLayer)

        return view
    }

    func updateUIView(_ uiView: UIView, context: Context) {
        // Update preview layer frame on size change
        if let previewLayer = uiView.layer.sublayers?.first as? AVCaptureVideoPreviewLayer {
            DispatchQueue.main.async {
                previewLayer.frame = uiView.bounds
            }
        }
    }
}

// MARK: - Preview

#if DEBUG
struct VideoCallScreen_Previews: PreviewProvider {
    static var previews: some View {
        VideoCallScreen(
            callId: "test-call-id",
            peerName: "Alice",
            onCallEnded: {}
        )
    }
}
#endif
