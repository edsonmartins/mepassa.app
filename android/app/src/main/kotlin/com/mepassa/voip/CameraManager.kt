package com.mepassa.voip

import android.content.Context
import android.util.Log
import android.util.Size
import androidx.camera.core.*
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.core.content.ContextCompat
import androidx.lifecycle.LifecycleOwner
import java.nio.ByteBuffer
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

/**
 * Camera Manager for video calls using CameraX
 *
 * Manages camera capture, preview, and frame extraction for WebRTC video calls.
 * Supports front/back camera switching and frame callbacks for encoding.
 */
class CameraManager(private val context: Context) {
    private var cameraProvider: ProcessCameraProvider? = null
    private var camera: Camera? = null
    private var preview: Preview? = null
    private var imageAnalysis: ImageAnalysis? = null
    private var cameraExecutor: ExecutorService = Executors.newSingleThreadExecutor()

    @CameraSelector.LensFacing
    private var currentLensFacing = CameraSelector.LENS_FACING_FRONT

    private var frameCallback: ((ByteArray, Int, Int) -> Unit)? = null

    /**
     * Start camera capture with preview and frame extraction
     *
     * @param lifecycleOwner Lifecycle owner for camera lifecycle
     * @param previewView SurfaceView for camera preview
     * @param onFrame Callback for each captured frame (data, width, height)
     */
    fun startCamera(
        lifecycleOwner: LifecycleOwner,
        previewView: PreviewView,
        onFrame: (ByteArray, Int, Int) -> Unit
    ) {
        this.frameCallback = onFrame

        val cameraProviderFuture = ProcessCameraProvider.getInstance(context)

        cameraProviderFuture.addListener({
            try {
                val cameraProvider = cameraProviderFuture.get()
                this.cameraProvider = cameraProvider

                bindCamera(lifecycleOwner, previewView)
            } catch (e: Exception) {
                Log.e(TAG, "Camera initialization failed", e)
            }
        }, ContextCompat.getMainExecutor(context))
    }

    private fun bindCamera(lifecycleOwner: LifecycleOwner, previewView: PreviewView) {
        val cameraProvider = this.cameraProvider ?: return

        // Preview use case
        preview = Preview.Builder()
            .build()
            .also {
                it.setSurfaceProvider(previewView.surfaceProvider)
            }

        // ImageAnalysis use case (for frame extraction)
        imageAnalysis = ImageAnalysis.Builder()
            .setTargetResolution(Size(640, 480))  // VGA for MVP
            .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
            .setOutputImageFormat(ImageAnalysis.OUTPUT_IMAGE_FORMAT_YUV_420_888)
            .build()
            .also { analysis ->
                analysis.setAnalyzer(cameraExecutor) { imageProxy ->
                    processFrame(imageProxy)
                }
            }

        // Camera selector
        val cameraSelector = CameraSelector.Builder()
            .requireLensFacing(currentLensFacing)
            .build()

        // Unbind all use cases before rebinding
        cameraProvider.unbindAll()

        try {
            // Bind use cases to camera
            camera = cameraProvider.bindToLifecycle(
                lifecycleOwner,
                cameraSelector,
                preview,
                imageAnalysis
            )

            Log.i(TAG, "Camera started successfully (lens: ${getLensFacingString()})")
        } catch (e: Exception) {
            Log.e(TAG, "Camera binding failed", e)
        }
    }

    /**
     * Process image frame and extract YUV data
     */
    private fun processFrame(imageProxy: ImageProxy) {
        try {
            // Convert ImageProxy to byte array (YUV420)
            val buffer = imageProxy.planes[0].buffer
            val data = ByteArray(buffer.remaining())
            buffer.get(data)

            // Call frame callback with YUV data
            frameCallback?.invoke(data, imageProxy.width, imageProxy.height)

        } catch (e: Exception) {
            Log.e(TAG, "Frame processing error", e)
        } finally {
            imageProxy.close()
        }
    }

    /**
     * Switch between front and back camera
     */
    fun switchCamera(lifecycleOwner: LifecycleOwner, previewView: PreviewView) {
        currentLensFacing = if (currentLensFacing == CameraSelector.LENS_FACING_FRONT) {
            CameraSelector.LENS_FACING_BACK
        } else {
            CameraSelector.LENS_FACING_FRONT
        }

        Log.i(TAG, "Switching to ${getLensFacingString()} camera")

        // Rebind camera with new lens facing
        bindCamera(lifecycleOwner, previewView)
    }

    /**
     * Stop camera capture and release resources
     */
    fun stopCamera() {
        cameraProvider?.unbindAll()
        cameraExecutor.shutdown()
        frameCallback = null

        Log.i(TAG, "Camera stopped")
    }

    /**
     * Check if current camera is front-facing
     */
    fun isFrontCamera(): Boolean {
        return currentLensFacing == CameraSelector.LENS_FACING_FRONT
    }

    /**
     * Get current lens facing as string for logging
     */
    private fun getLensFacingString(): String {
        return if (currentLensFacing == CameraSelector.LENS_FACING_FRONT) "FRONT" else "BACK"
    }

    /**
     * Enable/disable torch (flashlight) - only works for back camera
     */
    fun enableTorch(enable: Boolean) {
        camera?.cameraControl?.enableTorch(enable)
    }

    /**
     * Get camera info for debugging
     */
    fun getCameraInfo(): String {
        return "Camera: ${getLensFacingString()}, Preview: ${preview != null}, Analysis: ${imageAnalysis != null}"
    }

    companion object {
        private const val TAG = "CameraManager"
    }
}
