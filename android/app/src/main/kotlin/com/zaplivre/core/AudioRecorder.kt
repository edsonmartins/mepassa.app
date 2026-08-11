package com.zaplivre.core

import android.content.Context
import android.media.MediaRecorder
import android.os.Build
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.io.File
import java.io.IOException

/**
 * Audio recorder using MediaRecorder
 * Records audio in AAC format (M4A)
 */
class AudioRecorder(private val context: Context) {

    private var mediaRecorder: MediaRecorder? = null
    private var outputFile: File? = null

    private val _recordingState = MutableStateFlow<RecordingState>(RecordingState.Idle)
    val recordingState: StateFlow<RecordingState> = _recordingState.asStateFlow()

    private val _recordingDuration = MutableStateFlow(0L)
    val recordingDuration: StateFlow<Long> = _recordingDuration.asStateFlow()

    // Amplitude em tempo real (0..1) obtida do MediaRecorder.maxAmplitude
    private val _amplitude = MutableStateFlow(0f)
    val amplitude: StateFlow<Float> = _amplitude.asStateFlow()

    private var amplitudeJob: Job? = null
    private val scope = CoroutineScope(Dispatchers.IO)

    /**
     * Start recording audio
     */
    fun startRecording(): Result<File> {
        return try {
            // Create output file
            val file = createAudioFile()
            outputFile = file

            // Create MediaRecorder
            mediaRecorder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                MediaRecorder(context)
            } else {
                @Suppress("DEPRECATION")
                MediaRecorder()
            }

            mediaRecorder?.apply {
                setAudioSource(MediaRecorder.AudioSource.MIC)
                setOutputFormat(MediaRecorder.OutputFormat.MPEG_4)
                setAudioEncoder(MediaRecorder.AudioEncoder.AAC)
                setAudioEncodingBitRate(128000) // 128 kbps
                setAudioSamplingRate(44100) // 44.1 kHz
                setOutputFile(file.absolutePath)

                prepare()
                start()

                _recordingState.value = RecordingState.Recording(file)
                startAmplitudePolling()
            }

            Result.success(file)
        } catch (e: IOException) {
            _recordingState.value = RecordingState.Error(e.message ?: "Failed to start recording")
            Result.failure(e)
        } catch (e: Exception) {
            _recordingState.value = RecordingState.Error(e.message ?: "Unknown error")
            Result.failure(e)
        }
    }

    /**
     * Stop recording and return the audio file
     */
    fun stopRecording(): Result<File?> {
        return try {
            stopAmplitudePolling()
            mediaRecorder?.apply {
                stop()
                reset()
                release()
            }
            mediaRecorder = null

            val file = outputFile
            outputFile = null

            _recordingState.value = RecordingState.Idle

            Result.success(file)
        } catch (e: Exception) {
            stopAmplitudePolling()
            _recordingState.value = RecordingState.Error(e.message ?: "Failed to stop recording")
            Result.failure(e)
        }
    }

    /**
     * Poll the microphone amplitude while recording (for waveform display).
     * `getMaxAmplitude()` returns the max between polls, so each call resets it.
     */
    private fun startAmplitudePolling() {
        amplitudeJob?.cancel()
        amplitudeJob = scope.launch {
            while (isActive()) {
                delay(80) // ~12 updates/sec, sufficient for a smooth waveform
                val amp = mediaRecorder?.maxAmplitude ?: 0
                // Typical max amplitude is 0..32767; normalize with a soft cap for noise
                _amplitude.value = (amp / 32767f).coerceIn(0f, 1f)
            }
        }
    }

    private fun stopAmplitudePolling() {
        amplitudeJob?.cancel()
        amplitudeJob = null
        _amplitude.value = 0f
    }

    private suspend fun isActive(): Boolean = amplitudeJob?.isActive == true

    /**
     * Cancel recording and delete the file
     */
    fun cancelRecording() {
        stopAmplitudePolling()
        try {
            mediaRecorder?.apply {
                stop()
                reset()
                release()
            }
            mediaRecorder = null

            outputFile?.delete()
            outputFile = null

            _recordingState.value = RecordingState.Idle
        } catch (e: Exception) {
            _recordingState.value = RecordingState.Error(e.message ?: "Failed to cancel recording")
        }
    }

    /**
     * Create a temporary audio file
     */
    private fun createAudioFile(): File {
        val timestamp = System.currentTimeMillis()
        val fileName = "voice_message_$timestamp.m4a"
        return File(context.cacheDir, fileName)
    }

    /**
     * Get recording duration in milliseconds
     */
    fun getRecordingDuration(): Long {
        return _recordingDuration.value
    }

    /**
     * Release resources
     */
    fun release() {
        stopAmplitudePolling()
        mediaRecorder?.release()
        mediaRecorder = null
        outputFile = null
        _recordingState.value = RecordingState.Idle
    }
}

/**
 * Recording state
 */
sealed class RecordingState {
    object Idle : RecordingState()
    data class Recording(val file: File) : RecordingState()
    data class Error(val message: String) : RecordingState()
}
