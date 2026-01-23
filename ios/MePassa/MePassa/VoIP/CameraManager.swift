//
//  CameraManager.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import AVFoundation
import UIKit

/// CameraManager - Manages camera capture for video calls using AVFoundation
class CameraManager: NSObject, ObservableObject {
    
    // MARK: - Properties
    
    private let captureSession = AVCaptureSession()
    private var videoOutput: AVCaptureVideoDataOutput?
    private var currentCamera: AVCaptureDevice?
    private var previewLayer: AVCaptureVideoPreviewLayer?
    
    @Published var cameraPosition: AVCaptureDevice.Position = .front
    @Published var isCapturing: Bool = false
    
    private let videoQueue = DispatchQueue(label: "com.mepassa.videoQueue")
    private var onFrameCallback: ((CMSampleBuffer) -> Void)?
    
    // MARK: - Initialization
    
    override init() {
        super.init()
    }
    
    // MARK: - Camera Control
    
    /// Start camera capture
    /// - Parameter onFrame: Callback for each captured frame
    func startCapture(onFrame: @escaping (CMSampleBuffer) -> Void) {
        onFrameCallback = onFrame
        
        requestCameraPermission { [weak self] granted in
            guard granted else {
                print("❌ Camera permission denied")
                return
            }
            
            self?.setupCaptureSession()
        }
    }
    
    /// Stop camera capture
    func stopCapture() {
        if captureSession.isRunning {
            captureSession.stopRunning()
        }
        isCapturing = false
        print("🛑 Camera capture stopped")
    }
    
    /// Switch camera (front ↔ back)
    func switchCamera() {
        cameraPosition = (cameraPosition == .front) ? .back : .front
        
        // Reconfigure session with new camera
        captureSession.beginConfiguration()
        
        // Remove old inputs
        captureSession.inputs.forEach { captureSession.removeInput($0) }
        
        // Add new camera input
        guard let newCamera = AVCaptureDevice.default(
            .builtInWideAngleCamera,
            for: .video,
            position: cameraPosition
        ) else {
            captureSession.commitConfiguration()
            return
        }
        
        do {
            let input = try AVCaptureDeviceInput(device: newCamera)
            if captureSession.canAddInput(input) {
                captureSession.addInput(input)
                currentCamera = newCamera
            }
        } catch {
            print("❌ Switch camera failed: \(error)")
        }
        
        captureSession.commitConfiguration()
        
        print("📷 Camera switched to \(cameraPosition == .front ? "FRONT" : "BACK")")
    }
    
    // MARK: - Preview Layer
    
    /// Get preview layer for displaying camera feed
    func getPreviewLayer() -> AVCaptureVideoPreviewLayer {
        if let existingLayer = previewLayer {
            return existingLayer
        }
        
        let layer = AVCaptureVideoPreviewLayer(session: captureSession)
        layer.videoGravity = .resizeAspectFill
        previewLayer = layer
        return layer
    }
    
    // MARK: - Private Methods
    
    private func setupCaptureSession() {
        captureSession.beginConfiguration()
        
        // Set preset (resolution)
        captureSession.sessionPreset = .vga640x480
        
        // Camera input
        guard let camera = AVCaptureDevice.default(
            .builtInWideAngleCamera,
            for: .video,
            position: cameraPosition
        ) else {
            print("❌ No camera available")
            captureSession.commitConfiguration()
            return
        }
        
        currentCamera = camera
        
        do {
            let input = try AVCaptureDeviceInput(device: camera)
            if captureSession.canAddInput(input) {
                captureSession.addInput(input)
            }
        } catch {
            print("❌ Camera input failed: \(error)")
            captureSession.commitConfiguration()
            return
        }
        
        // Video output
        let output = AVCaptureVideoDataOutput()
        output.setSampleBufferDelegate(self, queue: videoQueue)
        output.videoSettings = [
            kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
        ]
        
        if captureSession.canAddOutput(output) {
            captureSession.addOutput(output)
            videoOutput = output
        }
        
        captureSession.commitConfiguration()
        
        // Start capture
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            self?.captureSession.startRunning()
            
            DispatchQueue.main.async {
                self?.isCapturing = true
                print("✅ Camera capture started")
            }
        }
    }
    
    private func requestCameraPermission(completion: @escaping (Bool) -> Void) {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized:
            completion(true)
            
        case .notDetermined:
            AVCaptureDevice.requestAccess(for: .video) { granted in
                DispatchQueue.main.async {
                    completion(granted)
                }
            }
            
        case .denied, .restricted:
            completion(false)
            
        @unknown default:
            completion(false)
        }
    }
    
    // MARK: - Cleanup
    
    deinit {
        stopCapture()
    }
}

// MARK: - AVCaptureVideoDataOutputSampleBufferDelegate

extension CameraManager: AVCaptureVideoDataOutputSampleBufferDelegate {
    
    func captureOutput(
        _ output: AVCaptureOutput,
        didOutput sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
        // Send frame to callback
        onFrameCallback?(sampleBuffer)
        
        // TODO: Convert CMSampleBuffer to byte array and send via FFI
        // Example:
        // guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else { return }
        // let data = convertPixelBufferToByteArray(pixelBuffer)
        // MePassaCore.shared.sendVideoFrame(callId: callId, data: data, width: width, height: height)
    }
    
    func captureOutput(
        _ output: AVCaptureOutput,
        didDrop sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
        // Frame dropped - can be used for statistics
        print("⚠️ Video frame dropped")
    }
}
