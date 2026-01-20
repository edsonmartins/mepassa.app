package com.mepassa.service

import android.util.Log
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import com.mepassa.util.NotificationHelper

/**
 * Firebase Cloud Messaging Service
 *
 * Handles incoming FCM messages and token refresh events.
 */
class MePassaFirebaseMessagingService : FirebaseMessagingService() {

    override fun onNewToken(token: String) {
        super.onNewToken(token)
        Log.d(TAG, "New FCM token received: ${token.take(20)}...")

        // TODO: Send token to Push Server
        // This will be implemented in ETAPA 4
        sendTokenToServer(token)
    }

    override fun onMessageReceived(message: RemoteMessage) {
        super.onMessageReceived(message)
        Log.d(TAG, "FCM message received from: ${message.from}")

        // Extract notification data
        val title = message.data["title"] ?: message.notification?.title ?: "Nova mensagem"
        val body = message.data["body"] ?: message.notification?.body ?: "Você recebeu uma mensagem"
        val peerId = message.data["peer_id"]

        Log.d(TAG, "Notification - Title: $title, Body: $body, PeerId: $peerId")

        // Show notification
        NotificationHelper.showMessageNotification(
            context = this,
            title = title,
            body = body,
            peerId = peerId
        )

        // Wake up MePassaService to poll new messages
        try {
            MePassaService.start(this)
            Log.d(TAG, "MePassaService started to sync messages")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to start MePassaService", e)
        }
    }

    private fun sendTokenToServer(token: String) {
        // TODO: Implement in ETAPA 4 (Integration)
        // Will use Retrofit/OkHttp to POST token to Push Server
        // Endpoint: POST /api/v1/register
        // Body: { peer_id, platform: "fcm", device_id, token }
        Log.d(TAG, "TODO: Send token to server: ${token.take(20)}...")
    }

    companion object {
        private const val TAG = "FCM"
    }
}
