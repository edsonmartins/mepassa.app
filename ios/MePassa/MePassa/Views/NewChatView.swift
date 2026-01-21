//
//  NewChatView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

struct NewChatView: View {
    @Environment(\.dismiss) var dismiss
    @State private var peerId = ""
    @State private var showingQRScanner = false

    var body: some View {
        NavigationView {
            VStack(spacing: 30) {
                // QR Scanner option
                Button(action: { showingQRScanner = true }) {
                    VStack(spacing: 12) {
                        Image(systemName: "qrcode.viewfinder")
                            .font(.system(size: 60))
                            .foregroundColor(.blue)

                        Text("Escanear QR Code")
                            .font(.headline)
                    }
                    .frame(maxWidth: .infinity)
                    .padding(40)
                    .background(Color.secondary.opacity(0.1))
                    .cornerRadius(16)
                }
                .buttonStyle(.plain)

                // Or divider
                HStack {
                    Rectangle()
                        .frame(height: 1)
                        .foregroundColor(.secondary.opacity(0.3))
                    Text("ou")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .padding(.horizontal, 8)
                    Rectangle()
                        .frame(height: 1)
                        .foregroundColor(.secondary.opacity(0.3))
                }

                // Manual peer ID input
                VStack(alignment: .leading, spacing: 12) {
                    Text("Inserir Peer ID manualmente")
                        .font(.headline)

                    TextField("12D3KooW...", text: $peerId)
                        .textFieldStyle(.roundedBorder)
                        .autocapitalization(.none)
                        .autocorrectionDisabled()

                    Button(action: startChat) {
                        Text("Iniciar conversa")
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(peerId.isEmpty ? Color.secondary.opacity(0.3) : Color.blue)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .disabled(peerId.isEmpty)
                }

                Spacer()
            }
            .padding()
            .navigationTitle("Nova Conversa")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancelar") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showingQRScanner) {
                QRScannerView { scannedPeerId in
                    peerId = scannedPeerId
                    showingQRScanner = false
                }
            }
        }
    }

    private func startChat() {
        // TODO: Verify peer ID format and initiate connection via UniFFI
        print("📱 Starting chat with peer: \(peerId)")
        dismiss()
    }
}

#Preview {
    NewChatView()
}
