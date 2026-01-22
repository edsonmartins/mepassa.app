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
                        mediaPickerVM.uploadImages(to: conversation.id)
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
                    Button(action: {
                        // TODO: Start voice recording
                    }) {
                        Image(systemName: "mic.fill")
                            .font(.title2)
                            .foregroundColor(.blue)
                    }
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
