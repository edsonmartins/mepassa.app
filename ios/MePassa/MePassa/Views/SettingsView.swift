//
//  SettingsView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

/// SettingsView - App settings screen
struct SettingsView: View {
    @State private var notificationsEnabled = true
    @State private var soundEnabled = true
    @State private var vibrationEnabled = true
    @State private var readReceiptsEnabled = true
    @State private var lastSeenEnabled = true
    @State private var showLogoutAlert = false

    var body: some View {
        Form {
            // Notifications section
            Section("Notificações") {
                Toggle("Ativar notificações", isOn: $notificationsEnabled)

                Toggle("Som", isOn: $soundEnabled)
                    .disabled(!notificationsEnabled)

                Toggle("Vibração", isOn: $vibrationEnabled)
                    .disabled(!notificationsEnabled)
            }

            // Privacy section
            Section("Privacidade") {
                Toggle("Confirmações de leitura", isOn: $readReceiptsEnabled)

                Toggle("Última visualização", isOn: $lastSeenEnabled)
            }

            // Storage section
            Section("Armazenamento") {
                HStack {
                    Text("Armazenamento usado")
                    Spacer()
                    Text("0 MB")
                        .foregroundColor(.secondary)
                }

                Button("Limpar cache de imagens") {
                    // TODO: Implement clear image cache
                }

                Button("Limpar cache de vídeos") {
                    // TODO: Implement clear video cache
                }
            }

            // About section
            Section("Sobre") {
                HStack {
                    Text("Versão")
                    Spacer()
                    Text("1.0.0 (Beta)")
                        .foregroundColor(.secondary)
                }

                Button("Licenças open source") {
                    // TODO: Show licenses
                }

                Button("Termos de uso") {
                    // TODO: Show terms
                }

                Button("Política de privacidade") {
                    // TODO: Show privacy policy
                }
            }

            // Logout section
            Section {
                Button("Sair", role: .destructive) {
                    showLogoutAlert = true
                }
            }
        }
        .navigationTitle("Configurações")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Sair", isPresented: $showLogoutAlert) {
            Button("Cancelar", role: .cancel) { }
            Button("Sair", role: .destructive) {
                // TODO: Implement logout
            }
        } message: {
            Text("Tem certeza que deseja sair?")
        }
    }
}

#Preview {
    NavigationView {
        SettingsView()
    }
}
