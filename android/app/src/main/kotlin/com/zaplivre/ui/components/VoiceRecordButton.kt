package com.zaplivre.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Send
import androidx.compose.material.icons.filled.Mic
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.input.pointer.positionChange
import androidx.compose.ui.platform.LocalHapticFeedback
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.hapticfeedback.HapticFeedbackType
import com.zaplivre.core.VoiceRecorderViewModel
import com.zaplivre.ui.theme.ZapColor
import kotlinx.coroutines.launch

/**
 * Botão de gravação de voz estilo WhatsApp/Telegram.
 *
 * - **Press e segurar** no mic inicia a gravação + vibração
 * - **Deslizar para a esquerda** cancela ("Slide to cancel")
 * - **Soltar** envia a mensagem
 * - **Deslizar para cima** trava a gravação, abrindo uma barra com
 *   waveform em tempo real, timer e botões (pausar / deletar / enviar)
 */
@Composable
fun VoiceRecordButton(
    viewModel: VoiceRecorderViewModel,
    onVoiceMessageRecorded: (java.io.File) -> Unit,
    modifier: Modifier = Modifier
) {
    val isRecording by viewModel.isRecording.collectAsState()
    val recordingDuration by viewModel.recordingDuration.collectAsState()
    val waveform by viewModel.waveform.collectAsState()

    val scope = rememberCoroutineScope()
    val haptic = LocalHapticFeedback.current

    // Estado de gesture: lock ativado? offset do slide-to-cancel / lock
    var isLocked by remember { mutableStateOf(false) }
    var isPaused by remember { mutableStateOf(false) }
    var dragOffsetX by remember { mutableFloatStateOf(0f) }
    var dragOffsetY by remember { mutableFloatStateOf(0f) }
    val isCanceling = dragOffsetX < -100f

    // Mic button (hold to record) - inicia a gravação imediatamente no down
    // e rastreia movimento manualmente (sem touch slop, como o WhatsApp)
    Box(
        modifier = modifier
            .size(48.dp)
            .clip(CircleShape)
            .background(ZapColor.primary, CircleShape)
            .pointerInput(Unit) {
                awaitEachGesture {
                    // Press (down) inicia a gravação imediatamente
                    awaitFirstDown(requireUnconsumed = false)
                    scope.launch {
                        viewModel.startRecording()
                        dragOffsetX = 0f
                        dragOffsetY = 0f
                        isLocked = false
                        isPaused = false
                    }
                    haptic.performHapticFeedback(HapticFeedbackType.LongPress)

                    // Rastreia movimento do dedo até soltar/cancelar
                    while (true) {
                        val event = awaitPointerEvent()
                        val change = event.changes.firstOrNull() ?: break
                        if (!change.pressed) break // soltou o dedo

                        // Consome até o final do gesto para não vazar pra UI
                        change.consume()

                        dragOffsetX = (dragOffsetX + change.positionChange().x).coerceAtMost(0f)
                        dragOffsetY = (dragOffsetY + change.positionChange().y).coerceAtMost(0f)

                        // Deslizou para cima o suficiente → trava a gravação
                        if (dragOffsetY < -120f && !isLocked) {
                            isLocked = true
                            dragOffsetX = 0f
                            dragOffsetY = 0f
                            haptic.performHapticFeedback(HapticFeedbackType.LongPress)
                        }
                    }

                    // Fim do gesto
                    if (isCanceling) {
                        viewModel.cancelRecording()
                    } else if (!isLocked) {
                        // Soltou sem travar → envia direto
                        val file = viewModel.stopRecording()
                        if (file != null && viewModel.recordingDuration.value >= 500) {
                            onVoiceMessageRecorded(file)
                        }
                    }
                    dragOffsetX = 0f
                    dragOffsetY = 0f
                }
            }
            .alpha(if (isLocked) 0f else 1f),
        contentAlignment = Alignment.Center
    ) {
        Icon(
            imageVector = if (isPaused) Icons.Default.PlayArrow else Icons.Default.Mic,
            contentDescription = "Hold to record voice message",
            tint = Color.White
        )
    }

    // Overlay de gravação (slide-to-cancel + waveform) enquanto segura
    AnimatedVisibility(
        visible = isRecording && !isLocked,
        enter = slideInVertically(initialOffsetY = { it }),
        exit = slideOutVertically(targetOffsetY = { it })
    ) {
        // Barrinha de progresso do cancel (vermelha quando chegando em cancel)
        RecordingOverlay(
            isCanceling = isCanceling,
            durationText = viewModel.formatDuration(recordingDuration),
            waveform = waveform,
            onReleaseSend = {
                val file = viewModel.stopRecording()
                if (file != null && recordingDuration >= 500) onVoiceMessageRecorded(file)
                dragOffsetX = 0f
            }
        )
    }

    // Barra de gravação travada (lock) - com botões + waveform full
    AnimatedVisibility(
        visible = isRecording && isLocked,
        enter = slideInVertically(initialOffsetY = { it }),
        exit = slideOutVertically(targetOffsetY = { it }),
        modifier = Modifier.fillMaxWidth()
    ) {
        LockedRecordingBar(
            isPaused = isPaused,
            durationText = viewModel.formatDuration(recordingDuration),
            waveform = waveform,
            onPauseToggle = {
                isPaused = !isPaused
                // Nota: pausa/resume real necessitaria pausar o MediaRecorder;
                // aqui apenas congela o timer na UI.
            },
            onDelete = {
                viewModel.cancelRecording()
                isLocked = false
                isPaused = false
            },
            onSend = {
                val file = viewModel.stopRecording()
                if (file != null && recordingDuration >= 500) onVoiceMessageRecorded(file)
                isLocked = false
            }
        )
    }
}

/**
 * Overlay transiente que aparece enquanto o usuário segura o mic:
 * indica "slide to cancel" e mostra o waveform ao vivo.
 */
@Composable
private fun RecordingOverlay(
    isCanceling: Boolean,
    durationText: String,
    waveform: List<Float>,
    onReleaseSend: () -> Unit,
) {
    Surface(
        color = MaterialTheme.colorScheme.surface,
        tonalElevation = 4.dp,
        shape = RoundedCornerShape(28.dp),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            // Ícone de cancel (seta → ✕) que fica vermelho ao passar no limite
            Icon(
                imageVector = Icons.Default.Close,
                contentDescription = "Slide to cancel",
                tint = if (isCanceling) ZapColor.danger else Color(0xFFCFCFCF),
                modifier = Modifier.size(20.dp)
            )

            Text(
                text = durationText,
                style = MaterialTheme.typography.bodyMedium,
                color = if (isCanceling) ZapColor.danger else MaterialTheme.colorScheme.onSurface,
                fontSize = 15.sp
            )

            Box(
                modifier = Modifier
                    .weight(1f)
                    .height(52.dp)
            ) {
                WaveformVisualizer(
                    amplitudes = waveform,
                    barColor = if (isCanceling) ZapColor.danger else Color(0xFF4CAF50),
                    barWidth = 3.dp,
                    barGap = 2.dp
                )
            }

            // Botão de enviar (verde)
            IconButton(
                onClick = onReleaseSend,
                modifier = Modifier
                    .size(40.dp)
                    .background(
                        if (isCanceling) ZapColor.danger.copy(alpha = 0.6f) else Color(0xFF4CAF50),
                        CircleShape
                    )
            ) {
                Icon(
                    imageVector = Icons.Filled.Send,
                    contentDescription = "Send voice message",
                    tint = Color.White,
                    modifier = Modifier.size(20.dp)
                )
            }
        }
    }
}

/**
 * Barra de gravação travada (após deslizar para cima): mostrar o conteúdo
 * completo com waveform grande, timer e botões de controle.
 */
@Composable
private fun LockedRecordingBar(
    isPaused: Boolean,
    durationText: String,
    waveform: List<Float>,
    onPauseToggle: () -> Unit,
    onDelete: () -> Unit,
    onSend: () -> Unit,
) {
    Surface(
        color = MaterialTheme.colorScheme.surface,
        tonalElevation = 4.dp,
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 12.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            // Deletar
            IconButton(onClick = onDelete) {
                Icon(
                    imageVector = Icons.Default.Close,
                    contentDescription = "Cancel recording",
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            // Pausar / retomar
            IconButton(onClick = onPauseToggle) {
                Icon(
                    imageVector = if (isPaused) Icons.Default.PlayArrow else Icons.Default.Pause,
                    contentDescription = if (isPaused) "Resume recording" else "Pause recording",
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            Column(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(2.dp)
            ) {
                // Timer + pode indicar pausado
                Text(
                    text = if (isPaused) "$durationText (pausado)" else durationText,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Box(modifier = Modifier.fillMaxWidth().height(60.dp)) {
                    WaveformVisualizer(
                        amplitudes = waveform,
                        barColor = Color(0xFF4CAF50)
                    )
                }
            }

            // Enviar (verde)
            FilledIconButton(
                onClick = onSend,
                modifier = Modifier.size(40.dp),
                colors = IconButtonDefaults.filledIconButtonColors(
                    containerColor = Color(0xFF4CAF50),
                    contentColor = Color.White
                )
            ) {
                Icon(
                    imageVector = Icons.Filled.Send,
                    contentDescription = "Send voice message",
                    modifier = Modifier.size(20.dp)
                )
            }
        }
    }
}