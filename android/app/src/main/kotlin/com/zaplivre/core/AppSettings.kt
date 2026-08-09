package com.zaplivre.core

import android.content.Context
import android.content.SharedPreferences

/**
 * Preferências do usuário (toggles de Configurações) persistidas em
 * SharedPreferences simples — não são segredos, então não usam
 * EncryptedSharedPreferences (diferente de [AndroidIdentityStore]/
 * [AndroidPushTokenStore]).
 *
 * F2: os toggles eram estado local (`remember { mutableStateOf }`) e
 * sumiam ao reabrir o app; agora persistem entre sessões.
 */
object AppSettings {

    private const val PREFS_NAME = "zaplivre_settings"

    private const val KEY_NOTIFICATIONS_ENABLED = "notifications_enabled"
    private const val KEY_SOUND_ENABLED = "sound_enabled"
    private const val KEY_VIBRATION_ENABLED = "vibration_enabled"
    private const val KEY_READ_RECEIPTS_ENABLED = "read_receipts_enabled"
    private const val KEY_LAST_SEEN_ENABLED = "last_seen_enabled"

    private fun prefs(context: Context): SharedPreferences =
        context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun notificationsEnabled(context: Context): Boolean =
        prefs(context).getBoolean(KEY_NOTIFICATIONS_ENABLED, true)

    fun setNotificationsEnabled(context: Context, enabled: Boolean) {
        prefs(context).edit().putBoolean(KEY_NOTIFICATIONS_ENABLED, enabled).apply()
    }

    fun soundEnabled(context: Context): Boolean =
        prefs(context).getBoolean(KEY_SOUND_ENABLED, true)

    fun setSoundEnabled(context: Context, enabled: Boolean) {
        prefs(context).edit().putBoolean(KEY_SOUND_ENABLED, enabled).apply()
    }

    fun vibrationEnabled(context: Context): Boolean =
        prefs(context).getBoolean(KEY_VIBRATION_ENABLED, true)

    fun setVibrationEnabled(context: Context, enabled: Boolean) {
        prefs(context).edit().putBoolean(KEY_VIBRATION_ENABLED, enabled).apply()
    }

    fun readReceiptsEnabled(context: Context): Boolean =
        prefs(context).getBoolean(KEY_READ_RECEIPTS_ENABLED, true)

    fun setReadReceiptsEnabled(context: Context, enabled: Boolean) {
        prefs(context).edit().putBoolean(KEY_READ_RECEIPTS_ENABLED, enabled).apply()
    }

    fun lastSeenEnabled(context: Context): Boolean =
        prefs(context).getBoolean(KEY_LAST_SEEN_ENABLED, true)

    fun setLastSeenEnabled(context: Context, enabled: Boolean) {
        prefs(context).edit().putBoolean(KEY_LAST_SEEN_ENABLED, enabled).apply()
    }
}
