package com.zaplivre.ui.components

import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.tween
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
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.changedToUp
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.input.pointer.positionChange
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.zaplivre.core.VoiceRecorderViewModel
import com.zaplivre.ui.theme.ZapColor
import kotlinx.coroutines.launch

/**
 * Controle de gravação de voz estilo WhatsApp/Telegram.
 *
 * Um único gesture estável (não recria ao mudar de estado) que:
 * - **Press** sobre o mic inicia a gravação imediatamente.
 * - **Deslizar para a esquerda** coloca em zona de cancelamento (UI fica vermelha).
 * - **Soltar** em zona de cancelamento → cancela; caso contrário → envia.
 * - **Deslizar para cima** trava (lock) → mantém a barra aberta com botões.
 */
@Composable
fun VoiceRecordingInlineBar(
    viewModel: VoiceRecorderViewModel,
    onVoiceMessageRecorded: (java.io.File) -> Unit,
    modifier: Modifier = Modifier
) {
    val isRecording by viewModel.isRecording.collectAsState()
    val recordingDuration by viewModel.recordingDuration.collectAsState()
    val waveform by viewModel.waveform.collectAsState()

    val scope = rememberCoroutineScope()
    var isLocked by remember { mutableStateOf(false) }

    // Progresso do drag horizontal (negativo = deslizou para a esquerda);
    // atualizado pelo gesture para recompor a UI ao vivo.
    var dragProgress by remember { mutableFloatStateOf(0f) }
    val isCanceling = !isLocked && dragProgress < -60f

    val isExpanded = isLocked || isRecording

    Box(
        modifier = modifier
            .then(if (isExpanded) Modifier.fillMaxWidth().height(48.dp)
                  else Modifier.size(48.dp))
            .clip(RoundedCornerShape(if (isExpanded) 28.dp else 24.dp))
            .background(
                when {
                    isExpanded && isCanceling -> ZapColor.danger.copy(alpha = 0.15f)
                    isExpanded -> MaterialTheme.colorScheme.surface
                    else -> ZapColor.primary
                },
                if (isExpanded) RoundedCornerShape(28.dp) else CircleShape
            )
            .pointerInput(Unit) {
                awaitEachGesture {
                    awaitFirstDown(requireUnconsumed = false)

                    // Inicia a gravação imediatamente no press, se ainda não gravando
                    if (!viewModel.isRecording.value) {
                        scope.launch {
                            viewModel.startRecording()
                            isLocked = false
                        }
                    }
                    dragProgress = 0f

                    var latestX = 0f
                    var latestY = 0f
                    while (true) {
                        val event = awaitPointerEvent()
                        val change = event.changes.firstOrNull() ?: break
                        if (change.changedToUp()) break // soltou o dedo
                        change.consume()

                        if (!isLocked) {
                            latestX = (latestX + change.positionChange().x).coerceAtMost(0f)
                            latestY = (latestY + change.positionChange().y).coerceAtMost(0f)
                            dragProgress = latestX

                            // Deslizou para cima o suficiente → trava
                            if (latestX > -60f && latestY < -120f) {
                                isLocked = true
                                dragProgress = 0f
                            }
                        }
                    }

                    // Fim do gesture: decide o destino (se não estiver travado)
                    if (!isLocked) {
                        if (latestX < -60f) {
                            viewModel.cancelRecording()
                        } else {
                            val file = viewModel.stopRecording()
                            if (file != null && viewModel.recordingDuration.value >= 500) {
                                onVoiceMessageRecorded(file)
                            }
                        }
                    }
                    dragProgress = 0f
                }
            },
        contentAlignment = Alignment.Center
    ) {
        if (isExpanded) {
            // ===== Barra de gravação =====
            Row(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(horizontal = 12.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Close,
                    contentDescription = "Slide to cancel",
                    tint = if (isCanceling) ZapColor.danger else Color(0xFFCFCFCF),
                    modifier = Modifier.size(20.dp)
                )
                Text(
                    text = viewModel.formatDuration(recordingDuration),
                    style = MaterialTheme.typography.bodyMedium,
                    color = if (isCanceling) ZapColor.danger
                        else MaterialTheme.colorScheme.onSurface,
                    fontSize = 15.sp
                )
                Box(
                    modifier = Modifier
                        .weight(1f)
                        .fillMaxHeight()
                ) {
                    WaveformVisualizer(
                        amplitudes = waveform,
                        barColor = if (isCanceling) ZapColor.danger else Color(0xFF4CAF50)
                    )
                }
                IconButton(
                    onClick = {
                        val file = viewModel.stopRecording()
                        if (file != null && viewModel.recordingDuration.value >= 500) {
                            onVoiceMessageRecorded(file)
                        }
                        isLocked = false
                    },
                    modifier = Modifier
                        .size(40.dp)
                        .background(if (isCanceling) ZapColor.danger else Color(0xFF4CAF50), CircleShape)
                ) {
                    Icon(
                        imageVector = Icons.Default.Send,
                        contentDescription = "Send voice message",
                        tint = Color.White,
                        modifier = Modifier.size(18.dp)
                    )
                }
            }
        } else {
            // ===== Botão de mic (ocioso) =====
            Icon(
                imageVector = Icons.Default.Mic,
                contentDescription = "Hold to record voice message",
                tint = Color.White
            )
        }
    }
}