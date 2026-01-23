//
//  ChatView.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI
import UniformTypeIdentifiers

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

    // Media gallery state
    @State private var showMediaGallery = false

    // Search state
    @State private var showSearch = false

    private var messagesList: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(spacing: 12) {
                    if messages.isEmpty {
                        emptyState
                    } else {
                        ForEach(messages) { message in
                            messageRow(message)
                        }
                    }
                }
                .padding()
            }
        }
    }

    private var emptyState: some View {
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
    }

    private func messageRow(_ message: Message) -> some View {
        VStack(alignment: message.isOutgoing ? .trailing : .leading, spacing: 4) {
            MessageBubble(message: message)
                .transition(.asymmetric(
                    insertion: .move(edge: .bottom).combined(with: .opacity),
                    removal: .opacity
                ))
                .animation(.easeOut(duration: 0.3), value: messages.count)
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

    var body: some View {
        VStack(spacing: 0) {
            messagesList
            Divider()
            imagePreviewSection
            messageInputBar
        }
        .sheet(isPresented: $showingImagePicker) {
            ImagePicker(selectedImages: $mediaPickerVM.selectedImages)
        }
        .sheet(isPresented: $showMediaGallery) {
            MediaGalleryView(conversationId: conversation.id, peerName: conversation.displayName)
        }
        .sheet(isPresented: $showSearch) {
            MessageSearchView(
                conversationId: conversation.id,
                peerName: conversation.displayName,
                onMessageTap: { message in
                    // Message tap handled - search view will dismiss
                }
            )
        }
        .sheet(isPresented: $showReactionPicker) {
            if let messageId = reactionPickerMessageId {
                ReactionPicker { emoji in
                    addReaction(messageId: messageId, emoji: emoji)
                }
            }
        }
        .alert("Excluir Mensagem", isPresented: $showDeleteAlert) {
            Button("Cancelar", role: .cancel) {}
            Button("Excluir", role: .destructive) {
                if let message = selectedMessage {
                    deleteMessage(message)
                }
            }
        } message: {
            Text("Tem certeza que deseja excluir esta mensagem?")
        }
        .alert("Encaminhar Mensagem", isPresented: $showForwardAlert) {
            Button("Cancelar", role: .cancel) {}
            Button("OK") {
                // Forward functionality will be implemented when peer selection UI is added
            }
        } message: {
            Text("Selecione o destinatário")
        }
        .navigationTitle(conversation.displayName)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItemGroup(placement: .navigationBarTrailing) {
                Button(action: { showMediaGallery = true }) {
                    Image(systemName: "photo.on.rectangle")
                }
                Button(action: { showSearch = true }) {
                    Image(systemName: "magnifyingglass")
                }
                Button(action: startVoiceCall) {
                    Image(systemName: "phone")
                }
                Button(action: startVideoCall) {
                    Image(systemName: "video")
                }
            }
        }
        .onAppear {
            loadMessages()
            loadReactions()
        }
    }

    private var imagePreviewSection: some View {
        Group {
            if !mediaPickerVM.selectedImages.isEmpty {
                SelectedImagesPreview(
                    selectedImages: mediaPickerVM.selectedImages,
                    onRemoveImage: { index in
                        mediaPickerVM.removeImage(at: index)
                    },
                    onSendImages: {
                        mediaPickerVM.uploadImages(to: conversation.peerId, quality: 0.85)
                    }
                )
            }
        }
    }

    private var messageInputBar: some View {
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
                            HapticFeedback.light()  // Haptic feedback on send

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
                            HapticFeedback.light()  // Haptic feedback on send

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

    private func sendMessage() {
        guard !messageText.isEmpty else { return }

        Task {
            do {
                let messageId = try await MePassaCore.shared.sendMessage(
                    to: conversation.peerId,
                    content: messageText
                )
                print("✅ Message sent: \(messageId)")

                // Clear input and reload
                await MainActor.run {
                    messageText = ""
                }
                loadMessages()
            } catch {
                print("❌ Error sending message: \(error)")
            }
        }
    }

    private func loadMessages() {
        Task {
            do {
                let ffiMessages = try await MePassaCore.shared.getConversationMessages(
                    peerId: conversation.peerId,
                    limit: 100,
                    offset: 0
                )

                let localPeerId = MePassaCore.shared.localPeerId ?? ""

                await MainActor.run {
                    messages = ffiMessages.map { ffiMsg in
                        Message(
                            id: ffiMsg.id,
                            content: ffiMsg.content ?? "",
                            senderId: ffiMsg.senderPeerId,
                            timestamp: ffiMsg.createdAt,
                            isOutgoing: ffiMsg.senderPeerId == localPeerId,
                            status: ffiMsg.status,
                            ffiMessage: ffiMsg
                        )
                    }
                }
            } catch {
                print("❌ Error loading messages: \(error)")
            }
        }
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
        Task {
            do {
                try await MePassaCore.shared.deleteMessage(messageId: message.id)
                print("✅ Message deleted: \(message.id)")
                // Reload messages
                loadMessages()
            } catch {
                print("❌ Error deleting message: \(error)")
            }
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
                    let reactions = try await MePassaCore.shared.getMessageReactions(messageId: message.id)

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
                    try await MePassaCore.shared.removeReaction(messageId: messageId, emoji: emoji)
                } else {
                    // Add reaction
                    try await MePassaCore.shared.addReaction(messageId: messageId, emoji: emoji)
                    HapticFeedback.medium()  // Haptic feedback on reaction
                }

                // Reload reactions for this message
                let reactions = try await MePassaCore.shared.getMessageReactions(messageId: messageId)
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

    private func addReaction(messageId: String, emoji: String) {
        Task {
            do {
                try await MePassaCore.shared.addReaction(messageId: messageId, emoji: emoji)
                HapticFeedback.medium()

                // Reload reactions for this message
                let reactions = try await MePassaCore.shared.getMessageReactions(messageId: messageId)
                let grouped = Dictionary(grouping: reactions, by: { $0.emoji })
                let reactionCounts = grouped.map { emoji, reactionList in
                    ReactionCount(
                        emoji: emoji,
                        count: reactionList.count,
                        hasReacted: reactionList.contains { $0.peerId == appState.currentUser?.peerId }
                    )
                }.sorted { $0.count > $1.count }

                await MainActor.run {
                    messageReactions[messageId] = reactionCounts
                }
            } catch {
                print("❌ Error adding reaction: \(error)")
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

                MessageStatusIndicator(
                    message: message.ffiMessage,
                    isOwnMessage: message.isOutgoing
                )
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
    let ffiMessage: FfiMessageWrapper?  // Keep reference to original FfiMessage

    init(id: String, content: String, senderId: String, timestamp: Date, isOutgoing: Bool, status: MessageStatus, ffiMessage: FfiMessageWrapper? = nil) {
        self.id = id
        self.content = content
        self.senderId = senderId
        self.timestamp = timestamp
        self.isOutgoing = isOutgoing
        self.status = status
        self.ffiMessage = ffiMessage
    }
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
