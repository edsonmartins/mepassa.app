package com.mepassa

import android.app.Application
import android.util.Log

/**
 * MePassa Application class
 *
 * Responsável por:
 * - Carregar biblioteca nativa (libmepassa_core.so)
 * - Inicializar configurações globais
 */
class MePassaApplication : Application() {

    companion object {
        private const val TAG = "MePassaApplication"

        // Load native library
        init {
            try {
                System.loadLibrary("mepassa_core")
                Log.i(TAG, "Native library loaded successfully")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load native library", e)
                throw RuntimeException("Failed to load mepassa_core native library", e)
            }
        }
    }

    override fun onCreate() {
        super.onCreate()
        Log.i(TAG, "MePassa Application created")

        // TODO: Inicializar outras dependências conforme necessário
        // - Notification channels
        // - Logging
        // - Crash reporting (futuro)
    }
}
