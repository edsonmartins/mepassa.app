package com.zaplivre.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/**
 * Visualizador de ondas de voz em tempo real, estilo WhatsApp/Telegram:
 * barras verticais centralizadas cuja altura reage à amplitude do áudio.
 *
 * Recebe a lista de amplitudes normalizadas (0..1) e renderiza uma barra por
 * amostra. As barras mais recentes ficam à direita e, quando a lista excede a
 * largura disponível, as antigas "rolam" para fora pela esquerda.
 */
@Composable
fun WaveformVisualizer(
    amplitudes: List<Float>,
    modifier: Modifier = Modifier,
    barColor: Color = Color(0xFF4CAF50),
    barWidth: Dp = 3.dp,
    barGap: Dp = 2.dp,
    minOpacity: Float = 0.25f,
) {
    if (amplitudes.isEmpty()) return

    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .fillMaxHeight()
    ) {
        val step = barWidth.toPx() + barGap.toPx()
        val availHeight = size.height

        // Quantas barras cabem na largura disponível
        val maxVisible = (size.width / step).toInt().coerceAtLeast(1)

        // Mantém apenas as últimas `maxVisible` amostras (rolagem da direita)
        val visible = amplitudes.takeLast(maxVisible)

        // Desenha começando no canto direito, avançando para a esquerda
        visible.forEachIndexed { i, amp ->
            val normalized = amp.coerceIn(0f, 1f)
            // Ganho: amplifica baixas amplitudes para o waveform ficar visível
            val boosted = (normalized * 1.6f + 0.1f).coerceIn(0f, 1f)
            val barHeight = (boosted * availHeight).coerceIn(8f, availHeight)
            val x = size.width - (visible.size - i) * step

            val opacity = (0.3f + (i.toFloat() / visible.size) * 0.7f)
                .coerceIn(minOpacity, 1f)

            drawLine(
                color = barColor.copy(alpha = opacity),
                start = Offset(x, (availHeight - barHeight) / 2f),
                end = Offset(x, (availHeight + barHeight) / 2f),
                strokeWidth = barWidth.toPx(),
                cap = StrokeCap.Round,
            )
        }

        // Barra "live" fantasma na borda direita enquanto a ondulação ainda rola
        val liveX = size.width - barWidth.toPx() / 2f
        drawRect(
            color = barColor.copy(alpha = 0.15f),
            topLeft = Offset(liveX - barWidth.toPx() / 2f, (availHeight - 8f) / 2f),
            size = Size(barWidth.toPx(), 8f),
        )
    }
}