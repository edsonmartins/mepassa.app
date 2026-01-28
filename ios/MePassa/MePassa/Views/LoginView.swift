//
//  LoginView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

struct LoginView: View {
    @EnvironmentObject var appState: AppState
    @State private var peerId: String = ""
    @State private var isGeneratingId = false
    @State private var showError = false
    @State private var errorMessage = ""

    var body: some View {
        NavigationView {
            VStack(spacing: 30) {
                // Logo and title
                VStack(spacing: 16) {
                    Image(systemName: "lock.shield.fill")
                        .font(.system(size: 80))
                        .foregroundColor(.blue)

                    Text("MePassa")
                        .font(.largeTitle)
                        .fontWeight(.bold)

                    Text("Privacidade total. Sem servidores centrais.")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal)
                }
                .padding(.top, 60)

                Spacer()

                // Login options
                VStack(spacing: 20) {
                    // Generate new identity
                    Button(action: generateNewIdentity) {
                        HStack {
                            Image(systemName: "person.badge.plus")
                            Text("Criar nova identidade")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .disabled(isGeneratingId)

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
                    .padding(.horizontal)

                    // Import existing identity
                    Button(action: {
                        // TODO: Show QR scanner or peer ID input
                        showError = true
                        errorMessage = "Importação de identidade em desenvolvimento"
                    }) {
                        HStack {
                            Image(systemName: "qrcode.viewfinder")
                            Text("Importar identidade existente")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .foregroundColor(.primary)
                        .cornerRadius(12)
                    }
                }
                .padding(.horizontal, 30)

                Spacer()

                // Info text
                Text("Sua identidade é criptograficamente segura e não está vinculada a telefone ou email")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 40)
                    .padding(.bottom, 40)
            }
            .navigationTitle("")
            .navigationBarHidden(true)
            .alert("Erro", isPresented: $showError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(errorMessage)
            }
        }
    }

    private func generateNewIdentity() {
        isGeneratingId = true

        Task {
            do {
                if !MePassaCore.shared.isInitialized {
                    try await MePassaCore.shared.initialize()
                    try await MePassaCore.shared.startListening()
                }

                if let realPeerId = MePassaCore.shared.localPeerId, !realPeerId.isEmpty {
                    await MainActor.run {
                        appState.login(peerId: realPeerId)
                        isGeneratingId = false
                    }
                } else {
                    await MainActor.run {
                        isGeneratingId = false
                        showError = true
                        errorMessage = "Não foi possível obter o Peer ID"
                    }
                }
            } catch {
                await MainActor.run {
                    isGeneratingId = false
                    showError = true
                    errorMessage = "Falha ao inicializar identidade: \(error.localizedDescription)"
                }
            }
        }
    }
}

#Preview {
    LoginView()
        .environmentObject(AppState())
}
