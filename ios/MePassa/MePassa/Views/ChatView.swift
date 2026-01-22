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

    // Reactions state
    @State private var messageReactions: [String: [ReactionCount]] = [:]
    @State private var showReactionPicker = false
    @State private var reactionPickerMessageId: String?

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
                                VStack(alignment: message.isOutgoing ? .trailing : .leading, spacing: 4) {
                                    MessageBubble(message: message)
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

                                    // Reaction bar
                                    if let reactions = messageReactions[message.id], !reactions.isEmpty {
                                        ReactionBar(
                                            reactions: reactions,
                                            onReactionTap: { emoji in
                                                handleReactionTap(messageId: message.id, emoji: emoji)
                                            },
                                            onAddReactionTap: {
                                                reactionPickerMessageId = message.id
                                                showReactionPicker = true
                                            }
                                        )
                                    }
                                }
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

                // Document picker button
                DocumentPickerButton(isEnabled: true) { fileURL in
                    Task {
                        do {
                            // Read file data
                            let fileData = try Data(contentsOf: fileURL)

                            // Get file info
                            let fileName = fileURL.lastPathComponent
                            let mimeType = fileURL.mimeType() ?? "application/octet-stream"

                            // Send via FFI
                            let messageId = try await MePassaCore.shared.sendDocumentMessage(
                                to: conversation.peerId,
                                fileData: fileData,
                                fileName: fileName,
                                mimeType: mimeType
                            )

                            print("✅ Document sent: \(messageId)")

                            // Reload messages
                            loadMessages()
                        } catch {
                            print("❌ Error sending document: \(error)")
                        }
                    }
                }

                // Video picker button
                VideoPickerButton(isEnabled: true) { videoInfo in
                    Task {
                        do {
                            // Read video file data
                            let videoData = try Data(contentsOf: videoInfo.url)

                            // Send video message
                            let messageId = try await MePassaCore.shared.sendVideoMessage(
                                toPeerId: conversation.peerId,
                                videoData: videoData,
                                fileName: videoInfo.fileName,
                                width: Int32(videoInfo.width),
                                height: Int32(videoInfo.height),
                                durationSeconds: Int32(videoInfo.durationSeconds),
                                thumbnailData: videoInfo.thumbnailData
                            )

                            print("✅ Video sent: \(messageId)")

                            // Reload messages
                            loadMessages()
                        } catch {
                            print("❌ Error sending video: \(error)")
                        }
                    }
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
        .sheet(isPresented: $showReactionPicker) {
            if let messageId = reactionPickerMessageId {
                ReactionPicker { emoji in
                    handleReactionTap(messageId: messageId, emoji: emoji)
                }
            }
        }
        .onChange(of: messages) { _ in
            loadReactions()
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

    private func loadReactions() {
        Task {
            var reactionsMap: [String: [ReactionCount]] = [:]

            for message in messages {
                do {
                    let reactions = try MePassaCore.shared.getMessageReactions(messageId: message.id)

                    // Aggregate by emoji
                    let grouped = Dictionary(grouping: reactions, by: { $0.emoji })
                    let reactionCounts = grouped.map { emoji, reactionList in
                        ReactionCount(
                            emoji: emoji,
                            count: reactionList.count,
                            hasReacted: reactionList.contains { $0.peerId == appState.currentUser?.peerId }
                        )
                    }.sorted { $0.count > $1.count }

                    reactionsMap[message.id] = reactionCounts
                } catch {
                    print("❌ Error loading reactions for message \(message.id): \(error)")
                }
            }

            messageReactions = reactionsMap
        }
    }

    private func handleReactionTap(messageId: String, emoji: String) {
        Task {
            do {
                let currentReactions = messageReactions[messageId] ?? []
                let hasReacted = currentReactions.first(where: { $0.emoji == emoji })?.hasReacted ?? false

                if hasReacted {
                    // Remove reaction
                    try MePassaCore.shared.removeReaction(messageId: messageId, emoji: emoji)
                } else {
                    // Add reaction
                    try MePassaCore.shared.addReaction(messageId: messageId, emoji: emoji)
                }

                // Reload reactions for this message
                let reactions = try MePassaCore.shared.getMessageReactions(messageId: messageId)
                let grouped = Dictionary(grouping: reactions, by: { $0.emoji })
                let reactionCounts = grouped.map { emoji, reactionList in
                    ReactionCount(
                        emoji: emoji,
                        count: reactionList.count,
                        hasReacted: reactionList.contains { $0.peerId == appState.currentUser?.peerId }
                    )
                }.sorted { $0.count > $1.count }

                messageReactions[messageId] = reactionCounts
            } catch {
                print("❌ Error toggling reaction: \(error)")
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

// MARK: - URL Extension

extension URL {
    /// Get MIME type from file URL
    func mimeType() -> String? {
        guard let uti = try? self.resourceValues(forKeys: [.typeIdentifierKey]).typeIdentifier else {
            return nil
        }

        if #available(iOS 14.0, *) {
            guard let utType = UTType(uti) else { return nil }
            return utType.preferredMIMEType
        } else {
            // Fallback for iOS 13
            return nil
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
