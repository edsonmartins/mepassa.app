package com.zaplivre.voip

import android.media.MediaCodec
import android.media.MediaCodecInfo
import android.media.MediaFormat
import android.os.Build
import android.util.Log
import java.nio.ByteBuffer

/**
 * VideoEncoder - Encodes YUV frames to H.264 using MediaCodec.
 *
 * MVP: expects NV21 input and outputs H.264 Annex B NAL units.
 */
class VideoEncoder(
    private val onEncoded: (ByteArray, Boolean, Int, Int) -> Unit
) {
    private var encoder: MediaCodec? = null
    private var configData: ByteArray? = null
    private var width: Int = 0
    private var height: Int = 0
    private var loggedFirstOutput = false
    private var lastInputPresentationTimeUs = 0L

    companion object {
        private const val TAG = "VideoEncoder"
        private const val MIME_TYPE = MediaFormat.MIMETYPE_VIDEO_AVC
        private const val BITRATE = 800_000
        private const val FPS = 15
        private const val IFRAME_INTERVAL = 2
        private const val TIMEOUT_US = 10_000L
    }

    private fun start(frameWidth: Int, frameHeight: Int) {
        if (encoder != null && width == frameWidth && height == frameHeight) return
        stop()
        width = frameWidth
        height = frameHeight

        val format = MediaFormat.createVideoFormat(MIME_TYPE, width, height).apply {
            setInteger(MediaFormat.KEY_COLOR_FORMAT, MediaCodecInfo.CodecCapabilities.COLOR_FormatYUV420Flexible)
            // Keep the encoder output consistent with the H.264 capability
            // negotiated by the Rust WebRTC peer (profile-level-id=42e01f).
            // Some devices, notably the Samsung tablet used in testing,
            // otherwise select High Profile while advertising Baseline in SDP.
            setInteger(MediaFormat.KEY_PROFILE, MediaCodecInfo.CodecProfileLevel.AVCProfileBaseline)
            setInteger(MediaFormat.KEY_LEVEL, MediaCodecInfo.CodecProfileLevel.AVCLevel31)
            setInteger(MediaFormat.KEY_BIT_RATE, BITRATE)
            setInteger(MediaFormat.KEY_FRAME_RATE, FPS)
            setInteger(MediaFormat.KEY_I_FRAME_INTERVAL, IFRAME_INTERVAL)
            if (Build.VERSION.SDK_INT >= 23) {
                setInteger("prepend-header-to-sync-frame", 1)
            }
        }

        encoder = MediaCodec.createEncoderByType(MIME_TYPE).apply {
            configure(format, null, null, MediaCodec.CONFIGURE_FLAG_ENCODE)
            start()
        }

        Log.i(TAG, "✅ Video encoder started (${width}x${height})")
    }

    fun stop() {
        try {
            encoder?.stop()
            encoder?.release()
            encoder = null
            configData = null
            loggedFirstOutput = false
            lastInputPresentationTimeUs = 0L
            Log.i(TAG, "🛑 Video encoder stopped")
        } catch (e: Exception) {
            Log.e(TAG, "❌ Failed to stop encoder", e)
        }
    }

    @Synchronized
    fun encodeFrame(
        nv21: ByteArray,
        frameWidth: Int,
        frameHeight: Int,
        presentationTimeUs: Long
    ) {
        if (frameWidth <= 0 || frameHeight <= 0 || frameWidth % 2 != 0 || frameHeight % 2 != 0) {
            Log.w(TAG, "Dropping invalid frame size ${frameWidth}x${frameHeight}")
            return
        }
        val minimumFrameIntervalUs = 1_000_000L / FPS
        if (lastInputPresentationTimeUs != 0L &&
            presentationTimeUs - lastInputPresentationTimeUs < minimumFrameIntervalUs
        ) {
            return
        }
        lastInputPresentationTimeUs = presentationTimeUs
        start(frameWidth, frameHeight)
        val codec = encoder ?: return
        val inputIndex = codec.dequeueInputBuffer(TIMEOUT_US)
        if (inputIndex >= 0) {
            val inputBuffer = codec.getInputBuffer(inputIndex)
            inputBuffer?.clear()

            val nv12 = nv21ToNv12(nv21)
            if (inputBuffer == null || nv12.size > inputBuffer.capacity()) {
                Log.w(TAG, "Dropping ${frameWidth}x${frameHeight} frame: ${nv12.size} bytes exceed encoder buffer ${inputBuffer?.capacity() ?: 0}")
                codec.queueInputBuffer(inputIndex, 0, 0, presentationTimeUs, 0)
                return
            }
            inputBuffer?.put(nv12)

            codec.queueInputBuffer(
                inputIndex,
                0,
                nv12.size,
                presentationTimeUs,
                0
            )
        }

        drainOutput(codec)
    }

    private fun drainOutput(codec: MediaCodec) {
        val bufferInfo = MediaCodec.BufferInfo()
        var outputIndex = codec.dequeueOutputBuffer(bufferInfo, TIMEOUT_US)
        while (outputIndex >= 0) {
            val outputBuffer = codec.getOutputBuffer(outputIndex)
            val outData = ByteArray(bufferInfo.size)
            outputBuffer?.get(outData)

            val isConfig = bufferInfo.flags and MediaCodec.BUFFER_FLAG_CODEC_CONFIG != 0
            val isKeyFrame = bufferInfo.flags and MediaCodec.BUFFER_FLAG_KEY_FRAME != 0

            if (isConfig) {
                configData = outData
            } else {
                val payload = if (isKeyFrame && configData != null) {
                    configData!! + outData
                } else {
                    outData
                }
                if (!loggedFirstOutput) {
                    val prefix = payload.take(8).joinToString(" ") { "%02x".format(it) }
                    Log.i(TAG, "First encoded frame: ${payload.size} bytes, key=$isKeyFrame, prefix=$prefix")
                    loggedFirstOutput = true
                }
                onEncoded(payload, isKeyFrame, width, height)
            }

            codec.releaseOutputBuffer(outputIndex, false)
            outputIndex = codec.dequeueOutputBuffer(bufferInfo, 0)
        }
    }

    private fun nv21ToNv12(nv21: ByteArray): ByteArray {
        val nv12 = nv21.clone()
        var i = width * height
        while (i + 1 < nv12.size) {
            val v = nv12[i]
            nv12[i] = nv12[i + 1]
            nv12[i + 1] = v
            i += 2
        }
        return nv12
    }
}
