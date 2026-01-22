//
//  CameraManager.swift
//  MePassa
//
//  Created on 2026-01-21
//  FASE 14: Videochamadas - Camera capture with AVFoundation
//

import AVFoundation
import UIKit
import Combine

/// Camera Manager for video calls using AVFoundation
///
/// Manages camera capture, preview, and frame extraction for WebRTC video calls.
/// Supports front/back camera switching and frame callbacks for encoding.
class CameraManager: NSObject, ObservableObject {

    // MARK: - Properties

    private let captureSession = AVCaptureSession()
    private var videoOutput: AVCaptureVideoDataOutput?
    private var currentCamera: AVCaptureDevice?
    private var previewLayer: AVCaptureVideoPreviewLayer?

    @Published var cameraPosition: AVCaptureDevice.Position = .front
    @Published var isRunning = false

    private let videoQueue = DispatchQueue(label: "com.mepassa.videoQueue", qos: .userInitiated)

    private var frameCallback: ((CMSampleBuffer) -> Void)?

    // MARK: - Initialization

    override init() {
        super.init()
    }

    // MARK: - Public Methods

    /// Start camera capture with frame callback
    ///
    /// - Parameter onFrame: Callback for each captured frame (CMSampleBuffer)
    func startCapture(onFrame: @escaping (CMSampleBuffer) -> Void) {
        self.frameCallback = onFrame

        requestCameraPermission { [weak self] granted in
            guard granted else {
                print("❌ Camera permission denied")
                return
            }

            self?.setupCaptureSession()
        }
    }

    /// Stop camera capture and release resources
    func stopCapture() {
        captureSession.stopRunning()

        DispatchQueue.main.async {
            self.isRunning = false
        }

        frameCallback = nil

        print("📹 Camera stopped")
    }

    /// Switch between front and back camera
    func switchCamera() {
        let newPosition: AVCaptureDevice.Position = (cameraPosition == .front) ? .back : .front

        print("📹 Switching to \(newPosition == .front ? "FRONT" : "BACK") camera")

        captureSession.beginConfiguration()

        // Remove old inputs
        captureSession.inputs.forEach { captureSession.removeInput($0) }

        // Add new camera input
        guard let newCamera = AVCaptureDevice.default(
            .builtInWideAngleCamera,
            for: .video,
            position: newPosition
        ) else {
            print("❌ Camera not available for position: \(newPosition)")
            captureSession.commitConfiguration()
            return
        }

        do {
            let input = try AVCaptureDeviceInput(device: newCamera)
            if captureSession.canAddInput(input) {
                captureSession.addInput(input)
                currentCamera = newCamera

                DispatchQueue.main.async {
                    self.cameraPosition = newPosition
                }
            }
        } catch {
            print("❌ Switch camera failed: \(error)")
        }

        captureSession.commitConfiguration()
    }

    /// Get preview layer for displaying camera feed
    ///
    /// - Returns: AVCaptureVideoPreviewLayer for camera preview
    func getPreviewLayer() -> AVCaptureVideoPreviewLayer {
        if let existingLayer = previewLayer {
            return existingLayer
        }

        let layer = AVCaptureVideoPreviewLayer(session: captureSession)
        layer.videoGravity = .resizeAspectFill
        previewLayer = layer
        return layer
    }

    /// Check if camera is currently front-facing
    var isFrontCamera: Bool {
        return cameraPosition == .front
    }

    // MARK: - Private Methods

    private func setupCaptureSession() {
        captureSession.beginConfiguration()

        // Set preset (resolution) - VGA for MVP (640x480)
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

        // YUV420 format for WebRTC compatibility
        output.videoSettings = [
            kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
        ]

        // Discard frames if processing is too slow
        output.alwaysDiscardsLateVideoFrames = true

        if captureSession.canAddOutput(output) {
            captureSession.addOutput(output)
            videoOutput = output

            // Set video orientation
            if let connection = output.connection(with: .video) {
                if connection.isVideoOrientationSupported {
                    connection.videoOrientation = .portrait
                }

                // Mirror front camera
                if cameraPosition == .front && connection.isVideoMirroringSupported {
                    connection.isVideoMirrored = true
                }
            }
        }

        captureSession.commitConfiguration()

        // Start capture session on background queue
        videoQueue.async { [weak self] in
            self?.captureSession.startRunning()

            DispatchQueue.main.async {
                self?.isRunning = true
            }

            print("✅ Camera started successfully (lens: \(self?.cameraPosition == .front ? "FRONT" : "BACK"))")
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
}

// MARK: - AVCaptureVideoDataOutputSampleBufferDelegate

extension CameraManager: AVCaptureVideoDataOutputSampleBufferDelegate {

    func captureOutput(
        _ output: AVCaptureOutput,
        didOutput sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
        // Send frame to callback for transmission via FFI
        frameCallback?(sampleBuffer)
    }

    func captureOutput(
        _ output: AVCaptureOutput,
        didDrop sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
        // Frame dropped due to processing backlog
        // This is normal for video capture - just log if needed
    }
}
