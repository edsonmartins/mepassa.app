//
//  RemoteVideoView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI
import AVFoundation

/// RemoteVideoView - Renders remote video using AVSampleBufferDisplayLayer
struct RemoteVideoView: UIViewRepresentable {
    let callId: String
    
    func makeUIView(context: Context) -> UIView {
        let view = UIView()
        view.backgroundColor = .black
        
        // Create AVSampleBufferDisplayLayer for rendering remote video
        let displayLayer = AVSampleBufferDisplayLayer()
        displayLayer.videoGravity = .resizeAspect
        displayLayer.frame = view.bounds
        
        view.layer.addSublayer(displayLayer)
        
        // Store layer reference in context
        context.coordinator.displayLayer = displayLayer
        
        // TODO: Register with MePassaCore to receive remote video frames
        // Example:
        // MePassaCore.shared.setRemoteVideoCallback(callId: callId) { sampleBuffer in
        //     displayLayer.enqueue(sampleBuffer)
        // }
        
        return view
    }
    
    func updateUIView(_ uiView: UIView, context: Context) {
        // Update layer frame on size change
        if let displayLayer = context.coordinator.displayLayer {
            DispatchQueue.main.async {
                displayLayer.frame = uiView.bounds
            }
        }
    }
    
    func makeCoordinator() -> Coordinator {
        Coordinator()
    }
    
    class Coordinator {
        var displayLayer: AVSampleBufferDisplayLayer?
    }
}

// MARK: - Alternative Implementation with Placeholder

/// RemoteVideoView with placeholder (for MVP/testing without actual video stream)
struct RemoteVideoViewPlaceholder: View {
    let callId: String
    
    var body: some View {
        ZStack {
            Color.black
            
            VStack(spacing: 16) {
                Image(systemName: "video.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.white.opacity(0.3))
                
                Text("Remote Video")
                    .font(.body)
                    .foregroundColor(.white.opacity(0.5))
                
                Text("Call ID: \(callId.prefix(8))...")
                    .font(.caption2)
                    .foregroundColor(.white.opacity(0.3))
            }
        }
    }
}

// MARK: - Preview

#Preview {
    RemoteVideoView(callId: "test-call-id")
        .frame(width: 300, height: 400)
}
