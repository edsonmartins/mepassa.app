//
//  VideoFrameHandler.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import Foundation
import AVFoundation
import VideoToolbox
// Note: No need to import mepassa - the generated Swift code (mepassa.swift)
// is part of the same target. The bridging header already imports mepassaFFI.h

/// VideoFrameHandler - Implements FfiVideoFrameCallback for rendering remote video
///
/// Receives H.264 encoded frames from FFI, decodes them using VideoToolbox,
/// and renders to an AVSampleBufferDisplayLayer.
class VideoFrameHandler: FfiVideoFrameCallback {

    private let callId: String
    private var displayLayer: AVSampleBufferDisplayLayer?
    private var formatDescription: CMVideoFormatDescription?

    // Callback for frame rendered events
    var onFrameRendered: ((UInt32, UInt32) -> Void)?

    init(callId: String) {
        self.callId = callId
    }

    /// Set the display layer for video rendering
    ///
    /// - Parameter layer: AVSampleBufferDisplayLayer to render frames to
    func setDisplayLayer(_ layer: AVSampleBufferDisplayLayer) {
        self.displayLayer = layer

        // Configure display layer
        layer.videoGravity = .resizeAspect
        layer.preventsDisplaySleepDuringVideoPlayback = true

        print("✅ Display layer configured for call: \(callId)")
    }

    /// Called from FFI when a remote video frame is received
    ///
    /// - Parameters:
    ///   - callId: Call identifier
    ///   - frameData: Raw H.264 frame data (NALUs)
    ///   - width: Frame width in pixels
    ///   - height: Frame height in pixels
    func onVideoFrame(callId: String, frameData: [UInt8], width: UInt32, height: UInt32) {
        // Ignore frames from other calls
        guard callId == self.callId else { return }

        // Check if display layer is set
        guard let displayLayer = displayLayer else {
            print("⚠️ Display layer not set, skipping frame")
            return
        }

        // Create CMSampleBuffer from frame data
        guard let sampleBuffer = createSampleBuffer(from: frameData, width: width, height: height) else {
            print("❌ Failed to create sample buffer")
            return
        }

        // Enqueue sample buffer for rendering
        DispatchQueue.main.async { [weak self] in
            displayLayer.enqueue(sampleBuffer)

            // Notify callback
            self?.onFrameRendered?(width, height)
        }
    }

    /// Create CMSampleBuffer from H.264 frame data
    ///
    /// - Parameters:
    ///   - frameData: Raw H.264 NALU data
    ///   - width: Frame width
    ///   - height: Frame height
    /// - Returns: CMSampleBuffer ready for rendering, or nil if creation fails
    private func createSampleBuffer(from frameData: [UInt8], width: UInt32, height: UInt32) -> CMSampleBuffer? {
        // Create format description if not already created
        if formatDescription == nil {
            var formatDesc: CMFormatDescription?

            // SPS (Sequence Parameter Set) - simplified for VGA H.264
            let sps: [UInt8] = [0x67, 0x42, 0x00, 0x1e, 0xab, 0x40, 0x50, 0x1e, 0xd0, 0x0f, 0x08, 0x84, 0x6a]
            // PPS (Picture Parameter Set)
            let pps: [UInt8] = [0x68, 0xce, 0x3c, 0x80]

            let parameterSetPointers: [UnsafePointer<UInt8>] = [
                UnsafePointer<UInt8>(sps),
                UnsafePointer<UInt8>(pps)
            ]
            let parameterSetSizes: [Int] = [sps.count, pps.count]

            let status = CMVideoFormatDescriptionCreateFromH264ParameterSets(
                allocator: kCFAllocatorDefault,
                parameterSetCount: 2,
                parameterSetPointers: parameterSetPointers,
                parameterSetSizes: parameterSetSizes,
                nalUnitHeaderLength: 4,
                formatDescriptionOut: &formatDesc
            )

            guard status == noErr, let formatDesc = formatDesc else {
                print("❌ Failed to create format description: \(status)")
                return nil
            }

            formatDescription = formatDesc
        }

        // Create block buffer from frame data
        var blockBuffer: CMBlockBuffer?
        let dataPointer = UnsafeMutablePointer<UInt8>(mutating: frameData)

        var status = CMBlockBufferCreateWithMemoryBlock(
            allocator: kCFAllocatorDefault,
            memoryBlock: nil,
            blockLength: frameData.count,
            blockAllocator: kCFAllocatorDefault,
            customBlockSource: nil,
            offsetToData: 0,
            dataLength: frameData.count,
            flags: 0,
            blockBufferOut: &blockBuffer
        )

        guard status == kCMBlockBufferNoErr, let blockBuffer = blockBuffer else {
            print("❌ Failed to create block buffer: \(status)")
            return nil
        }

        // Copy frame data to block buffer
        status = CMBlockBufferReplaceDataBytes(
            with: dataPointer,
            blockBuffer: blockBuffer,
            offsetIntoDestination: 0,
            dataLength: frameData.count
        )

        guard status == kCMBlockBufferNoErr else {
            print("❌ Failed to copy data to block buffer: \(status)")
            return nil
        }

        // Create sample buffer
        var sampleBuffer: CMSampleBuffer?
        var timingInfo = CMSampleTimingInfo(
            duration: CMTime.invalid,
            presentationTimeStamp: CMClockGetTime(CMClockGetHostTimeClock()),
            decodeTimeStamp: CMTime.invalid
        )

        status = CMSampleBufferCreate(
            allocator: kCFAllocatorDefault,
            dataBuffer: blockBuffer,
            dataReady: true,
            makeDataReadyCallback: nil,
            refcon: nil,
            formatDescription: formatDescription,
            sampleCount: 1,
            sampleTimingEntryCount: 1,
            sampleTimingArray: &timingInfo,
            sampleSizeEntryCount: 0,
            sampleSizeArray: nil,
            sampleBufferOut: &sampleBuffer
        )

        guard status == noErr, let sampleBuffer = sampleBuffer else {
            print("❌ Failed to create sample buffer: \(status)")
            return nil
        }

        return sampleBuffer
    }

    /// Cleanup resources
    func release() {
        displayLayer = nil
        formatDescription = nil
        print("🧹 VideoFrameHandler released for call: \(callId)")
    }
}
