package com.mepassa.ui.screens.chat

import android.net.Uri
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.Send
import androidx.compose.material.icons.filled.Phone
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.mepassa.R
import com.mepassa.core.MePassaClientWrapper
import com.mepassa.core.VoiceRecorderViewModel
import com.mepassa.ui.components.ImagePickerButton
import com.mepassa.ui.components.SelectedImagesPreview
import com.mepassa.ui.components.VoiceRecordButton
import kotlinx.coroutines.launch
import uniffi.mepassa.FfiMessage
import java.text.SimpleDateFormat
import java.util.*

/**
 * ChatScreen - Tela de conversa individual
 *
 * Exibe mensagens trocadas com um peer específico.
 * Permite enviar novas mensagens de texto.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ChatScreen(
    peerId: String,
    onNavigateBack: () -> Unit,
    onStartCall: () -> Unit
) {
    val scope = rememberCoroutineScope()
    val listState = rememberLazyListState()
    val context = androidx.compose.ui.platform.LocalContext.current

    var messages by remember { mutableStateOf<List<FfiMessage>>(emptyList()) }
    var messageInput by remember { mutableStateOf("") }
    var isSending by remember { mutableStateOf(false) }
    val localPeerId by MePassaClientWrapper.localPeerId.collectAsState()

    // Image selection state
    var selectedImages by remember { mutableStateOf<List<Uri>>(emptyList()) }

    // Voice recorder
    val voiceRecorderViewModel = remember { VoiceRecorderViewModel(context) }

    // Carregar mensagens
    LaunchedEffect(peerId) {
        scope.launch {
            messages = MePassaClientWrapper.getConversationMessages(peerId)
            // Scroll para última mensagem
            if (messages.isNotEmpty()) {
                listState.animateScrollToItem(messages.lastIndex)
            }
        }
    }

    // Recarregar mensagens periodicamente
    LaunchedEffect(peerId) {
        while (true) {
            kotlinx.coroutines.delay(2000) // A cada 2 segundos
            scope.launch {
                val newMessages = MePassaClientWrapper.getConversationMessages(peerId)
                if (newMessages.size > messages.size) {
                    messages = newMessages
                    // Auto-scroll se nova mensagem
                    listState.animateScrollToItem(messages.lastIndex)
                }
            }
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text(
                            text = peerId.take(16) + "...",
                            style = MaterialTheme.typography.titleMedium,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                        Text(
                            text = stringResource(R.string.chat_status_connected),
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = "Voltar"
                        )
                    }
                },
                actions = {
                    // Botão de chamada de voz
                    IconButton(onClick = onStartCall) {
                        Icon(
                            imageVector = Icons.Default.Phone,
                            contentDescription = "Iniciar chamada",
                            tint = MaterialTheme.colorScheme.onPrimaryContainer
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.onPrimaryContainer
                )
            )
        },
        bottomBar = {
            Column {
                // Selected images preview
                if (selectedImages.isNotEmpty()) {
                    SelectedImagesPreview(
                        selectedImages = selectedImages.map { uri ->
                            com.mepassa.core.MediaItem(
                                uri = uri,
                                type = com.mepassa.core.MediaType.IMAGE,
                                fileName = null,
                                fileSize = null
                            )
                        },
                        onRemoveImage = { uri ->
                            selectedImages = selectedImages.filterNot { it == uri }
                        },
                        onSendImages = {
                            scope.launch {
                                try {
                                    // Send each selected image via FFI
                                    selectedImages.forEach { uri ->
                                        val inputStream = context.contentResolver.openInputStream(uri)
                                        if (inputStream != null) {
                                            val imageBytes = inputStream.use { it.readBytes() }
                                            val fileName = uri.lastPathSegment ?: "image_${System.currentTimeMillis()}.jpg"

                                            // Call FFI to send image with compression
                                            MePassaClientWrapper.client?.sendImageMessage(
                                                toPeerId = peerId,
                                                imageData = imageBytes.toUByteArray().toList(),
                                                fileName = fileName,
                                                quality = 85u
                                            )
                                        }
                                    }

                                    // Clear selection after sending
                                    selectedImages = emptyList()

                                    // Reload messages to show sent images
                                    messages = MePassaClientWrapper.getConversationMessages(peerId)
                                    if (messages.isNotEmpty()) {
                                        listState.animateScrollToItem(messages.lastIndex)
                                    }
                                } catch (e: Exception) {
                                    // TODO: Show error to user
                                    println("Error sending images: ${e.message}")
                                }
                            }
                        }
                    )
                }

                // Message input bar
                MessageInputBar(
                    messageInput = messageInput,
                    onMessageInputChange = { messageInput = it },
                    onSendClick = {
                        if (messageInput.isNotBlank() && !isSending) {
                            val content = messageInput.trim()
                            messageInput = ""
                            isSending = true

                            scope.launch {
                                val result = MePassaClientWrapper.sendTextMessage(peerId, content)
                                isSending = false

                                if (result.isSuccess) {
                                    // Recarregar mensagens
                                    messages = MePassaClientWrapper.getConversationMessages(peerId)
                                    listState.animateScrollToItem(messages.lastIndex)
                                } else {
                                    // TODO: Mostrar erro
                                }
                            }
                        }
                    },
                    onSelectImages = { uris ->
                        selectedImages = selectedImages + uris
                    },
                    onVoiceMessageRecorded = { audioFile ->
                        scope.launch {
                            try {
                                // Read audio file bytes
                                val audioBytes = audioFile.readBytes()
                                val durationSeconds = (audioFile.length() / 16000).toInt() // Rough estimate

                                // Call FFI to send voice message
                                MePassaClientWrapper.client?.sendVoiceMessage(
                                    toPeerId = peerId,
                                    audioData = audioBytes.toUByteArray().toList(),
                                    fileName = audioFile.name,
                                    durationSeconds = durationSeconds
                                )

                                // Reload messages to show sent voice message
                                messages = MePassaClientWrapper.getConversationMessages(peerId)
                                if (messages.isNotEmpty()) {
                                    listState.animateScrollToItem(messages.lastIndex)
                                }
                            } catch (e: Exception) {
                                // TODO: Show error to user
                                println("Error sending voice message: ${e.message}")
                            }
                        }
                    },
                    voiceRecorderViewModel = voiceRecorderViewModel,
                    isSending = isSending
                )
            }
        }
    ) { paddingValues ->
        if (messages.isEmpty()) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(paddingValues),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = "Nenhuma mensagem ainda.\nEnvie a primeira!",
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        } else {
            LazyColumn(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(paddingValues),
                state = listState,
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(messages) { message ->
                    MessageBubble(
                        message = message,
                        isOwnMessage = message.senderPeerId == localPeerId
                    )
                }
            }
        }
    }
}

/**
 * Barra de input de mensagem
 */
@Composable
fun MessageInputBar(
    messageInput: String,
    onMessageInputChange: (String) -> Unit,
    onSendClick: () -> Unit,
    onSelectImages: (List<Uri>) -> Unit,
    onVoiceMessageRecorded: (java.io.File) -> Unit,
    voiceRecorderViewModel: VoiceRecorderViewModel,
    isSending: Boolean
) {
    Surface(
        tonalElevation = 3.dp,
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .padding(8.dp)
                .fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            // Image picker button
            ImagePickerButton(
                onImagesPicked = onSelectImages,
                maxSelection = 10,
                enabled = !isSending
            )

            OutlinedTextField(
                value = messageInput,
                onValueChange = onMessageInputChange,
                modifier = Modifier.weight(1f),
                placeholder = {
                    Text(stringResource(R.string.chat_input_hint))
                },
                maxLines = 4,
                enabled = !isSending,
                shape = RoundedCornerShape(24.dp)
            )

            // Send button or Voice record button
            if (messageInput.isNotBlank()) {
                IconButton(
                    onClick = onSendClick,
                    enabled = !isSending
                ) {
                    if (isSending) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(24.dp),
                            strokeWidth = 2.dp
                        )
                    } else {
                        Icon(
                            Icons.AutoMirrored.Filled.Send,
                            contentDescription = stringResource(R.string.chat_send),
                            tint = MaterialTheme.colorScheme.primary
                        )
                    }
                }
            } else {
                VoiceRecordButton(
                    viewModel = voiceRecorderViewModel,
                    onVoiceMessageRecorded = onVoiceMessageRecorded
                )
            }
        }
    }
}

/**
 * Bolha de mensagem individual
 */
@Composable
fun MessageBubble(
    message: FfiMessage,
    isOwnMessage: Boolean
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = if (isOwnMessage) Arrangement.End else Arrangement.Start
    ) {
        Surface(
            shape = RoundedCornerShape(
                topStart = 16.dp,
                topEnd = 16.dp,
                bottomStart = if (isOwnMessage) 16.dp else 4.dp,
                bottomEnd = if (isOwnMessage) 4.dp else 16.dp
            ),
            color = if (isOwnMessage) {
                MaterialTheme.colorScheme.primaryContainer
            } else {
                MaterialTheme.colorScheme.surfaceVariant
            },
            modifier = Modifier.widthIn(max = 280.dp)
        ) {
            Column(
                modifier = Modifier.padding(12.dp)
            ) {
                message.contentPlaintext?.let { content ->
                    Text(
                        text = content,
                        style = MaterialTheme.typography.bodyMedium
                    )
                }

                Spacer(modifier = Modifier.height(4.dp))

                Text(
                    text = formatMessageTime(message.createdAt),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
    }
}

/**
 * Formata timestamp da mensagem (HH:mm)
 */
private fun formatMessageTime(timestamp: Long): String {
    val date = Date(timestamp * 1000)
    return SimpleDateFormat("HH:mm", Locale.getDefault()).format(date)
}
