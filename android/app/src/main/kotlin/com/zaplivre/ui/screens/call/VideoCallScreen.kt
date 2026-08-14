package com.zaplivre.ui.screens.call

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.zaplivre.core.ZapLivreClientWrapper
import com.zaplivre.voip.CallAudioManager
import com.zaplivre.voip.NativeWebRtcSession
import kotlinx.coroutines.launch
import org.webrtc.SurfaceViewRenderer

/**
 * VideoCallScreen - UI for video call with local preview and remote video
 */
@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun VideoCallScreen(
    callId: String,
    remotePeerId: String,
    peerName: String,
    onHangup: () -> Unit,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val audioManager = remember { CallAudioManager(context) }

    DisposableEffect(callId) {
        audioManager.startCall()
        onDispose {
            audioManager.stopCall()
        }
    }

    // Camera permission
    val cameraPermissionState = rememberPermissionState(android.Manifest.permission.CAMERA)

    // Request permission on first composition
    LaunchedEffect(Unit) {
        if (!cameraPermissionState.status.isGranted) {
            cameraPermissionState.launchPermissionRequest()
        }
    }

    // State
    var videoEnabled by remember { mutableStateOf(true) }
    var isMuted by remember { mutableStateOf(false) }
    var callDuration by remember { mutableStateOf(0) }

    var localRenderer by remember { mutableStateOf<SurfaceViewRenderer?>(null) }
    var remoteRenderer by remember { mutableStateOf<SurfaceViewRenderer?>(null) }
    var nativeSession by remember { mutableStateOf<NativeWebRtcSession?>(null) }

    DisposableEffect(
        callId,
        remotePeerId,
        cameraPermissionState.status.isGranted,
        localRenderer,
        remoteRenderer
    ) {
        if (cameraPermissionState.status.isGranted && localRenderer != null && remoteRenderer != null) {
            val localPeerId = ZapLivreClientWrapper.localPeerId.value.orEmpty()
            val session = NativeWebRtcSession(context, callId)
            nativeSession = session
            session.start(
                localRenderer = localRenderer!!,
                remoteRenderer = remoteRenderer!!,
                createOffer = localPeerId < remotePeerId
            )
        }
        onDispose {
            nativeSession?.stop()
            nativeSession = null
        }
    }

    LaunchedEffect(videoEnabled, nativeSession) {
        nativeSession?.setVideoEnabled(videoEnabled)
    }

    Box(
        modifier = modifier.fillMaxSize()
    ) {
        // Remote video (full screen)
        AndroidView(
            factory = { ctx -> SurfaceViewRenderer(ctx).also { remoteRenderer = it } },
            modifier = Modifier.fillMaxSize()
        )

        // Local video preview (PiP - top right corner)
        if (videoEnabled) {
            Box(
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .padding(16.dp)
                    .size(120.dp, 160.dp)
                    .clip(RoundedCornerShape(12.dp))
                    .background(Color.Black)
            ) {
                AndroidView(
                    factory = { ctx ->
                        SurfaceViewRenderer(ctx).also { localRenderer = it }
                    },
                    modifier = Modifier.fillMaxSize()
                )
            }
        }

        // Controls overlay (bottom)
        Column(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .fillMaxWidth()
                .background(Color.Black.copy(alpha = 0.5f))
                .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Call info
            Text(
                text = peerName,
                color = Color.White,
                fontSize = 20.sp,
                fontWeight = FontWeight.Medium
            )

            // Call duration
            Text(
                text = formatDuration(callDuration),
                color = Color.White.copy(alpha = 0.8f),
                fontSize = 16.sp
            )

            // Control buttons row
            Row(
                horizontalArrangement = Arrangement.spacedBy(20.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                // Video toggle
                IconButton(
                    onClick = {
                        if (!cameraPermissionState.status.isGranted) {
                            // Request permission if not granted
                            cameraPermissionState.launchPermissionRequest()
                            return@IconButton
                        }

                        videoEnabled = !videoEnabled
                    },
                    modifier = Modifier
                        .size(56.dp)
                        .background(
                            color = if (videoEnabled) MaterialTheme.colorScheme.primary
                            else Color.Red,
                            shape = CircleShape
                        )
                ) {
                    Icon(
                        imageVector = if (videoEnabled) Icons.Default.Videocam
                        else Icons.Default.VideocamOff,
                        contentDescription = "Toggle video",
                        tint = Color.White
                    )
                }

                // Mute toggle
                IconButton(
                    onClick = {
                        scope.launch {
                            try {
                                ZapLivreClientWrapper.toggleMute(callId)
                                isMuted = audioManager.toggleMute()
                                nativeSession?.setAudioEnabled(!isMuted)
                            } catch (e: Exception) {
                                Log.e("VideoCallScreen", "Failed to toggle mute", e)
                            }
                        }
                    },
                    modifier = Modifier
                        .size(56.dp)
                        .background(
                            color = if (isMuted) Color.Red
                            else MaterialTheme.colorScheme.primary,
                            shape = CircleShape
                        )
                ) {
                    Icon(
                        imageVector = if (isMuted) Icons.Default.MicOff
                        else Icons.Default.Mic,
                        contentDescription = "Toggle mute",
                        tint = Color.White
                    )
                }

                // Switch camera
                IconButton(
                    onClick = {
                        if (!cameraPermissionState.status.isGranted) {
                            // Request permission if not granted
                            cameraPermissionState.launchPermissionRequest()
                            return@IconButton
                        }

                        nativeSession?.switchCamera()
                    },
                    modifier = Modifier
                        .size(56.dp)
                        .background(
                            color = MaterialTheme.colorScheme.primary,
                            shape = CircleShape
                        )
                ) {
                    Icon(
                        imageVector = Icons.Default.Cameraswitch,
                        contentDescription = "Switch camera",
                        tint = Color.White
                    )
                }

                // Hangup
                IconButton(
                    onClick = {
                        scope.launch {
                            try {
                                ZapLivreClientWrapper.hangupCall(callId)
                            } catch (e: Exception) {
                                Log.e("VideoCallScreen", "Failed to hangup", e)
                            } finally {
                                onHangup()
                            }
                        }
                    },
                    modifier = Modifier
                        .size(72.dp)
                        .background(color = Color(0xFFE53935), shape = CircleShape)
                ) {
                    Icon(
                        imageVector = Icons.Default.CallEnd,
                        contentDescription = "End call",
                        tint = Color.White,
                        modifier = Modifier.size(32.dp)
                    )
                }
            }
        }
    }

    // Call duration timer
    LaunchedEffect(Unit) {
        while (true) {
            kotlinx.coroutines.delay(1000)
            callDuration++
        }
    }
}

/**
 * Format call duration (seconds → MM:SS or HH:MM:SS)
 */
private fun formatDuration(seconds: Int): String {
    val hours = seconds / 3600
    val minutes = (seconds % 3600) / 60
    val secs = seconds % 60

    return if (hours > 0) {
        String.format("%d:%02d:%02d", hours, minutes, secs)
    } else {
        String.format("%02d:%02d", minutes, secs)
    }
}
