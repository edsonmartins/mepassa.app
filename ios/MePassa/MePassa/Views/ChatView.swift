//
//  ChatView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

struct ChatView: View {
    let conversation: Conversation
    @EnvironmentObject var appState: AppState
    @State private var messageText = ""
    @State private var messages: [Message] = []

    // Image picker state
    @StateObject private var mediaPickerVM = MediaPickerViewModel()
    @State private var showingImagePicker = false

    // Voice recorder state
    @StateObject private var voiceRecorderVM = VoiceRecorderViewModel()

    // Message actions state
    @State private var selectedMessage: Message?
    @State private var showDeleteAlert = false
    @State private var showForwardAlert = false

    var body: some View {
        VStack(spacing: 0) {
            // Messages list
            ScrollViewReader { proxy in
                ScrollView {
                    LazyVStack(spacing: 12) {
                        if messages.isEmpty {
                            // Empty state
                            VStack(spacing: 12) {
                                Image(systemName: "lock.fill")
                                    .font(.system(size: 50))
                                    .foregroundColor(.secondary)

                                Text("Conversa criptografada de ponta a ponta")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                                    .multilineTextAlignment(.center)
                            }
                            .padding(.top, 100)
                        } else {
                            ForEach(messages) { message in
                                MessageBubble(message: message)
                                    .id(message.id)
                                    .contextMenu {
                                        Button(action: {
                                            selectedMessage = message
                                            showForwardAlert = true
                                        }) {
                                            Label("Encaminhar", systemImage: "arrowshape.turn.up.forward")
                                        }

                                        Button(role: .destructive, action: {
                                            selectedMessage = message
                                            showDeleteAlert = true
                                        }) {
                                            Label("Excluir", systemImage: "trash")
                                        }
                                    }
                            }
                        }
                    }
                    .padding()
                }
            }

            Divider()

            // Selected images preview
            if !mediaPickerVM.selectedImages.isEmpty {
                SelectedImagesPreview(
                    selectedImages: mediaPickerVM.selectedImages,
                    onRemoveImage: { index in
                        mediaPickerVM.removeImage(at: index)
                    },
                    onSendImages: {
                        // Send images via FFI with compression
                        mediaPickerVM.uploadImages(to: conversation.peerId, quality: 0.85)
                    }
                )
            }

            // Message input
            HStack(spacing: 12) {
                // Image picker button
                Button(action: {
                    showingImagePicker = true
                }) {
                    Image(systemName: "photo.on.rectangle")
                        .font(.title2)
                        .foregroundColor(.blue)
                }

                // Text field
                TextField("Mensagem", text: $messageText)
                    .textFieldStyle(.plain)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(Color.secondary.opacity(0.1))
                    .cornerRadius(20)

                // Send or voice button
                if messageText.isEmpty {
                    VoiceRecordButton(
                        viewModel: voiceRecorderVM,
                        onVoiceMessageRecorded: { audioURL in
                            Task {
                                do {
                                    // Read audio file data
                                    let audioData = try Data(contentsOf: audioURL)

                                    // Get file name and estimate duration
                                    let fileName = audioURL.lastPathComponent
                                    let durationSeconds = Int32(voiceRecorderVM.recordingDuration)

                                    // Send via FFI
                                    let messageId = try await MePassaCore.shared.sendVoiceMessage(
                                        to: conversation.peerId,
                                        audioData: audioData,
                                        fileName: fileName,
                                        durationSeconds: durationSeconds
                                    )

                                    print("✅ Voice message sent: \(messageId)")

                                    // Reload messages
                                    loadMessages()
                                } catch {
                                    print("❌ Error sending voice message: \(error)")
                                }
                            }
                        }
                    )
                } else {
                    Button(action: sendMessage) {
                        Image(systemName: "arrow.up.circle.fill")
                            .font(.title2)
                            .foregroundColor(.blue)
                    }
                }
            }
            .padding(.horizontal)
            .padding(.vertical, 8)
        }
        .sheet(isPresented: $showingImagePicker) {
            ImagePicker(selectedImages: $mediaPickerVM.selectedImages, maxSelection: 10)
        }
        .navigationTitle(conversation.displayName)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                HStack(spacing: 16) {
                    Button(action: startVoiceCall) {
                        Image(systemName: "phone.fill")
                    }

                    Button(action: startVideoCall) {
                        Image(systemName: "video.fill")
                    }
                }
            }
        }
        .onAppear {
            loadMessages()
        }
        .alert("Excluir mensagem", isPresented: $showDeleteAlert, presenting: selectedMessage) { message in
            Button("Cancelar", role: .cancel) { }
            Button("Excluir", role: .destructive) {
                deleteMessage(message)
            }
        } message: { _ in
            Text("Tem certeza que deseja excluir esta mensagem?")
        }
        .alert("Encaminhar mensagem", isPresented: $showForwardAlert, presenting: selectedMessage) { message in
            Button("OK", role: .cancel) { }
        } message: { _ in
            Text("Funcionalidade de encaminhamento será implementada em breve.\n\nTODO: Adicionar seletor de conversas.")
        }
    }

    private func sendMessage() {
        guard !messageText.isEmpty else { return }

        // TODO: Send message via UniFFI
        let message = Message(
            id: UUID().uuidString,
            content: messageText,
            senderId: appState.currentUser?.peerId ?? "",
            timestamp: Date(),
            isOutgoing: true,
            status: .sent
        )

        messages.append(message)
        messageText = ""
    }

    private func loadMessages() {
        // TODO: Load messages from storage via UniFFI
        // For now, show empty state
    }

    private func startVoiceCall() {
        // TODO: Initiate VoIP call via CallManager
        print("📞 Starting voice call with \(conversation.peerId)")
    }

    private func startVideoCall() {
        // TODO: Initiate video call
        print("📹 Starting video call with \(conversation.peerId)")
    }

    private func deleteMessage(_ message: Message) {
        do {
            try MePassaCore.shared.deleteMessage(messageId: message.id)
            print("✅ Message deleted: \(message.id)")
            // Reload messages
            loadMessages()
        } catch {
            print("❌ Error deleting message: \(error)")
        }
    }

    private func forwardMessage(_ message: Message, to peerId: String) {
        Task {
            do {
                let newMessageId = try await MePassaCore.shared.forwardMessage(
                    messageId: message.id,
                    toPeerId: peerId
                )
                print("✅ Message forwarded: \(newMessageId)")
                // Reload messages
                loadMessages()
            } catch {
                print("❌ Error forwarding message: \(error)")
            }
        }
    }
}

struct MessageBubble: View {
    let message: Message

    var body: some View {
        HStack {
            if message.isOutgoing {
                Spacer()
            }

            VStack(alignment: message.isOutgoing ? .trailing : .leading, spacing: 4) {
                Text(message.content)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(message.isOutgoing ? Color.blue : Color.secondary.opacity(0.2))
                    .foregroundColor(message.isOutgoing ? .white : .primary)
                    .cornerRadius(16)

                HStack(spacing: 4) {
                    Text(message.timestamp.formatted(date: .omitted, time: .shortened))
                        .font(.caption2)
                        .foregroundColor(.secondary)

                    if message.isOutgoing {
                        Image(systemName: message.status.iconName)
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }
                }
            }

            if !message.isOutgoing {
                Spacer()
            }
        }
    }
}

// MARK: - Models

struct Message: Identifiable {
    let id: String
    let content: String
    let senderId: String
    let timestamp: Date
    let isOutgoing: Bool
    let status: MessageStatus
}

// MessageStatus enum is provided by the Rust FFI bindings (mepassa.swift)
// Extension to add UI helpers
extension MessageStatus {
    var iconName: String {
        switch self {
        case .pending: return "clock"
        case .sent: return "checkmark"
        case .delivered: return "checkmark.circle"
        case .read: return "checkmark.circle.fill"
        case .failed: return "exclamationmark.circle"
        }
    }
}

#Preview {
    NavigationView {
        ChatView(conversation: Conversation(
            id: "1",
            peerId: "12D3KooW...",
            displayName: "Alice",
            lastMessage: "Olá!",
            unreadCount: 0
        ))
        .environmentObject(AppState())
    }
}
