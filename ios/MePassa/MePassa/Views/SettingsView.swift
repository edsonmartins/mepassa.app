//
//  SettingsView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

struct SettingsView: View {
    @Environment(\.dismiss) var dismiss
    @EnvironmentObject var appState: AppState
    @State private var showingQRCode = false
    @State private var showingLogoutAlert = false

    var body: some View {
        NavigationView {
            List {
                // Profile section
                Section {
                    HStack {
                        Circle()
                            .fill(Color.blue.gradient)
                            .frame(width: 60, height: 60)
                            .overlay(
                                Image(systemName: "person.fill")
                                    .font(.title)
                                    .foregroundColor(.white)
                            )

                        VStack(alignment: .leading, spacing: 4) {
                            Text(appState.currentUser?.username ?? "Usuário MePassa")
                                .font(.headline)

                            Text(appState.currentUser?.peerId.prefix(16) ?? "")
                                .font(.caption)
                                .foregroundColor(.secondary)
                                .monospaced()
                        }
                    }
                    .padding(.vertical, 8)

                    Button(action: { showingQRCode = true }) {
                        HStack {
                            Image(systemName: "qrcode")
                            Text("Mostrar meu QR Code")
                        }
                    }
                } header: {
                    Text("Perfil")
                }

                // Account section
                Section {
                    NavigationLink(destination: Text("Dispositivos vinculados")) {
                        Label("Dispositivos vinculados", systemImage: "iphone.and.ipad")
                    }

                    NavigationLink(destination: Text("Backup")) {
                        Label("Backup e restauração", systemImage: "arrow.clockwise.icloud")
                    }
                } header: {
                    Text("Conta")
                }

                // Privacy section
                Section {
                    NavigationLink(destination: Text("Privacidade")) {
                        Label("Privacidade", systemImage: "lock.shield")
                    }

                    NavigationLink(destination: Text("Dados e armazenamento")) {
                        Label("Dados e armazenamento", systemImage: "externaldrive")
                    }
                } header: {
                    Text("Configurações")
                }

                // About section
                Section {
                    HStack {
                        Text("Versão")
                        Spacer()
                        Text("1.0.0")
                            .foregroundColor(.secondary)
                    }

                    NavigationLink(destination: Text("Sobre o MePassa")) {
                        Text("Sobre o MePassa")
                    }

                    NavigationLink(destination: Text("Termos e privacidade")) {
                        Text("Termos e privacidade")
                    }
                } header: {
                    Text("Sobre")
                }

                // Logout
                Section {
                    Button(role: .destructive, action: { showingLogoutAlert = true }) {
                        HStack {
                            Spacer()
                            Text("Sair")
                            Spacer()
                        }
                    }
                }
            }
            .navigationTitle("Configurações")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Fechar") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showingQRCode) {
                MyQRCodeView()
            }
            .alert("Sair do MePassa?", isPresented: $showingLogoutAlert) {
                Button("Cancelar", role: .cancel) { }
                Button("Sair", role: .destructive) {
                    appState.logout()
                    dismiss()
                }
            } message: {
                Text("Você precisará importar sua identidade novamente para fazer login.")
            }
        }
    }
}

#Preview {
    SettingsView()
        .environmentObject(AppState())
}
