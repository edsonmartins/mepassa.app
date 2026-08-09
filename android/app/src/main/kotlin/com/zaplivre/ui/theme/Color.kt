package com.zaplivre.ui.theme

import androidx.compose.runtime.Composable
import androidx.compose.runtime.ReadOnlyComposable
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import kotlin.math.abs

/**
 * Paleta ZapLivre (espelha ios/.../DesignSystem/ZapTheme.swift).
 *
 * A identidade: navy profundo + amarelo/âmbar energético + laranja pontual
 * (docs de marca + documentos/zaplivre-paleta-cores.md). Diferencia o produto
 * de mensageiros verdes; o gradiente de marca (amarelo→âmbar→laranja) é usado
 * com restrição — só onde o app quer chamar a ação. Tudo o mais é quieto.
 *
 * As cores adaptam light/dark via [LocalZapColors], provido pelo [ZapLivreTheme].
 * Nas telas, use `ZapColor.primary` etc (leitura em contexto @Composable),
 * exatamente como o `ZapColor` do iOS.
 */
data class ZapColors(
    val primary: Color,
    val spark: Color,
    val ink: Color,
    val slate: Color,
    val canvas: Color,
    val chatCanvas: Color,
    val surface: Color,
    val hairline: Color,
    val bubbleOut: Color,
    val bubbleOutInk: Color,
    val bubbleIn: Color,
    val bubbleInInk: Color,
    val online: Color,
    val danger: Color,
    val onPrimary: Color,
    val isDark: Boolean,
) {
    /** Gradiente de marca: amarelo → âmbar → laranja. Usar com restrição. */
    val sparkBrush: Brush
        get() = Brush.linearGradient(
            listOf(
                Color(0xFFFFD400),
                Color(0xFFFFAA00),
                Color(0xFFFF7900),
            )
        )

    /** Navy profundo — base estrutural da marca (headers, splash, superfícies). */
    val navy: Color
        get() = Color(0xFF061C3A)

    /** Paleta de avatares sem foto — cor derivada do id, para dar vida à lista. */
    val avatarPalette: List<Color>
        get() = listOf(
            Color(0xFF0B2A50), Color(0xFF7C3AED), Color(0xFF00875A),
            Color(0xFFE8618C), Color(0xFFF2884B), Color(0xFFB45309),
            Color(0xFF0EA5E9), Color(0xFFC026D3),
        )

    /**
     * Cor estável derivada de um id (djb2) — mesma lógica dos avatares.
     * Usada para avatar sem foto e nome de autor em grupos.
     */
    fun accent(seed: String): Color {
        var hash = 5381
        for (b in seed.toByteArray()) hash = ((hash shl 5) + hash) + b.toInt()
        val palette = avatarPalette
        return palette[abs(hash) % palette.size]
    }
}

val LightZapColors = ZapColors(
    primary = Color(0xFFFFB000),
    spark = Color(0xFFFFD400),
    ink = Color(0xFF0F172A),
    slate = Color(0xFF64748B),
    canvas = Color(0xFFF7F9FC),
    chatCanvas = Color(0xFFEEF2F7),
    surface = Color(0xFFFFFFFF),
    hairline = Color(0xFFE2E8F0),
    bubbleOut = Color(0xFFFFF3C4),
    bubbleOutInk = Color(0xFF0F172A),
    bubbleIn = Color(0xFFFFFFFF),
    bubbleInInk = Color(0xFF0F172A),
    online = Color(0xFF22C55E),
    danger = Color(0xFFEF4444),
    onPrimary = Color(0xFF061C3A),
    isDark = false,
)

val DarkZapColors = ZapColors(
    primary = Color(0xFFFFAA00),
    spark = Color(0xFFFFD400),
    ink = Color(0xFFF8FAFC),
    slate = Color(0xFF94A3B8),
    canvas = Color(0xFF03152E),
    chatCanvas = Color(0xFF03152E),
    surface = Color(0xFF0B2A50),
    hairline = Color(0xFF193B61),
    bubbleOut = Color(0xFF3B3214),
    bubbleOutInk = Color(0xFFF8FAFC),
    bubbleIn = Color(0xFF102B4D),
    bubbleInInk = Color(0xFFF8FAFC),
    online = Color(0xFF22C55E),
    danger = Color(0xFFEF4444),
    onPrimary = Color(0xFF061C3A),
    isDark = true,
)

val LocalZapColors = staticCompositionLocalOf { LightZapColors }

/** Acesso aos tokens ZapLivre em contexto @Composable: `ZapColor.primary`. */
object ZapColor {
    val current: ZapColors
        @Composable @ReadOnlyComposable get() = LocalZapColors.current

    val primary: Color @Composable @ReadOnlyComposable get() = current.primary
    val spark: Color @Composable @ReadOnlyComposable get() = current.spark
    val ink: Color @Composable @ReadOnlyComposable get() = current.ink
    val slate: Color @Composable @ReadOnlyComposable get() = current.slate
    val canvas: Color @Composable @ReadOnlyComposable get() = current.canvas
    val chatCanvas: Color @Composable @ReadOnlyComposable get() = current.chatCanvas
    val surface: Color @Composable @ReadOnlyComposable get() = current.surface
    val hairline: Color @Composable @ReadOnlyComposable get() = current.hairline
    val bubbleOut: Color @Composable @ReadOnlyComposable get() = current.bubbleOut
    val bubbleOutInk: Color @Composable @ReadOnlyComposable get() = current.bubbleOutInk
    val bubbleIn: Color @Composable @ReadOnlyComposable get() = current.bubbleIn
    val bubbleInInk: Color @Composable @ReadOnlyComposable get() = current.bubbleInInk
    val online: Color @Composable @ReadOnlyComposable get() = current.online
    val danger: Color @Composable @ReadOnlyComposable get() = current.danger
    val onPrimary: Color @Composable @ReadOnlyComposable get() = current.onPrimary

    val sparkBrush: Brush @Composable @ReadOnlyComposable get() = current.sparkBrush
    val navy: Color @Composable @ReadOnlyComposable get() = current.navy
    val avatarPalette: List<Color> @Composable @ReadOnlyComposable get() = current.avatarPalette

    @Composable @ReadOnlyComposable
    fun accent(seed: String): Color = current.accent(seed)
}
