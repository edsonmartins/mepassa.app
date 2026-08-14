package com.zaplivre.ui.components

import android.graphics.Bitmap
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.google.zxing.BarcodeFormat
import com.google.zxing.EncodeHintType
import com.google.zxing.qrcode.QRCodeWriter

/** QR interoperável: o conteúdo é o Peer ID puro, formato aceito pelo scanner iOS. */
@Composable
fun PeerQrCode(
    payload: String,
    modifier: Modifier = Modifier,
    size: Dp = 240.dp,
) {
    val bitmap = remember(payload) {
        payload.takeIf { it.isNotBlank() }?.let { runCatching { encodePeerId(it) }.getOrNull() }
    }

    Box(
        modifier = modifier
            .size(size)
            .background(Color.White)
            .padding(12.dp)
            .testTag("qr_image"),
        contentAlignment = Alignment.Center,
    ) {
        if (bitmap != null) {
            Image(
                bitmap = bitmap.asImageBitmap(),
                contentDescription = "QR Code do meu Peer ID",
                modifier = Modifier.size(size - 24.dp),
            )
        } else {
            Text("Identidade indisponível", style = MaterialTheme.typography.bodySmall)
        }
    }
}

private fun encodePeerId(peerId: String): Bitmap {
    val pixels = 1200
    val matrix = QRCodeWriter().encode(
        peerId,
        BarcodeFormat.QR_CODE,
        pixels,
        pixels,
        mapOf(EncodeHintType.MARGIN to 2),
    )
    return Bitmap.createBitmap(pixels, pixels, Bitmap.Config.ARGB_8888).apply {
        val colors = IntArray(pixels * pixels)
        for (y in 0 until pixels) {
            for (x in 0 until pixels) {
                colors[y * pixels + x] = if (matrix[x, y]) android.graphics.Color.BLACK else android.graphics.Color.WHITE
            }
        }
        setPixels(colors, 0, pixels, 0, 0, pixels, pixels)
    }
}
