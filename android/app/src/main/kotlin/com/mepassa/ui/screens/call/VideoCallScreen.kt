package com.mepassa.ui.screens.call

import android.util.Log
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
import com.mepassa.voip.CameraManager
import kotlinx.coroutines.launch
import uniffi.mepassa.FfiVideoCodec

/**
 * VideoCallScreen - UI for video call with local preview and remote video
 */
@OptIn(ExperimentalPermissionsApi::class)
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

    // Camera manager
    val cameraManager = remember { CameraManager(context) }

    // Preview views
    var localPreviewView by remember { mutableStateOf<PreviewView?>(null) }
    
    DisposableEffect(cameraPermissionState.status.isGranted, videoEnabled, localPreviewView) {
        // Start camera when screen appears and permission is granted
        if (cameraPermissionState.status.isGranted && videoEnabled && localPreviewView != null) {
            cameraManager.startCamera(
                lifecycleOwner = lifecycleOwner,
                previewView = localPreviewView!!,
                onFrameCallback = { data, width, height ->
                    // Send frame to WebRTC via FFI
                    scope.launch {
                        try {
                            MePassaClientWrapper.sendVideoFrame(
                                callId = callId,
                                frameData = data,
                                width = width.toUInt(),
                                height = height.toUInt()
                            )
                        } catch (e: Exception) {
                            // Frame drop is acceptable
                        }
                    }
                }
            )

            // Enable video track on WebRTC
            scope.launch {
                try {
                    MePassaClientWrapper.enableVideo(callId, FfiVideoCodec.H264)
                } catch (e: Exception) {
                    Log.e("VideoCallScreen", "Failed to enable video", e)
                }
            }
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
                            if (cameraPermissionState.status.isGranted && videoEnabled) {
                                cameraManager.startCamera(
                                    lifecycleOwner = lifecycleOwner,
                                    previewView = preview,
                                    onFrameCallback = { data, width, height ->
                                        scope.launch {
                                            try {
                                                MePassaClientWrapper.sendVideoFrame(
                                                    callId = callId,
                                                    frameData = data,
                                                    width = width.toUInt(),
                                                    height = height.toUInt()
                                                )
                                            } catch (e: Exception) {
                                                // Frame drop is acceptable
                                            }
                                        }
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
                        if (!cameraPermissionState.status.isGranted) {
                            // Request permission if not granted
                            cameraPermissionState.launchPermissionRequest()
                            return@IconButton
                        }

                        videoEnabled = !videoEnabled
                        if (videoEnabled && localPreviewView != null) {
                            cameraManager.startCamera(
                                lifecycleOwner = lifecycleOwner,
                                previewView = localPreviewView!!,
                                onFrameCallback = { data, width, height ->
                                    scope.launch {
                                        try {
                                            MePassaClientWrapper.sendVideoFrame(
                                                callId = callId,
                                                frameData = data,
                                                width = width.toUInt(),
                                                height = height.toUInt()
                                            )
                                        } catch (e: Exception) {
                                            // Frame drop is acceptable
                                        }
                                    }
                                }
                            )
                            // Enable video track on WebRTC
                            scope.launch {
                                try {
                                    MePassaClientWrapper.enableVideo(callId, FfiVideoCodec.H264)
                                } catch (e: Exception) {
                                    Log.e("VideoCallScreen", "Failed to enable video", e)
                                }
                            }
                        } else {
                            cameraManager.stopCamera()
                            // Disable video track on WebRTC
                            scope.launch {
                                try {
                                    MePassaClientWrapper.disableVideo(callId)
                                } catch (e: Exception) {
                                    Log.e("VideoCallScreen", "Failed to disable video", e)
                                }
                            }
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
                        if (!cameraPermissionState.status.isGranted) {
                            // Request permission if not granted
                            cameraPermissionState.launchPermissionRequest()
                            return@IconButton
                        }

                        if (localPreviewView != null) {
                            cameraManager.switchCamera(
                                lifecycleOwner = lifecycleOwner,
                                previewView = localPreviewView!!,
                                onFrameCallback = { data, width, height ->
                                    scope.launch {
                                        try {
                                            MePassaClientWrapper.sendVideoFrame(
                                                callId = callId,
                                                frameData = data,
                                                width = width.toUInt(),
                                                height = height.toUInt()
                                            )
                                        } catch (e: Exception) {
                                            // Frame drop is acceptable
                                        }
                                    }
                                }
                            )
                            // Notify FFI about camera switch
                            scope.launch {
                                try {
                                    MePassaClientWrapper.switchCamera(callId)
                                } catch (e: Exception) {
                                    Log.e("VideoCallScreen", "Failed to switch camera", e)
                                }
                            }
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
 * RemoteVideoView - Renders remote video using SurfaceView + MediaCodec
 */
@Composable
fun RemoteVideoView(
    callId: String,
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    // Video frame handler (implements FfiVideoFrameCallback)
    val videoHandler = remember(callId) {
        com.mepassa.voip.VideoFrameHandler(callId)
    }

    DisposableEffect(callId) {
        // Register callback when view appears
        scope.launch {
            try {
                MePassaClientWrapper.registerVideoFrameCallback(videoHandler)
                Log.d("RemoteVideoView", "✅ Video frame callback registered for call: $callId")
            } catch (e: Exception) {
                Log.e("RemoteVideoView", "❌ Failed to register video callback", e)
            }
        }

        onDispose {
            // Cleanup
            videoHandler.release()
        }
    }

    // SurfaceView for rendering decoded frames
    AndroidView(
        factory = { ctx ->
            android.view.SurfaceView(ctx).apply {
                holder.addCallback(object : android.view.SurfaceHolder.Callback {
                    override fun surfaceCreated(holder: android.view.SurfaceHolder) {
                        Log.d("RemoteVideoView", "📹 Surface created for call: $callId")
                        videoHandler.setSurface(holder.surface)
                    }

                    override fun surfaceChanged(
                        holder: android.view.SurfaceHolder,
                        format: Int,
                        width: Int,
                        height: Int
                    ) {
                        Log.d("RemoteVideoView", "🔄 Surface changed: ${width}x${height}")
                    }

                    override fun surfaceDestroyed(holder: android.view.SurfaceHolder) {
                        Log.d("RemoteVideoView", "🗑️ Surface destroyed")
                    }
                })
            }
        },
        modifier = modifier.background(Color.Black)
    )
}
