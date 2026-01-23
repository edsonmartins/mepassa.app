package com.mepassa.ui.screens.call

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
import com.mepassa.voip.CameraManager
import kotlinx.coroutines.launch

/**
 * VideoCallScreen - UI for video call with local preview and remote video
 */
@Composable
fun VideoCallScreen(
    callId: String,
    peerName: String,
    onHangup: () -> Unit,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val lifecycleOwner = LocalLifecycleOwner.current
    val scope = rememberCoroutineScope()
    
    // State
    var videoEnabled by remember { mutableStateOf(true) }
    var isMuted by remember { mutableStateOf(false) }
    var callDuration by remember { mutableStateOf(0) }
    
    // Camera manager
    val cameraManager = remember { CameraManager(context) }
    
    // Preview views
    var localPreviewView by remember { mutableStateOf<PreviewView?>(null) }
    
    DisposableEffect(Unit) {
        // Start camera when screen appears
        if (videoEnabled && localPreviewView != null) {
            cameraManager.startCamera(
                lifecycleOwner = lifecycleOwner,
                previewView = localPreviewView!!,
                onFrameCallback = { data, width, height ->
                    // TODO: Send frame to WebRTC via FFI
                    // MePassaClientWrapper.sendVideoFrame(callId, data, width, height)
                }
            )
        }
        
        onDispose {
            cameraManager.stopCamera()
            cameraManager.release()
        }
    }

    Box(
        modifier = modifier.fillMaxSize()
    ) {
        // Remote video (full screen)
        RemoteVideoView(
            callId = callId,
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
                        PreviewView(ctx).also { preview ->
                            localPreviewView = preview
                            if (videoEnabled) {
                                cameraManager.startCamera(
                                    lifecycleOwner = lifecycleOwner,
                                    previewView = preview,
                                    onFrameCallback = { data, width, height ->
                                        // TODO: Send frame to WebRTC
                                    }
                                )
                            }
                        }
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
                        videoEnabled = !videoEnabled
                        if (videoEnabled && localPreviewView != null) {
                            cameraManager.startCamera(
                                lifecycleOwner = lifecycleOwner,
                                previewView = localPreviewView!!,
                                onFrameCallback = { data, width, height ->
                                    // TODO: Send frame to WebRTC
                                }
                            )
                            // TODO: Call FFI enable_video
                            // MePassaClientWrapper.enableVideo(callId, FfiVideoCodec.H264)
                        } else {
                            cameraManager.stopCamera()
                            // TODO: Call FFI disable_video
                            // MePassaClientWrapper.disableVideo(callId)
                        }
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
                        isMuted = !isMuted
                        // TODO: Call FFI toggle_mute
                        // MePassaClientWrapper.toggleMute(callId)
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
                        if (localPreviewView != null) {
                            cameraManager.switchCamera(
                                lifecycleOwner = lifecycleOwner,
                                previewView = localPreviewView!!,
                                onFrameCallback = { data, width, height ->
                                    // TODO: Send frame to WebRTC
                                }
                            )
                            // TODO: Call FFI switch_camera
                            // MePassaClientWrapper.switchCamera(callId)
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

                // Hangup
                IconButton(
                    onClick = onHangup,
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

/**
 * RemoteVideoView - Placeholder for remote video rendering
 * TODO: Implement actual video rendering using SurfaceView or TextureView
 */
@Composable
fun RemoteVideoView(
    callId: String,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier.background(Color.Black),
        contentAlignment = Alignment.Center
    ) {
        // Placeholder - will be replaced with actual video surface
        Text(
            text = "Remote Video\n(To be implemented)",
            color = Color.White.copy(alpha = 0.5f),
            fontSize = 14.sp
        )
    }
}
