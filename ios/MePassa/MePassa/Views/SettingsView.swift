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
    @State private var showExportSheet = false
    @State private var exportData = ""
    @State private var showExportError = false
    @State private var exportErrorMessage = ""

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

            // Identity section
            Section("Identidade") {
                Button("Exportar backup da identidade") {
                    Task {
                        do {
                            exportData = try await MePassaCore.shared.exportIdentity()
                            showExportSheet = true
                        } catch {
                            exportErrorMessage = error.localizedDescription
                            showExportError = true
                        }
                    }
                }

                Button("Exportar prekeys") {
                    Task {
                        do {
                            exportData = try await MePassaCore.shared.exportPrekeyBundle()
                            showExportSheet = true
                        } catch {
                            exportErrorMessage = error.localizedDescription
                            showExportError = true
                        }
                    }
                }
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
        .alert("Erro", isPresented: $showExportError) {
            Button("OK", role: .cancel) { }
        } message: {
            Text(exportErrorMessage)
        }
        .sheet(isPresented: $showExportSheet) {
            NavigationView {
                VStack(spacing: 16) {
                    Text("Backup da identidade (Base64)")
                        .font(.headline)

                    TextEditor(text: $exportData)
                        .font(.system(.body, design: .monospaced))
                        .frame(minHeight: 220)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(Color.secondary.opacity(0.4))
                        )
                        .padding(.horizontal)

                    Button(action: {
                        UIPasteboard.general.string = exportData
                    }) {
                        Text("Copiar")
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.blue)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .padding(.horizontal)

                    Spacer()
                }
                .padding(.top, 12)
                .navigationTitle("Exportar Identidade")
                .navigationBarTitleDisplayMode(.inline)
                .toolbar {
                    ToolbarItem(placement: .cancellationAction) {
                        Button("Fechar") { showExportSheet = false }
                    }
                }
            }
        }
    }
}

#Preview {
    NavigationView {
        SettingsView()
    }
}
