package com.zaplivre.voip

import android.content.Context
import android.util.Log
import android.util.Size
import androidx.camera.core.Camera
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.core.content.ContextCompat
import androidx.lifecycle.LifecycleOwner
import java.nio.ByteBuffer
import java.util.concurrent.Executors

/**
 * CameraManager - Manages camera capture for video calls using CameraX
 */
class CameraManager(private val context: Context) {
    
    companion object {
        private const val TAG = "CameraManager"
    }

    private val cameraExecutor = Executors.newSingleThreadExecutor()

    private var cameraProvider: ProcessCameraProvider? = null
    private var camera: Camera? = null
    private var preview: Preview? = null
    private var imageAnalysis: ImageAnalysis? = null

    private var currentLensFacing = CameraSelector.LENS_FACING_FRONT

    /**
     * Start camera capture
     * @param lifecycleOwner Activity or Fragment lifecycle owner
     * @param previewView Surface for camera preview
     * @param onFrameCallback Callback for each captured frame
     */
    fun startCamera(
        lifecycleOwner: LifecycleOwner,
        previewView: PreviewView,
        onFrameCallback: (ByteArray, Int, Int) -> Unit
    ) {
        val cameraProviderFuture = ProcessCameraProvider.getInstance(context)

        cameraProviderFuture.addListener({
            try {
                val provider = cameraProviderFuture.get()
                this.cameraProvider = provider

                // Preview use case
                preview = Preview.Builder()
                    .setTargetResolution(Size(640, 480))
                    .build()
                    .also {
                        it.setSurfaceProvider(previewView.surfaceProvider)
                    }

                // ImageAnalysis use case (for sending frames to WebRTC)
                imageAnalysis = ImageAnalysis.Builder()
                    .setTargetResolution(Size(640, 480))
                    .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                    .build()
                    .also { analysis ->
                        analysis.setAnalyzer(cameraExecutor) { imageProxy ->
                            try {
                                val data = yuv420ToNv21(imageProxy)
                                val rotation = imageProxy.imageInfo.rotationDegrees
                                val rotated = rotateNv21(
                                    data,
                                    imageProxy.width,
                                    imageProxy.height,
                                    rotation
                                )
                                val outputWidth = if (rotation == 90 || rotation == 270) {
                                    imageProxy.height
                                } else {
                                    imageProxy.width
                                }
                                val outputHeight = if (rotation == 90 || rotation == 270) {
                                    imageProxy.width
                                } else {
                                    imageProxy.height
                                }
                                onFrameCallback(rotated, outputWidth, outputHeight)
                            } catch (e: Exception) {
                                Log.e(TAG, "Error processing frame", e)
                            } finally {
                                imageProxy.close()
                            }
                        }
                    }

                // Camera selector
                val cameraSelector = CameraSelector.Builder()
                    .requireLensFacing(currentLensFacing)
                    .build()

                // Bind to lifecycle
                provider.unbindAll()
                camera = provider.bindToLifecycle(
                    lifecycleOwner,
                    cameraSelector,
                    preview,
                    imageAnalysis
                )

                Log.i(TAG, "✅ Camera started successfully")

            } catch (e: Exception) {
                Log.e(TAG, "❌ Camera binding failed", e)
            }
        }, ContextCompat.getMainExecutor(context))
    }

    /**
     * Switch camera (front ↔ back)
     * @param lifecycleOwner Activity or Fragment lifecycle owner
     * @param previewView Surface for camera preview
     * @param onFrameCallback Callback for each captured frame
     */
    fun switchCamera(
        lifecycleOwner: LifecycleOwner,
        previewView: PreviewView,
        onFrameCallback: (ByteArray, Int, Int) -> Unit
    ) {
        // Toggle lens facing
        currentLensFacing = if (currentLensFacing == CameraSelector.LENS_FACING_FRONT) {
            CameraSelector.LENS_FACING_BACK
        } else {
            CameraSelector.LENS_FACING_FRONT
        }

        // Restart camera with new lens facing
        stopCamera()
        startCamera(lifecycleOwner, previewView, onFrameCallback)

        Log.i(TAG, "📷 Camera switched to ${if (currentLensFacing == CameraSelector.LENS_FACING_FRONT) "FRONT" else "BACK"}")
    }

    /**
     * Stop camera capture
     */
    fun stopCamera() {
        cameraProvider?.unbindAll()
        camera = null
        preview = null
        imageAnalysis = null

        Log.i(TAG, "🛑 Camera stopped")
    }

    /**
     * Check if camera is running
     */
    fun isRunning(): Boolean {
        return camera != null
    }

    /**
     * Get current lens facing
     */
    fun getCurrentLensFacing(): Int {
        return currentLensFacing
    }

    /**
     * Cleanup resources
     */
    fun release() {
        stopCamera()
        cameraExecutor.shutdown()
    }

    private fun yuv420ToNv21(image: androidx.camera.core.ImageProxy): ByteArray {
        val width = image.width
        val height = image.height
        val ySize = width * height
        val uvSize = width * height / 2
        val nv21 = ByteArray(ySize + uvSize)

        val yPlane = image.planes[0]
        val uPlane = image.planes[1]
        val vPlane = image.planes[2]

        val yBuffer = yPlane.buffer
        val uBuffer = uPlane.buffer
        val vBuffer = vPlane.buffer

        val yRowStride = yPlane.rowStride
        val yPixelStride = yPlane.pixelStride
        val uRowStride = uPlane.rowStride
        val uPixelStride = uPlane.pixelStride
        val vRowStride = vPlane.rowStride
        val vPixelStride = vPlane.pixelStride

        var pos = 0
        for (row in 0 until height) {
            var col = 0
            while (col < width) {
                nv21[pos++] = yBuffer.get(row * yRowStride + col * yPixelStride)
                col++
            }
        }

        var uvPos = ySize
        val chromaHeight = height / 2
        val chromaWidth = width / 2
        for (row in 0 until chromaHeight) {
            var col = 0
            while (col < chromaWidth) {
                val vIndex = row * vRowStride + col * vPixelStride
                val uIndex = row * uRowStride + col * uPixelStride
                nv21[uvPos++] = vBuffer.get(vIndex)
                nv21[uvPos++] = uBuffer.get(uIndex)
                col++
            }
        }

        return nv21
    }

    private fun rotateNv21(data: ByteArray, width: Int, height: Int, degrees: Int): ByteArray {
        if (degrees == 0) return data
        val output = ByteArray(data.size)
        val ySize = width * height
        var position = 0

        when (degrees) {
            90 -> {
                for (x in 0 until width) {
                    for (y in height - 1 downTo 0) output[position++] = data[y * width + x]
                }
                for (x in 0 until width step 2) {
                    for (y in height / 2 - 1 downTo 0) {
                        val source = ySize + y * width + x
                        output[position++] = data[source]
                        output[position++] = data[source + 1]
                    }
                }
            }
            180 -> {
                for (index in ySize - 1 downTo 0) output[position++] = data[index]
                for (index in data.size - 2 downTo ySize step 2) {
                    output[position++] = data[index]
                    output[position++] = data[index + 1]
                }
            }
            270 -> {
                for (x in width - 1 downTo 0) {
                    for (y in 0 until height) output[position++] = data[y * width + x]
                }
                for (x in width - 2 downTo 0 step 2) {
                    for (y in 0 until height / 2) {
                        val source = ySize + y * width + x
                        output[position++] = data[source]
                        output[position++] = data[source + 1]
                    }
                }
            }
            else -> return data
        }

        return output
    }
}
