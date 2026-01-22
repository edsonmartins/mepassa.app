//
//  RemoteVideoView.swift
//  MePassa
//
//  Created on 2026-01-21
//  FASE 14: Videochamadas - Remote video rendering component
//

import SwiftUI
import AVFoundation

/// RemoteVideoView - Component for rendering remote video stream
///
/// Displays the remote peer's video feed using AVSampleBufferDisplayLayer.
/// For MVP, this shows a placeholder that will be connected to WebRTC video track.
struct RemoteVideoView: UIViewRepresentable {

    // MARK: - Properties

    let callId: String

    // MARK: - UIViewRepresentable

    func makeUIView(context: Context) -> RemoteVideoDisplayView {
        let view = RemoteVideoDisplayView(callId: callId)
        return view
    }

    func updateUIView(_ uiView: RemoteVideoDisplayView, context: Context) {
        // Update view if needed
    }
}

// MARK: - RemoteVideoDisplayView

/// UIView subclass that manages AVSampleBufferDisplayLayer for video rendering
class RemoteVideoDisplayView: UIView {

    // MARK: - Properties

    private let callId: String
    private var displayLayer: AVSampleBufferDisplayLayer?
    private var isVideoReceiving = false

    private let placeholderView: UIView = {
        let view = UIView()
        view.backgroundColor = .black
        return view
    }()

    private let iconImageView: UIImageView = {
        let imageView = UIImageView()
        let config = UIImage.SymbolConfiguration(pointSize: 64, weight: .regular)
        imageView.image = UIImage(systemName: "video.fill", withConfiguration: config)
        imageView.tintColor = UIColor.white.withAlphaComponent(0.5)
        imageView.contentMode = .scaleAspectFit
        return imageView
    }()

    private let messageLabel: UILabel = {
        let label = UILabel()
        label.text = "Waiting for video..."
        label.textColor = UIColor.white.withAlphaComponent(0.7)
        label.font = UIFont.systemFont(ofSize: 16, weight: .medium)
        label.textAlignment = .center
        return label
    }()

    // MARK: - Initialization

    init(callId: String) {
        self.callId = callId
        super.init(frame: .zero)
        setupUI()
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    // MARK: - Layout

    override func layoutSubviews() {
        super.layoutSubviews()

        placeholderView.frame = bounds

        // Center icon and label
        let iconSize: CGFloat = 64
        iconImageView.frame = CGRect(
            x: (bounds.width - iconSize) / 2,
            y: (bounds.height - iconSize) / 2 - 40,
            width: iconSize,
            height: iconSize
        )

        messageLabel.frame = CGRect(
            x: 0,
            y: iconImageView.frame.maxY + 16,
            width: bounds.width,
            height: 30
        )

        // Update display layer frame if exists
        displayLayer?.frame = bounds
    }

    // MARK: - Private Methods

    private func setupUI() {
        backgroundColor = .black

        // Add placeholder view
        addSubview(placeholderView)
        placeholderView.addSubview(iconImageView)
        placeholderView.addSubview(messageLabel)

        // Initialize display layer (hidden initially)
        setupDisplayLayer()

        // TODO: Register with MePassaCore to receive remote video frames
        // For MVP, this will show placeholder until video stream is received
    }

    private func setupDisplayLayer() {
        let layer = AVSampleBufferDisplayLayer()
        layer.videoGravity = .resizeAspect
        layer.frame = bounds

        // Set background color
        layer.backgroundColor = UIColor.black.cgColor

        self.layer.insertSublayer(layer, at: 0)
        displayLayer = layer

        print("📹 Remote video display layer initialized for call: \(callId)")
    }

    /// Enqueue video frame for rendering (to be called from FFI callback)
    ///
    /// - Parameter sampleBuffer: CMSampleBuffer containing video frame
    func enqueueVideoFrame(_ sampleBuffer: CMSampleBuffer) {
        guard let displayLayer = displayLayer else { return }

        // Show video, hide placeholder
        if !isVideoReceiving {
            DispatchQueue.main.async {
                self.isVideoReceiving = true
                self.placeholderView.isHidden = true
            }
        }

        // Check if display layer is ready
        guard displayLayer.isReadyForMoreMediaData else {
            print("⚠️ Display layer not ready for more data")
            return
        }

        // Enqueue sample buffer for display
        displayLayer.enqueue(sampleBuffer)
    }

    /// Flush display layer and reset to placeholder
    func resetVideo() {
        displayLayer?.flush()

        DispatchQueue.main.async {
            self.isVideoReceiving = false
            self.placeholderView.isHidden = false
        }

        print("📹 Remote video reset for call: \(callId)")
    }

    /// Update placeholder message
    ///
    /// - Parameter message: New message to display
    func updatePlaceholder(message: String) {
        DispatchQueue.main.async {
            self.messageLabel.text = message
        }
    }
}

// MARK: - Preview

#if DEBUG
struct RemoteVideoView_Previews: PreviewProvider {
    static var previews: some View {
        RemoteVideoView(callId: "test-call-id")
            .edgesIgnoringSafeArea(.all)
    }
}
#endif
