package com.zaplivre.voip

import android.content.Context
import android.media.AudioDeviceInfo
import android.media.AudioManager
import android.os.Build
import android.util.Log
import androidx.annotation.RequiresApi

/**
 * CallAudioManager - Gerencia roteamento de áudio durante chamadas VoIP
 *
 * Responsabilidades:
 * - Configurar AudioManager para modo VoIP
 * - Gerenciar dispositivos de áudio (Speaker, Earpiece, Bluetooth)
 * - Request/Abandon audio focus
 * - Detectar e rotear para Bluetooth headsets
 */
class CallAudioManager(private val context: Context) {

    companion object {
        private const val TAG = "CallAudioManager"
    }

    private val audioManager: AudioManager =
        context.getSystemService(Context.AUDIO_SERVICE) as AudioManager

    private var savedAudioMode: Int = AudioManager.MODE_NORMAL
    private var savedSpeakerphoneOn: Boolean = false
    private var savedMicrophoneMute: Boolean = false
    private var audioFocusRequested: Boolean = false

    /**
     * Inicia gerenciamento de áudio para chamada
     */
    fun startCall() {
        Log.i(TAG, "Starting call audio management")

        // Salvar configurações atuais
        savedAudioMode = audioManager.mode
        savedSpeakerphoneOn = isSpeakerphoneOn()
        savedMicrophoneMute = audioManager.isMicrophoneMute

        // Configurar modo de comunicação (otimizado para voz)
        audioManager.mode = AudioManager.MODE_IN_COMMUNICATION

        // Request audio focus
        requestAudioFocus()

        // Verificar se há Bluetooth conectado
        if (hasBluetoothDevice()) {
            Log.i(TAG, "Bluetooth device detected, routing to Bluetooth")
            routeToBluetoothIfAvailable()
        } else {
            // Por padrão, usar earpiece (não speaker)
            setSpeakerphone(false)
        }

        // Unmute por padrão
        audioManager.isMicrophoneMute = false

        Log.i(TAG, "Call audio started - Mode: ${audioManager.mode}, Speaker: ${isSpeakerphoneOn()}, Device: ${currentDeviceName()}")
    }

    /**
     * Finaliza gerenciamento de áudio
     */
    fun stopCall() {
        Log.i(TAG, "Stopping call audio management")

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            audioManager.clearCommunicationDevice()
        } else {
            @Suppress("DEPRECATION")
            if (audioManager.isBluetoothScoOn) {
                @Suppress("DEPRECATION")
                audioManager.stopBluetoothSco()
                @Suppress("DEPRECATION")
                audioManager.isBluetoothScoOn = false
            }
        }

        // Restaurar configurações originais
        audioManager.mode = savedAudioMode
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) {
            setSpeakerphoneLegacy(savedSpeakerphoneOn)
        }
        audioManager.isMicrophoneMute = savedMicrophoneMute

        // Abandon audio focus
        abandonAudioFocus()

        Log.i(TAG, "Call audio stopped")
    }

    /**
     * Toggle speakerphone (alto-falante)
     *
     * @return true se speakerphone está ativado após toggle
     */
    fun toggleSpeakerphone(): Boolean {
        setSpeakerphone(!isSpeakerphoneOn())
        return isSpeakerphoneOn()
    }

    /**
     * Force speakerphone state
     */
    fun setSpeakerphone(enabled: Boolean) {
        val routed = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            val targetType = if (enabled) {
                AudioDeviceInfo.TYPE_BUILTIN_SPEAKER
            } else {
                AudioDeviceInfo.TYPE_BUILTIN_EARPIECE
            }
            val target = audioManager.availableCommunicationDevices.firstOrNull {
                it.type == targetType
            }
            if (target != null) {
                audioManager.setCommunicationDevice(target)
            } else if (!enabled) {
                audioManager.clearCommunicationDevice()
                true
            } else {
                false
            }
        } else {
            setSpeakerphoneLegacy(enabled)
            true
        }
        Log.i(TAG, "Speakerphone requested: $enabled, routed: $routed, device: ${currentDeviceName()}")
    }

    /**
     * Toggle mute do microfone
     *
     * @return true se microfone está mutado após toggle
     */
    fun toggleMute(): Boolean {
        val newState = !audioManager.isMicrophoneMute
        audioManager.isMicrophoneMute = newState
        Log.i(TAG, "Microphone mute toggled: $newState")

        return newState
    }

    /**
     * Force mute state
     */
    fun setMuted(muted: Boolean) {
        audioManager.isMicrophoneMute = muted
        Log.i(TAG, "Microphone mute set to: $muted")
    }

    /**
     * Verifica se speakerphone está ativado
     */
    fun isSpeakerphoneOn(): Boolean = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        audioManager.communicationDevice?.type == AudioDeviceInfo.TYPE_BUILTIN_SPEAKER
    } else {
        @Suppress("DEPRECATION")
        audioManager.isSpeakerphoneOn
    }

    /**
     * Verifica se microfone está mutado
     */
    fun isMicrophoneMute(): Boolean = audioManager.isMicrophoneMute

    /**
     * Verifica se há dispositivo Bluetooth conectado
     */
    fun hasBluetoothDevice(): Boolean {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            hasBluetoothDeviceApi23()
        } else {
            // Fallback para APIs antigas
            audioManager.isBluetoothA2dpOn || audioManager.isBluetoothScoOn
        }
    }

    @RequiresApi(Build.VERSION_CODES.M)
    private fun hasBluetoothDeviceApi23(): Boolean {
        val devices = audioManager.getDevices(AudioManager.GET_DEVICES_OUTPUTS)
        return devices.any { device ->
            device.type == AudioDeviceInfo.TYPE_BLUETOOTH_SCO ||
            device.type == AudioDeviceInfo.TYPE_BLUETOOTH_A2DP
        }
    }

    /**
     * Rotear áudio para Bluetooth (se disponível)
     */
    fun routeToBluetoothIfAvailable(): Boolean {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            val bluetooth = audioManager.availableCommunicationDevices.firstOrNull {
                it.type == AudioDeviceInfo.TYPE_BLUETOOTH_SCO ||
                    it.type == AudioDeviceInfo.TYPE_BLE_HEADSET ||
                    it.type == AudioDeviceInfo.TYPE_BLE_SPEAKER
            }
            if (bluetooth != null && audioManager.setCommunicationDevice(bluetooth)) {
                Log.i(TAG, "Audio routed to Bluetooth: ${bluetooth.productName}")
                return true
            }
        } else if (hasBluetoothDevice()) {
            @Suppress("DEPRECATION")
            audioManager.isSpeakerphoneOn = false
            @Suppress("DEPRECATION")
            audioManager.isBluetoothScoOn = true
            @Suppress("DEPRECATION")
            audioManager.startBluetoothSco()
            Log.i(TAG, "Audio routed to Bluetooth")
            return true
        }
        Log.w(TAG, "No Bluetooth communication device available")
        return false
    }

    @Suppress("DEPRECATION")
    private fun setSpeakerphoneLegacy(enabled: Boolean) {
        if (enabled && audioManager.isBluetoothScoOn) {
            audioManager.stopBluetoothSco()
            audioManager.isBluetoothScoOn = false
        }
        audioManager.isSpeakerphoneOn = enabled
    }

    private fun currentDeviceName(): String = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        audioManager.communicationDevice?.let { "${it.productName} (type=${it.type})" } ?: "system default"
    } else if (isSpeakerphoneOn()) {
        "speaker"
    } else {
        "earpiece/bluetooth"
    }

    /**
     * Request audio focus (necessário para VoIP)
     */
    private fun requestAudioFocus() {
        if (audioFocusRequested) return

        val result = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            requestAudioFocusApi26()
        } else {
            @Suppress("DEPRECATION")
            audioManager.requestAudioFocus(
                null,
                AudioManager.STREAM_VOICE_CALL,
                AudioManager.AUDIOFOCUS_GAIN_TRANSIENT
            )
        }

        audioFocusRequested = (result == AudioManager.AUDIOFOCUS_REQUEST_GRANTED)
        Log.i(TAG, "Audio focus requested: $audioFocusRequested")
    }

    @RequiresApi(Build.VERSION_CODES.O)
    private fun requestAudioFocusApi26(): Int {
        val focusRequest = android.media.AudioFocusRequest.Builder(
            AudioManager.AUDIOFOCUS_GAIN_TRANSIENT
        )
            .setAudioAttributes(
                android.media.AudioAttributes.Builder()
                    .setUsage(android.media.AudioAttributes.USAGE_VOICE_COMMUNICATION)
                    .setContentType(android.media.AudioAttributes.CONTENT_TYPE_SPEECH)
                    .build()
            )
            .build()

        return audioManager.requestAudioFocus(focusRequest)
    }

    /**
     * Abandon audio focus
     */
    private fun abandonAudioFocus() {
        if (!audioFocusRequested) return

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            // API 26+ não precisa fazer nada (focus request é local)
        } else {
            @Suppress("DEPRECATION")
            audioManager.abandonAudioFocus(null)
        }

        audioFocusRequested = false
        Log.i(TAG, "Audio focus abandoned")
    }

    /**
     * Retorna lista de dispositivos de áudio disponíveis
     */
    fun getAvailableDevices(): List<AudioDevice> {
        val devices = mutableListOf<AudioDevice>()

        // Earpiece sempre disponível
        devices.add(AudioDevice.EARPIECE)

        // Speaker sempre disponível
        devices.add(AudioDevice.SPEAKER)

        // Bluetooth se disponível
        if (hasBluetoothDevice()) {
            devices.add(AudioDevice.BLUETOOTH)
        }

        return devices
    }

    enum class AudioDevice {
        EARPIECE,
        SPEAKER,
        BLUETOOTH
    }
}
