package com.zaplivre.voip

import android.Manifest
import android.annotation.SuppressLint
import android.content.Context
import android.content.pm.PackageManager
import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.AudioTrack
import android.media.MediaRecorder
import android.util.Log
import androidx.core.content.ContextCompat
import com.zaplivre.core.ZapLivreClientWrapper
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import uniffi.zaplivre.FfiAudioFrameCallback

/** Bridges Android's microphone/speaker PCM stream to the Rust WebRTC core. */
@OptIn(ExperimentalUnsignedTypes::class)
object VoipAudioBridge : FfiAudioFrameCallback {
    private const val TAG = "VoipAudioBridge"
    private const val SAMPLE_RATE = 48_000
    private const val FRAME_SAMPLES = 960 // 20 ms, matching Opus' default frame size
    private const val FRAME_BYTES = FRAME_SAMPLES * 2

    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val lock = Any()
    private var activeCallId: String? = null
    private val activeOwners = mutableSetOf<String>()
    private var captureJob: Job? = null
    private var recorder: AudioRecord? = null
    private var player: AudioTrack? = null

    @SuppressLint("MissingPermission")
    fun start(context: Context, callId: String, owner: String) {
        if (ContextCompat.checkSelfPermission(context, Manifest.permission.RECORD_AUDIO) !=
            PackageManager.PERMISSION_GRANTED
        ) {
            Log.e(TAG, "Cannot start audio: RECORD_AUDIO is not granted")
            return
        }

        synchronized(lock) {
            if (activeCallId == callId && captureJob?.isActive == true) {
                activeOwners += owner
                return
            }
            stopLocked()
            activeOwners += owner

            val recordBuffer = maxOf(
                FRAME_BYTES * 4,
                AudioRecord.getMinBufferSize(
                    SAMPLE_RATE,
                    AudioFormat.CHANNEL_IN_MONO,
                    AudioFormat.ENCODING_PCM_16BIT
                )
            )
            val playbackBuffer = maxOf(
                FRAME_BYTES * 4,
                AudioTrack.getMinBufferSize(
                    SAMPLE_RATE,
                    AudioFormat.CHANNEL_OUT_MONO,
                    AudioFormat.ENCODING_PCM_16BIT
                )
            )

            val newRecorder = AudioRecord(
                MediaRecorder.AudioSource.VOICE_COMMUNICATION,
                SAMPLE_RATE,
                AudioFormat.CHANNEL_IN_MONO,
                AudioFormat.ENCODING_PCM_16BIT,
                recordBuffer
            )
            val newPlayer = AudioTrack.Builder()
                .setAudioAttributes(
                    AudioAttributes.Builder()
                        .setUsage(AudioAttributes.USAGE_VOICE_COMMUNICATION)
                        .setContentType(AudioAttributes.CONTENT_TYPE_SPEECH)
                        .build()
                )
                .setAudioFormat(
                    AudioFormat.Builder()
                        .setEncoding(AudioFormat.ENCODING_PCM_16BIT)
                        .setSampleRate(SAMPLE_RATE)
                        .setChannelMask(AudioFormat.CHANNEL_OUT_MONO)
                        .build()
                )
                .setBufferSizeInBytes(playbackBuffer)
                .setTransferMode(AudioTrack.MODE_STREAM)
                .build()

            if (newRecorder.state != AudioRecord.STATE_INITIALIZED ||
                newPlayer.state != AudioTrack.STATE_INITIALIZED
            ) {
                newRecorder.release()
                newPlayer.release()
                Log.e(TAG, "Cannot initialize Android audio devices")
                return
            }

            activeCallId = callId
            recorder = newRecorder
            player = newPlayer
            newPlayer.play()
            newRecorder.startRecording()
            captureJob = scope.launch { capture(callId, newRecorder) }
            Log.i(TAG, "Audio bridge started for $callId")
        }
    }

    fun stop(callId: String, owner: String) {
        synchronized(lock) {
            if (activeCallId != callId) return
            activeOwners -= owner
            if (activeOwners.isNotEmpty()) return
            stopLocked()
        }
        Log.i(TAG, "Audio bridge stopped for $callId")
    }

    override fun onAudioFrame(
        callId: String,
        data: List<UByte>,
        sampleRate: UInt,
        channels: UInt
    ) {
        if (callId != activeCallId || sampleRate.toInt() != SAMPLE_RATE || channels.toInt() != 1) {
            return
        }
        val bytes = ByteArray(data.size) { data[it].toByte() }
        synchronized(lock) {
            player?.takeIf { it.playState == AudioTrack.PLAYSTATE_PLAYING }
                ?.write(bytes, 0, bytes.size, AudioTrack.WRITE_NON_BLOCKING)
        }
    }

    private suspend fun capture(callId: String, audioRecord: AudioRecord) {
        val buffer = ByteArray(FRAME_BYTES)
        while (scope.isActive && activeCallId == callId) {
            val read = audioRecord.read(buffer, 0, buffer.size, AudioRecord.READ_BLOCKING)
            if (read > 0) {
                try {
                    ZapLivreClientWrapper.sendAudioFrame(
                        callId,
                        buffer.copyOf(read).asUByteArray().asList(),
                        SAMPLE_RATE.toUInt(),
                        1u
                    )
                } catch (error: Exception) {
                    // Hangup removes the Rust peer before Compose disposes the
                    // screen. Stop quietly instead of flooding logs for frames
                    // that were already captured during that short race.
                    if (activeCallId != callId || error.message.orEmpty().contains("Call not found")) {
                        return
                    }
                    Log.w(TAG, "Unable to send audio frame for $callId: ${error.message}")
                }
            } else if (read < 0) {
                Log.e(TAG, "AudioRecord read failed: $read")
                break
            }
        }
    }

    private fun stopLocked() {
        activeCallId = null
        activeOwners.clear()
        captureJob?.cancel()
        captureJob = null
        recorder?.runCatching { stop() }
        recorder?.release()
        recorder = null
        player?.runCatching { stop() }
        player?.release()
        player = null
    }
}
