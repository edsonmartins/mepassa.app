package com.mepassa.ui.screens.call

import android.Manifest
import androidx.camera.view.PreviewView
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
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.mepassa.core.MePassaClientWrapper
import com.mepassa.voip.CallAudioManager
import com.mepassa.voip.CameraManager
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlin.time.Duration.Companion.seconds

/**
 * VideoCallScreen - Tela durante uma videochamada ativa
 *
 * Exibe:
 * - Vídeo remoto em fullscreen
 * - Preview local da câmera (PiP no canto superior direito)
 * - Timer de duração da chamada
 * - Botões de controle: video toggle, mute, camera switch, hangup
 */
@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun VideoCallScreen(
    callId: String,
    remotePeerId: String,
    onCallEnded: () -> Unit
) {
    val scope = rememberCoroutineScope()
    val context = LocalContext.current
    val lifecycleOwner = LocalLifecycleOwner.current

    // Camera permission state
    val cameraPermissionState = rememberPermissionState(Manifest.permission.CAMERA)

    // Managers
    val audioManager = remember { CallAudioManager(context) }
    val cameraManager = remember { CameraManager(context) }

    // State
    var isMuted by remember { mutableStateOf(false) }
    var isVideoEnabled by remember { mutableStateOf(true) }
    var isFrontCamera by remember { mutableStateOf(true) }
    var callDuration by remember { mutableStateOf(0) }
    var isCallActive by remember { mutableStateOf(true) }
    var localPreviewView by remember { mutableStateOf<PreviewView?>(null) }

    // Initialize audio management
    DisposableEffect(Unit) {
        audioManager.startCall()
        onDispose {
            audioManager.stopCall()
            cameraManager.stopCamera()
        }
    }

    // Timer for call duration
    LaunchedEffect(Unit) {
        while (isCallActive) {
            delay(1.seconds)
            callDuration++
        }
    }

    // Request camera permission on start
    LaunchedEffect(Unit) {
        if (!cameraPermissionState.status.isGranted) {
            cameraPermissionState.launchPermissionRequest()
        }
    }

    // Start camera when permission granted and video enabled
    LaunchedEffect(cameraPermissionState.status.isGranted, isVideoEnabled, localPreviewView) {
        if (cameraPermissionState.status.isGranted && isVideoEnabled && localPreviewView != null) {
            cameraManager.startCamera(lifecycleOwner, localPreviewView!!) { frameData, width, height ->
                // Send frame to FFI for encoding and transmission
                scope.launch {
                    try {
                        MePassaClientWrapper.client?.sendVideoFrame(
                            callId = callId,
                            frameData = frameData,
                            width = width.toUInt(),
                            height = height.toUInt()
                        )
                    } catch (e: Exception) {
                        // Log error but don't crash
                    }
                }
            }
        }
    }

    // Main layout
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(Color.Black)
    ) {
        // Remote video (fullscreen)
        // TODO: Implement RemoteVideoView component
        Box(
            modifier = Modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = "Remote Video\n(Placeholder)",
                color = Color.White,
                fontSize = 18.sp
            )
        }

        // Local camera preview (PiP - top right corner)
        if (isVideoEnabled && cameraPermissionState.status.isGranted) {
            AndroidView(
                factory = { ctx ->
                    PreviewView(ctx).also { previewView ->
                        localPreviewView = previewView
                    }
                },
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .padding(16.dp)
                    .size(120.dp, 160.dp)
                    .clip(RoundedCornerShape(12.dp))
            )
        }

        // Call info overlay (top)
        Column(
            modifier = Modifier
                .align(Alignment.TopStart)
                .padding(16.dp)
                .background(
                    Color.Black.copy(alpha = 0.5f),
                    shape = RoundedCornerShape(8.dp)
                )
                .padding(horizontal = 16.dp, vertical = 8.dp),
            horizontalAlignment = Alignment.Start
        ) {
            // Peer name (truncated)
            Text(
                text = remotePeerId.take(16) + if (remotePeerId.length > 16) "..." else "",
                color = Color.White,
                fontSize = 16.sp,
                fontWeight = FontWeight.Medium
            )

            // Call duration
            Text(
                text = formatDuration(callDuration),
                color = Color.White.copy(alpha = 0.8f),
                fontSize = 14.sp
            )
        }

        // Controls overlay (bottom)
        Column(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .fillMaxWidth()
                .background(Color.Black.copy(alpha = 0.6f))
                .padding(vertical = 24.dp, horizontal = 16.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Control buttons row
            Row(
                horizontalArrangement = Arrangement.SpaceEvenly,
                modifier = Modifier.fillMaxWidth()
            ) {
                // Video toggle button
                IconButton(
                    onClick = {
                        isVideoEnabled = !isVideoEnabled
                        scope.launch {
                            try {
                                if (isVideoEnabled) {
                                    MePassaClientWrapper.client?.enableVideo(callId, uniffi.mepassa_core.FfiVideoCodec.H264)
                                } else {
                                    MePassaClientWrapper.client?.disableVideo(callId)
                                    cameraManager.stopCamera()
                                }
                            } catch (e: Exception) {
                                // Handle error
                            }
                        }
                    },
                    modifier = Modifier
                        .size(56.dp)
                        .background(
                            color = if (isVideoEnabled) MaterialTheme.colorScheme.primary else Color.Red,
                            shape = CircleShape
                        )
                ) {
                    Icon(
                        imageVector = if (isVideoEnabled) Icons.Default.Videocam else Icons.Default.VideocamOff,
                        contentDescription = "Toggle video",
                        tint = Color.White
                    )
                }

                // Mute toggle button
                IconButton(
                    onClick = {
                        isMuted = !isMuted
                        scope.launch {
                            MePassaClientWrapper.client?.toggleMute(callId)
                        }
                    },
                    modifier = Modifier
                        .size(56.dp)
                        .background(
                            color = if (isMuted) Color.Red else MaterialTheme.colorScheme.primary,
                            shape = CircleShape
                        )
                ) {
                    Icon(
                        imageVector = if (isMuted) Icons.Default.MicOff else Icons.Default.Mic,
                        contentDescription = "Toggle mute",
                        tint = Color.White
                    )
                }

                // Switch camera button
                IconButton(
                    onClick = {
                        isFrontCamera = !isFrontCamera
                        if (localPreviewView != null) {
                            cameraManager.switchCamera(lifecycleOwner, localPreviewView!!)
                        }
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

                // Hangup button (larger, more prominent)
                IconButton(
                    onClick = {
                        isCallActive = false
                        scope.launch {
                            MePassaClientWrapper.client?.hangupCall(callId)
                            cameraManager.stopCamera()
                            onCallEnded()
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

        // Camera permission denied message
        if (!cameraPermissionState.status.isGranted && isVideoEnabled) {
            Box(
                modifier = Modifier
                    .align(Alignment.Center)
                    .background(
                        Color.Black.copy(alpha = 0.7f),
                        shape = RoundedCornerShape(12.dp)
                    )
                    .padding(24.dp)
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Icon(
                        imageVector = Icons.Default.Videocam,
                        contentDescription = null,
                        tint = Color.White,
                        modifier = Modifier.size(48.dp)
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = "Camera Permission Required",
                        color = Color.White,
                        fontSize = 18.sp,
                        fontWeight = FontWeight.Medium
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Button(
                        onClick = { cameraPermissionState.launchPermissionRequest() }
                    ) {
                        Text("Grant Permission")
                    }
                }
            }
        }
    }
}

/**
 * Format call duration in MM:SS format
 */
private fun formatDuration(seconds: Int): String {
    val minutes = seconds / 60
    val secs = seconds % 60
    return String.format("%02d:%02d", minutes, secs)
}
