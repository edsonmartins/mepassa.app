//
//  ProfileView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

/// ProfileView - User profile screen
struct ProfileView: View {
    @Environment(\.dismiss) var dismiss

    @State private var userName = "Usuário MePassa"
    @State private var isEditingName = false
    @State private var localPeerId = ""
    @State private var showCopiedAlert = false
    @State private var showExportSheet = false
    @State private var exportData = ""
    @State private var showExportError = false
    @State private var exportErrorMessage = ""

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    Spacer().frame(height: 16)

                    // Avatar
                    ZStack(alignment: .bottomTrailing) {
                        Circle()
                            .fill(Color.blue.opacity(0.2))
                            .frame(width: 120, height: 120)
                            .overlay(
                                Image(systemName: "person.fill")
                                    .font(.system(size: 50))
                                    .foregroundColor(.blue)
                            )

                        // Edit button
                        Button(action: {
                            // TODO: Open avatar picker
                        }) {
                            Image(systemName: "pencil.circle.fill")
                                .font(.system(size: 36))
                                .foregroundColor(.blue)
                                .background(Color.white.clipShape(Circle()))
                        }
                    }

                    // Name
                    if isEditingName {
                        VStack(spacing: 12) {
                            TextField("Nome", text: $userName)
                                .textFieldStyle(.roundedBorder)
                                .multilineTextAlignment(.center)

                            HStack(spacing: 12) {
                                Button("Cancelar") {
                                    isEditingName = false
                                }
                                .buttonStyle(.bordered)

                                Button("Salvar") {
                                    // TODO: Save name
                                    isEditingName = false
                                }
                                .buttonStyle(.borderedProminent)
                            }
                        }
                        .padding(.horizontal)
                    } else {
                        VStack(spacing: 8) {
                            Text(userName)
                                .font(.title2)
                                .fontWeight(.bold)

                            Button("Editar nome") {
                                isEditingName = true
                            }
                            .font(.subheadline)
                        }
                    }

                    Divider()

                    // Peer ID section
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Seu ID MePassa")
                            .font(.headline)

                        HStack {
                            Text(String(localPeerId.prefix(32)) + "...")
                                .font(.system(.body, design: .monospaced))
                                .lineLimit(1)
                                .truncationMode(.tail)

                            Spacer()

                            Button(action: {
                                UIPasteboard.general.string = localPeerId
                                showCopiedAlert = true
                            }) {
                                Image(systemName: "doc.on.doc")
                                    .foregroundColor(.blue)
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(8)

                        Text("Compartilhe este ID para que outros possam te adicionar")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding(.horizontal)

                    Button(action: exportIdentity) {
                        HStack {
                            Image(systemName: "arrow.up.doc")
                            Text("Exportar identidade")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .padding(.horizontal)

                    Button(action: exportPrekeyBundle) {
                        HStack {
                            Image(systemName: "key.horizontal")
                            Text("Exportar prekeys")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .foregroundColor(.primary)
                        .cornerRadius(12)
                    }
                    .padding(.horizontal)

                    // QR Code placeholder
                    VStack(spacing: 8) {
                        RoundedRectangle(cornerRadius: 8)
                            .fill(Color.white)
                            .frame(width: 200, height: 200)
                            .overlay(
                                VStack {
                                    Text("QR CODE")
                                        .font(.caption)
                                    Text(String(localPeerId.prefix(8)) + "...")
                                        .font(.caption2)
                                }
                                .foregroundColor(.gray)
                            )
                            .shadow(radius: 2)

                        Text("Escaneie para conectar")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }

                    Spacer()
                }
                .padding()
            }
            .navigationTitle("Perfil")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Fechar") {
                        dismiss()
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    NavigationLink("Configurações") {
                        SettingsView()
                    }
                }
            }
        }
        .onAppear {
            loadPeerId()
        }
        .alert("ID copiado!", isPresented: $showCopiedAlert) {
            Button("OK", role: .cancel) { }
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

    private func loadPeerId() {
        localPeerId = MePassaCore.shared.localPeerId ?? ""
    }

    private func exportIdentity() {
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

    private func exportPrekeyBundle() {
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

#Preview {
    ProfileView()
}
