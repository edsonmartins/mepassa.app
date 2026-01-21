package com.mepassa.ui.screens.call

import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Videocam
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import android.util.Log

/**
 * RemoteVideoView - Component for rendering remote video stream
 *
 * Displays the remote peer's video feed using SurfaceView.
 * For MVP, this is a placeholder that will be connected to WebRTC video track.
 *
 * @param callId ID of the active video call
 * @param modifier Modifier for layout customization
 */
@Composable
fun RemoteVideoView(
    callId: String,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    var isVideoReceiving by remember { mutableStateOf(false) }

    // For MVP: Show placeholder
    // In production: This would be connected to WebRTC video track
    Box(
        modifier = modifier
            .fillMaxSize()
            .background(Color.Black),
        contentAlignment = Alignment.Center
    ) {
        if (!isVideoReceiving) {
            // Placeholder when no video is being received
            androidx.compose.foundation.layout.Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Icon(
                    imageVector = Icons.Default.Videocam,
                    contentDescription = "Waiting for video",
                    tint = Color.White.copy(alpha = 0.5f),
                    modifier = androidx.compose.ui.Modifier.size(64.dp)
                )
                androidx.compose.foundation.layout.Spacer(modifier = androidx.compose.ui.Modifier.height(16.dp))
                Text(
                    text = "Waiting for video...",
                    color = Color.White.copy(alpha = 0.7f),
                    fontSize = 16.sp,
                    fontWeight = FontWeight.Medium
                )
            }
        } else {
            // SurfaceView for video rendering (future implementation)
            AndroidView(
                factory = { ctx ->
                    SurfaceView(ctx).apply {
                        holder.addCallback(object : SurfaceHolder.Callback {
                            override fun surfaceCreated(holder: SurfaceHolder) {
                                Log.d(TAG, "Remote video surface created for call: $callId")
                                // TODO: Initialize video renderer
                                // TODO: Register with CallManager to receive remote video frames
                                // TODO: Decode and render frames to surface
                            }

                            override fun surfaceChanged(
                                holder: SurfaceHolder,
                                format: Int,
                                width: Int,
                                height: Int
                            ) {
                                Log.d(TAG, "Remote video surface changed: ${width}x${height}")
                                // Handle surface dimension changes
                            }

                            override fun surfaceDestroyed(holder: SurfaceHolder) {
                                Log.d(TAG, "Remote video surface destroyed")
                                // Cleanup resources
                            }
                        })
                    }
                },
                modifier = Modifier.fillMaxSize()
            )
        }
    }
}

/**
 * RemoteVideoView with error state
 *
 * Alternative version that shows error messages when video fails
 */
@Composable
fun RemoteVideoViewWithError(
    callId: String,
    errorMessage: String? = null,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier
            .fillMaxSize()
            .background(Color.Black),
        contentAlignment = Alignment.Center
    ) {
        if (errorMessage != null) {
            // Show error message
            androidx.compose.foundation.layout.Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                androidx.compose.material3.Icon(
                    imageVector = androidx.compose.material.icons.Icons.Default.Error,
                    contentDescription = "Video error",
                    tint = Color.Red,
                    modifier = androidx.compose.ui.Modifier.size(48.dp)
                )
                androidx.compose.foundation.layout.Spacer(modifier = androidx.compose.ui.Modifier.height(16.dp))
                Text(
                    text = errorMessage,
                    color = Color.White,
                    fontSize = 14.sp
                )
            }
        } else {
            // Normal remote video view
            RemoteVideoView(callId = callId)
        }
    }
}

private const val TAG = "RemoteVideoView"
