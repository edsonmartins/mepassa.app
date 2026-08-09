//
//  ZapTheme.swift
//  ZapLivre / ZapLivre
//
//  Design system central. A identidade: navy profundo + amarelo/âmbar
//  energético + laranja pontual (documentos/zaplivre-paleta-cores.md). Diferencia
//  o produto de mensageiros verdes; o gradiente de marca (amarelo→âmbar→laranja)
//  é usado com restrição — só onde o app quer chamar a ação. Tudo o mais é quieto.
//

import SwiftUI
import UIKit

// MARK: - Hex helpers

extension UIColor {
    convenience init(hex: UInt) {
        self.init(
            red: CGFloat((hex >> 16) & 0xFF) / 255.0,
            green: CGFloat((hex >> 8) & 0xFF) / 255.0,
            blue: CGFloat(hex & 0xFF) / 255.0,
            alpha: 1.0
        )
    }
}

extension Color {
    /// Cor adaptativa: um valor para light, outro para dark. Dá dark mode
    /// automático sem depender de Asset Catalog.
    init(light: UInt, dark: UInt) {
        self.init(UIColor { trait in
            trait.userInterfaceStyle == .dark ? UIColor(hex: dark) : UIColor(hex: light)
        })
    }
}

// MARK: - Paleta ZapLivre

enum ZapColor {
    /// Âmbar de marca — ações principais, badges, links.
    static let primary = Color(light: 0xFFB000, dark: 0xFFAA00)
    /// Amarelo da marca — destaque, indicadores ativos.
    static let spark = Color(light: 0xFFD400, dark: 0xFFD400)
    /// Texto principal.
    static let ink = Color(light: 0x0F172A, dark: 0xF8FAFC)
    /// Texto secundário, ícones neutros, timestamps.
    static let slate = Color(light: 0x64748B, dark: 0x94A3B8)

    /// Fundo das telas de lista (Conversas, Grupos, Settings).
    static let canvas = Color(light: 0xF7F9FC, dark: 0x03152E)
    /// Fundo do chat (atrás das bolhas).
    static let chatCanvas = Color(light: 0xEEF2F7, dark: 0x03152E)
    /// Cartões / superfícies elevadas / bolha recebida.
    static let surface = Color(light: 0xFFFFFF, dark: 0x0B2A50)
    /// Divisórias / hairlines.
    static let hairline = Color(light: 0xE2E8F0, dark: 0x193B61)

    /// Bolha própria (enviada) — âmbar claro (light) / âmbar escurecido (dark).
    static let bubbleOut = Color(light: 0xFFF3C4, dark: 0x3B3214)
    static let bubbleOutInk = Color(light: 0x0F172A, dark: 0xF8FAFC)
    /// Bolha recebida.
    static let bubbleIn = Color(light: 0xFFFFFF, dark: 0x102B4D)
    static let bubbleInInk = Color(light: 0x0F172A, dark: 0xF8FAFC)

    /// Presença online (verde é convenção universal, não exclusiva de terceiros).
    static let online = Color(light: 0x22C55E, dark: 0x22C55E)
    /// Destrutivo / erro.
    static let danger = Color(light: 0xEF4444, dark: 0xEF4444)

    /// Navy profundo — base estrutural (headers, splash, superfícies).
    static let navy = Color(light: 0x061C3A, dark: 0x061C3A)
    /// Texto sobre ação âmbar/amarela — navy de marca (contraste, não branco).
    static let onPrimary = Color(light: 0x061C3A, dark: 0x061C3A)

    /// Paleta de avatares sem foto — cor derivada do id, para dar vida à lista.
    static let avatarPalette: [Color] = [
        Color(hex6: 0x0B2A50), Color(hex6: 0x7C3AED), Color(hex6: 0x00875A),
        Color(hex6: 0xE8618C), Color(hex6: 0xF2884B), Color(hex6: 0xB45309),
        Color(hex6: 0x0EA5E9), Color(hex6: 0xC026D3)
    ]

    /// Cor estável derivada de um id — mesma lógica dos avatares. Usada para
    /// avatar sem foto e nome de autor em grupos.
    static func accent(for seed: String) -> Color {
        var hash = 5381
        for byte in seed.utf8 { hash = ((hash << 5) &+ hash) &+ Int(byte) }
        return avatarPalette[abs(hash) % avatarPalette.count]
    }

    /// Gradiente de marca: amarelo → âmbar → laranja. Usar com restrição.
    static let sparkGradient = LinearGradient(
        colors: [Color(hex6: 0xFFD400), Color(hex6: 0xFFAA00), Color(hex6: 0xFF7900)],
        startPoint: .topLeading,
        endPoint: .bottomTrailing
    )
}

private extension Color {
    init(hex6: UInt) {
        self.init(
            red: Double((hex6 >> 16) & 0xFF) / 255.0,
            green: Double((hex6 >> 8) & 0xFF) / 255.0,
            blue: Double(hex6 & 0xFF) / 255.0
        )
    }
}

// MARK: - Tipografia

/// Escala tipográfica. Títulos e branding em SF Rounded (amigável, "livre");
/// corpo e utilitários na SF padrão. Substitui os `.font(.system(size:))` soltos.
enum ZapFont {
    static let brand = Font.system(size: 26, weight: .heavy, design: .rounded)
    static let title = Font.system(size: 20, weight: .bold, design: .rounded)
    static let rowName = Font.system(size: 17, weight: .semibold, design: .rounded)
    static let body = Font.system(size: 16, weight: .regular)
    static let preview = Font.system(size: 15, weight: .regular)
    static let caption = Font.system(size: 12, weight: .regular)
    static let badge = Font.system(size: 12, weight: .bold, design: .rounded)
}

// MARK: - Métricas

enum ZapMetric {
    static let bubbleRadius: CGFloat = 18
    static let cardRadius: CGFloat = 16
    static let buttonRadius: CGFloat = 14
    static let avatar: CGFloat = 52
    static let avatarSmall: CGFloat = 38

    static let gutter: CGFloat = 16
    static let rowGap: CGFloat = 12
    static let tight: CGFloat = 8
}
