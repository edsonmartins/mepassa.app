package com.mepassa.core

import android.content.Context
import android.net.Uri
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.mepassa.MePassaClient
import java.io.File
import java.io.FileOutputStream

/**
 * ViewModel for managing media (images) selection and upload
 */
class MediaPickerViewModel(
    private val client: MePassaClient,
    private val context: Context
) : ViewModel() {

    private val _selectedImages = MutableStateFlow<List<MediaItem>>(emptyList())
    val selectedImages: StateFlow<List<MediaItem>> = _selectedImages.asStateFlow()

    private val _uploadState = MutableStateFlow<UploadState>(UploadState.Idle)
    val uploadState: StateFlow<UploadState> = _uploadState.asStateFlow()

    /**
     * Add images from URIs (from Photo Picker)
     */
    fun addImages(uris: List<Uri>) {
        viewModelScope.launch {
            val newItems = uris.map { uri ->
                MediaItem(
                    uri = uri,
                    type = MediaType.IMAGE,
                    fileName = getFileName(uri),
                    fileSize = getFileSize(uri)
                )
            }
            _selectedImages.value = _selectedImages.value + newItems
        }
    }

    /**
     * Remove an image from selection
     */
    fun removeImage(uri: Uri) {
        _selectedImages.value = _selectedImages.value.filterNot { it.uri == uri }
    }

    /**
     * Clear all selected images
     */
    fun clearSelection() {
        _selectedImages.value = emptyList()
    }

    /**
     * Upload selected images to a conversation
     */
    fun uploadImages(conversationId: String) {
        if (_selectedImages.value.isEmpty()) return

        viewModelScope.launch {
            _uploadState.value = UploadState.Uploading(0, _selectedImages.value.size)

            try {
                _selectedImages.value.forEachIndexed { index, mediaItem ->
                    uploadSingleImage(conversationId, mediaItem)

                    _uploadState.value = UploadState.Uploading(
                        current = index + 1,
                        total = _selectedImages.value.size
                    )
                }

                _uploadState.value = UploadState.Success
                clearSelection()
            } catch (e: Exception) {
                _uploadState.value = UploadState.Error(e.message ?: "Upload failed")
            }
        }
    }

    /**
     * Upload a single image (with compression)
     */
    private suspend fun uploadSingleImage(conversationId: String, mediaItem: MediaItem) {
        withContext(Dispatchers.IO) {
            // Read image bytes from URI
            val inputStream = context.contentResolver.openInputStream(mediaItem.uri)
                ?: throw Exception("Failed to open image: ${mediaItem.uri}")

            val imageBytes = inputStream.use { it.readBytes() }

            // TODO: Call FFI method to upload image with compression
            // For now, just save to temp file
            val tempFile = File(context.cacheDir, "upload_${System.currentTimeMillis()}.jpg")
            FileOutputStream(tempFile).use { output ->
                output.write(imageBytes)
            }

            // In the future:
            // client.sendMediaMessage(
            //     conversationId = conversationId,
            //     mediaType = "image",
            //     data = imageBytes,
            //     fileName = mediaItem.fileName
            // )
        }
    }

    /**
     * Get file name from URI
     */
    private fun getFileName(uri: Uri): String? {
        return context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
            val nameIndex = cursor.getColumnIndex(android.provider.OpenableColumns.DISPLAY_NAME)
            if (cursor.moveToFirst() && nameIndex != -1) {
                cursor.getString(nameIndex)
            } else {
                null
            }
        }
    }

    /**
     * Get file size from URI
     */
    private fun getFileSize(uri: Uri): Long? {
        return context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
            val sizeIndex = cursor.getColumnIndex(android.provider.OpenableColumns.SIZE)
            if (cursor.moveToFirst() && sizeIndex != -1) {
                cursor.getLong(sizeIndex)
            } else {
                null
            }
        }
    }

    /**
     * Reset upload state
     */
    fun resetUploadState() {
        _uploadState.value = UploadState.Idle
    }
}

/**
 * Media item (image, video, file)
 */
data class MediaItem(
    val uri: Uri,
    val type: MediaType,
    val fileName: String?,
    val fileSize: Long?
)

/**
 * Media type
 */
enum class MediaType {
    IMAGE,
    VIDEO,
    DOCUMENT,
    AUDIO
}

/**
 * Upload state
 */
sealed class UploadState {
    object Idle : UploadState()
    data class Uploading(val current: Int, val total: Int) : UploadState()
    object Success : UploadState()
    data class Error(val message: String) : UploadState()
}
