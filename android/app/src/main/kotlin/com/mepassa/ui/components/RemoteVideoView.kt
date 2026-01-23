package com.mepassa.ui.components

import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView

/**
 * RemoteVideoView - Composable wrapper for SurfaceView to render remote video
 *
 * This component uses SurfaceView to render video frames received from the remote peer.
 * The actual video decoding and rendering logic will be implemented in the FFI layer.
 */
@Composable
fun RemoteVideoView(
    callId: String,
    modifier: Modifier = Modifier,
    onSurfaceCreated: ((SurfaceHolder) -> Unit)? = null,
    onSurfaceDestroyed: (() -> Unit)? = null
) {
    val context = LocalContext.current
    val surfaceView = remember { SurfaceView(context) }

    DisposableEffect(callId) {
        val callback = object : SurfaceHolder.Callback {
            override fun surfaceCreated(holder: SurfaceHolder) {
                // Initialize video renderer
                onSurfaceCreated?.invoke(holder)
                
                // TODO: Register with CallManager to receive remote video frames
                // The FFI layer will provide video frames that need to be rendered
                // on this surface.
                //
                // Example:
                // MePassaClientWrapper.setRemoteVideoSurface(callId, holder.surface)
            }

            override fun surfaceChanged(
                holder: SurfaceHolder,
                format: Int,
                width: Int,
                height: Int
            ) {
                // Handle surface size changes
                // May need to adjust video scaling/aspect ratio
            }

            override fun surfaceDestroyed(holder: SurfaceHolder) {
                // Cleanup video renderer
                onSurfaceDestroyed?.invoke()
                
                // TODO: Unregister from CallManager
                // MePassaClientWrapper.removeRemoteVideoSurface(callId)
            }
        }

        surfaceView.holder.addCallback(callback)

        onDispose {
            surfaceView.holder.removeCallback(callback)
        }
    }

    AndroidView(
        factory = { surfaceView },
        modifier = modifier
    )
}
