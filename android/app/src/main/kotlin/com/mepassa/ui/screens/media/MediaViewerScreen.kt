package com.mepassa.ui.screens.media

import android.graphics.BitmapFactory
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Download
import androidx.compose.material.icons.filled.Share
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import com.google.accompanist.pager.ExperimentalPagerApi
import com.google.accompanist.pager.HorizontalPager
import com.google.accompanist.pager.rememberPagerState
import com.mepassa.core.MePassaCore
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.mepassa.FfiMedia
import uniffi.mepassa.FfiMediaType

/**
 * MediaViewerScreen - Fullscreen media viewer with zoom and swipe
 */
@OptIn(ExperimentalMaterial3Api::class, ExperimentalPagerApi::class)
@Composable
fun MediaViewerScreen(
    mediaItems: List<FfiMedia>,
    initialIndex: Int,
    onBack: () -> Unit,
    modifier: Modifier = Modifier
) {
    val scope = rememberCoroutineScope()
    val pagerState = rememberPagerState(initialPage = initialIndex)
    val currentMedia = mediaItems.getOrNull(pagerState.currentPage)

    var showUI by remember { mutableStateOf(true) }

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(Color.Black)
    ) {
        // Horizontal pager for swiping between media
        HorizontalPager(
            count = mediaItems.size,
            state = pagerState,
            modifier = Modifier.fillMaxSize()
        ) { page ->
            val media = mediaItems[page]

            when (media.mediaType) {
                FfiMediaType.IMAGE -> {
                    ZoomableImage(
                        media = media,
                        onToggleUI = { showUI = !showUI }
                    )
                }
                FfiMediaType.VIDEO -> {
                    VideoPlayerView(
                        media = media,
                        onToggleUI = { showUI = !showUI }
                    )
                }
                else -> {
                    // Unsupported media type
                    Box(
                        modifier = Modifier.fillMaxSize(),
                        contentAlignment = Alignment.Center
                    ) {
                        Text(
                            text = "Tipo de mídia não suportado",
                            color = Color.White,
                            style = MaterialTheme.typography.bodyLarge
                        )
                    }
                }
            }
        }

        // Top bar (overlay)
        if (showUI) {
            TopAppBar(
                title = {
                    Column {
                        Text(
                            text = currentMedia?.fileName ?: "Mídia",
                            color = Color.White
                        )
                        Text(
                            text = "${pagerState.currentPage + 1} de ${mediaItems.size}",
                            color = Color.White.copy(alpha = 0.7f),
                            style = MaterialTheme.typography.bodySmall
                        )
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(
                            Icons.Default.ArrowBack,
                            contentDescription = "Back",
                            tint = Color.White
                        )
                    }
                },
                actions = {
                    // Share button
                    IconButton(onClick = {
                        // TODO: Implement share functionality
                        println("Share media: ${currentMedia?.fileName}")
                    }) {
                        Icon(
                            Icons.Default.Share,
                            contentDescription = "Share",
                            tint = Color.White
                        )
                    }

                    // Download button
                    IconButton(onClick = {
                        scope.launch {
                            currentMedia?.let { media ->
                                downloadMedia(media)
                            }
                        }
                    }) {
                        Icon(
                            Icons.Default.Download,
                            contentDescription = "Download",
                            tint = Color.White
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color.Black.copy(alpha = 0.7f)
                ),
                modifier = Modifier.align(Alignment.TopCenter)
            )
        }

        // Page indicator (bottom)
        if (showUI && mediaItems.size > 1) {
            Row(
                modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .padding(16.dp),
                horizontalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                repeat(mediaItems.size.coerceAtMost(10)) { index ->
                    Box(
                        modifier = Modifier
                            .size(6.dp)
                            .background(
                                color = if (index == pagerState.currentPage) {
                                    Color.White
                                } else {
                                    Color.White.copy(alpha = 0.3f)
                                },
                                shape = MaterialTheme.shapes.small
                            )
                    )
                }
            }
        }
    }
}

/**
 * ZoomableImage - Image with pinch-to-zoom and pan support
 */
@Composable
fun ZoomableImage(
    media: FfiMedia,
    onToggleUI: () -> Unit,
    modifier: Modifier = Modifier
) {
    val scope = rememberCoroutineScope()
    var bitmap by remember { mutableStateOf<android.graphics.Bitmap?>(null) }
    var isLoading by remember { mutableStateOf(true) }

    var scale by remember { mutableStateOf(1f) }
    var offset by remember { mutableStateOf(Offset.Zero) }

    // Load image
    LaunchedEffect(media.id) {
        scope.launch {
            try {
                val imageData = withContext(Dispatchers.IO) {
                    // Try local path first
                    media.localPath?.let { path ->
                        val file = java.io.File(path)
                        if (file.exists()) {
                            return@withContext file.readBytes()
                        }
                    }

                    // Download from hash
                    MePassaCore.downloadMedia(media.mediaHash)
                }

                bitmap = BitmapFactory.decodeByteArray(imageData, 0, imageData.size)
            } catch (e: Exception) {
                println("❌ Error loading image: ${e.message}")
            } finally {
                isLoading = false
            }
        }
    }

    Box(
        modifier = modifier
            .fillMaxSize()
            .pointerInput(Unit) {
                detectTapGestures(
                    onDoubleTap = {
                        // Double tap to zoom in/out
                        scale = if (scale > 1f) 1f else 2f
                        if (scale == 1f) offset = Offset.Zero
                    },
                    onTap = {
                        onToggleUI()
                    }
                )
            }
            .pointerInput(Unit) {
                detectTransformGestures { _, pan, zoom, _ ->
                    scale = (scale * zoom).coerceIn(1f, 5f)

                    if (scale > 1f) {
                        val maxX = (size.width * (scale - 1)) / 2
                        val maxY = (size.height * (scale - 1)) / 2

                        offset = Offset(
                            x = (offset.x + pan.x).coerceIn(-maxX, maxX),
                            y = (offset.y + pan.y).coerceIn(-maxY, maxY)
                        )
                    } else {
                        offset = Offset.Zero
                    }
                }
            },
        contentAlignment = Alignment.Center
    ) {
        if (isLoading) {
            CircularProgressIndicator(color = Color.White)
        } else {
            bitmap?.let { bmp ->
                Image(
                    bitmap = bmp.asImageBitmap(),
                    contentDescription = media.fileName,
                    contentScale = ContentScale.Fit,
                    modifier = Modifier
                        .fillMaxSize()
                        .graphicsLayer(
                            scaleX = scale,
                            scaleY = scale,
                            translationX = offset.x,
                            translationY = offset.y
                        )
                )
            } ?: run {
                Text(
                    text = "Erro ao carregar imagem",
                    color = Color.White,
                    style = MaterialTheme.typography.bodyLarge
                )
            }
        }
    }
}

/**
 * Download media to device storage
 */
private suspend fun downloadMedia(media: FfiMedia) {
    withContext(Dispatchers.IO) {
        try {
            val data = MePassaCore.downloadMedia(media.mediaHash)

            // Save to Downloads folder
            val downloadsDir = android.os.Environment.getExternalStoragePublicDirectory(
                android.os.Environment.DIRECTORY_DOWNLOADS
            )
            val fileName = media.fileName ?: "mepassa_${System.currentTimeMillis()}"
            val file = java.io.File(downloadsDir, fileName)

            file.writeBytes(data)
            println("✅ Media downloaded to: ${file.absolutePath}")
        } catch (e: Exception) {
            println("❌ Error downloading media: ${e.message}")
        }
    }
}
