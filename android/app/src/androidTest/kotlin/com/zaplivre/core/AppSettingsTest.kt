package com.zaplivre.core

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Teste instrumentado (device/emulador) da persistência de preferências
 * implementada no F2. Valida que os toggles de Configurações sobrevivem a uma
 * nova leitura (simulando reabrir o app).
 */
@RunWith(AndroidJUnit4::class)
class AppSettingsTest {

    private val context get() = ApplicationProvider.getApplicationContext<android.content.Context>()

    private fun clearPrefs() {
        context.getSharedPreferences("zaplivre_settings", android.content.Context.MODE_PRIVATE)
            .edit().clear().commit()
    }

    @Test
    fun defaultsAreEnabled() {
        clearPrefs()
        assertTrue(AppSettings.notificationsEnabled(context))
        assertTrue(AppSettings.soundEnabled(context))
        assertTrue(AppSettings.vibrationEnabled(context))
        assertTrue(AppSettings.readReceiptsEnabled(context))
        assertTrue(AppSettings.lastSeenEnabled(context))
    }

    @Test
    fun togglesPersistAcrossReads() {
        clearPrefs()
        AppSettings.setNotificationsEnabled(context, false)
        AppSettings.setSoundEnabled(context, false)
        AppSettings.setVibrationEnabled(context, false)
        AppSettings.setReadReceiptsEnabled(context, false)
        AppSettings.setLastSeenEnabled(context, false)

        // Nova leitura (mesmo processo/SharedPreferences) reflete o estado salvo.
        assertFalse(AppSettings.notificationsEnabled(context))
        assertFalse(AppSettings.soundEnabled(context))
        assertFalse(AppSettings.vibrationEnabled(context))
        assertFalse(AppSettings.readReceiptsEnabled(context))
        assertFalse(AppSettings.lastSeenEnabled(context))
    }

    @Test
    fun independentPerToggle() {
        clearPrefs()
        AppSettings.setNotificationsEnabled(context, false)
        // Desligar notificações não afeta os demais toggles.
        assertTrue(AppSettings.soundEnabled(context))
        assertTrue(AppSettings.vibrationEnabled(context))
    }
}
