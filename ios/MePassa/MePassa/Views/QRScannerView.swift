//
//  QRScannerView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI
import AVFoundation

struct QRScannerView: View {
    @Environment(\.dismiss) var dismiss
    let onScan: (String) -> Void

    var body: some View {
        NavigationView {
            ZStack {
                // TODO: Implement camera-based QR scanner using AVFoundation
                // For now, show placeholder
                VStack(spacing: 20) {
                    Image(systemName: "qrcode.viewfinder")
                        .font(.system(size: 100))
                        .foregroundColor(.secondary)

                    Text("Scanner QR em desenvolvimento")
                        .font(.headline)
                        .foregroundColor(.secondary)

                    Text("Use a opção de inserir Peer ID manualmente")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal, 40)
                }
            }
            .navigationTitle("Escanear QR Code")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Fechar") {
                        dismiss()
                    }
                }
            }
        }
    }
}

#Preview {
    QRScannerView { peerId in
        print("Scanned: \(peerId)")
    }
}
