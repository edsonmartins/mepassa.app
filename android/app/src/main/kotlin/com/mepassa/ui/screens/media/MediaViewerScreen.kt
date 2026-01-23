package com.mepassa.ui.screens.media

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uniffi.mepassa.FfiMedia

/**
 * MediaViewerScreen - Fullscreen media viewer
 *
 * TODO: Implement full media viewer with:
 * - Image/video playback
 * - Zoom and pan gestures
 * - Horizontal swipe between media items
 * - Share and download options
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MediaViewerScreen(
    mediaItems: List<FfiMedia>,
    initialIndex: Int = 0,
    onNavigateBack: () -> Unit
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Media Viewer") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            imageVector = Icons.Filled.ArrowBack,
                            contentDescription = "Back"
                        )
                    }
                }
            )
        }
    ) { padding ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding),
            contentAlignment = Alignment.Center
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                Text(
                    text = "Media Viewer",
                    style = MaterialTheme.typography.headlineMedium
                )
                Text(
                    text = "Full media viewer to be implemented",
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Text(
                    text = "${mediaItems.size} media items",
                    style = MaterialTheme.typography.bodyMedium
                )
            }
        }
    }
}
