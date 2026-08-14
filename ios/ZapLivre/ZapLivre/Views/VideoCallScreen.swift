import StreamWebRTC
import SwiftUI

struct VideoCallScreen: View {
    let callId: String
    let remotePeerId: String
    let peerName: String
    let onHangup: () -> Void

    @StateObject private var session: NativeWebRtcSession
    @State private var videoEnabled = true
    @State private var isMuted = false
    @State private var callDuration = 0

    private let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    init(callId: String, remotePeerId: String, peerName: String, onHangup: @escaping () -> Void) {
        self.callId = callId
        self.remotePeerId = remotePeerId
        self.peerName = peerName
        self.onHangup = onHangup
        _session = StateObject(
            wrappedValue: NativeWebRtcSession(callId: callId, remotePeerId: remotePeerId)
        )
    }

    var body: some View {
        ZStack {
            WebRtcVideoView(track: session.remoteVideoTrack, mirror: false)
                .background(Color.black)
                .ignoresSafeArea()

            if videoEnabled {
                WebRtcVideoView(track: session.localVideoTrack, mirror: true)
                    .frame(width: 120, height: 160)
                    .background(Color.black)
                    .clipShape(RoundedRectangle(cornerRadius: 12))
                    .overlay(
                        RoundedRectangle(cornerRadius: 12)
                            .stroke(Color.white.opacity(0.3), lineWidth: 2)
                    )
                    .padding()
                    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topTrailing)
            }

            VStack {
                Spacer()
                VStack(spacing: 16) {
                    Text(peerName)
                        .font(.title2.weight(.medium))
                        .foregroundColor(.white)
                    Text(formatDuration(callDuration))
                        .foregroundColor(.white.opacity(0.8))

                    HStack(spacing: 20) {
                        controlButton(
                            icon: videoEnabled ? "video.fill" : "video.slash.fill",
                            color: videoEnabled ? .blue : .red,
                            action: toggleVideo
                        )
                        controlButton(
                            icon: isMuted ? "mic.slash.fill" : "mic.fill",
                            color: isMuted ? .red : .blue,
                            action: toggleMute
                        )
                        controlButton(
                            icon: "arrow.triangle.2.circlepath.camera.fill",
                            color: .blue,
                            action: session.switchCamera
                        )
                        Button(action: hangup) {
                            Image(systemName: "phone.down.fill")
                                .font(.system(size: 28))
                                .frame(width: 72, height: 72)
                                .background(Color.red)
                                .foregroundColor(.white)
                                .clipShape(Circle())
                        }
                    }
                }
                .padding(.vertical, 24)
                .frame(maxWidth: .infinity)
                .background(Color.black.opacity(0.5).ignoresSafeArea(edges: .bottom))
            }
        }
        .onAppear { session.start() }
        .onDisappear { session.stop() }
        .onReceive(timer) { _ in callDuration += 1 }
    }

    private func controlButton(
        icon: String,
        color: Color,
        action: @escaping () -> Void
    ) -> some View {
        Button(action: action) {
            Image(systemName: icon)
                .font(.system(size: 24))
                .frame(width: 56, height: 56)
                .background(color)
                .foregroundColor(.white)
                .clipShape(Circle())
        }
    }

    private func toggleVideo() {
        videoEnabled.toggle()
        session.setVideoEnabled(videoEnabled)
    }

    private func toggleMute() {
        isMuted.toggle()
        session.setAudioEnabled(!isMuted)
    }

    private func hangup() {
        session.stop()
        Task { try? await ZapLivreCore.shared.hangupCall(callId: callId) }
        onHangup()
    }

    private func formatDuration(_ seconds: Int) -> String {
        let hours = seconds / 3600
        let minutes = (seconds % 3600) / 60
        let remaining = seconds % 60
        return hours > 0
            ? String(format: "%d:%02d:%02d", hours, minutes, remaining)
            : String(format: "%02d:%02d", minutes, remaining)
    }
}

private struct WebRtcVideoView: UIViewRepresentable {
    let track: RTCVideoTrack?
    let mirror: Bool

    func makeCoordinator() -> Coordinator { Coordinator() }

    func makeUIView(context: Context) -> RTCMTLVideoView {
        let view = RTCMTLVideoView(frame: .zero)
        view.videoContentMode = .scaleAspectFill
        view.transform = mirror ? CGAffineTransform(scaleX: -1, y: 1) : .identity
        context.coordinator.renderer = view
        context.coordinator.setTrack(track)
        return view
    }

    func updateUIView(_ view: RTCMTLVideoView, context: Context) {
        view.transform = mirror ? CGAffineTransform(scaleX: -1, y: 1) : .identity
        context.coordinator.setTrack(track)
    }

    static func dismantleUIView(_ view: RTCMTLVideoView, coordinator: Coordinator) {
        coordinator.setTrack(nil)
    }

    final class Coordinator {
        weak var renderer: RTCVideoRenderer?
        private var track: RTCVideoTrack?

        func setTrack(_ newTrack: RTCVideoTrack?) {
            guard track !== newTrack else { return }
            if let renderer { track?.remove(renderer) }
            track = newTrack
            if let renderer { newTrack?.add(renderer) }
        }
    }
}

#Preview {
    VideoCallScreen(
        callId: "test-call-id",
        remotePeerId: "remote-peer",
        peerName: "Test User",
        onHangup: {}
    )
}
