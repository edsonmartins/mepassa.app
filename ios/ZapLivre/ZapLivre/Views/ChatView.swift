//
//  ChatView.swift
//  ZapLivre
//
//  Created by ZapLivre Team
//  Copyright © 2026 ZapLivre. All rights reserved.
//

import SwiftUI
import UniformTypeIdentifiers
import CoreImage.CIFilterBuiltins

struct ChatView: View {
    let conversation: Conversation
    @EnvironmentObject var appState: AppState
    @EnvironmentObject var callManager: CallManager
    @State private var messageText = ""
    @State private var messages: [Message] = []
    @State private var oldestMessageCursor: (Int64, String)?
    @State private var isLoadingOlderMessages = false
    @State private var hasOlderMessages = true

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
    @State private var mediaIndex: [String: FfiMedia] = [:]
    @State private var showReactionPicker = false
    @State private var reactionPickerMessageId: String?

    // Media gallery state
    @State private var showMediaGallery = false
    @State private var showSecurityNumber = false
    @State private var activeVideoCallId: String?

    // Search state
    @State private var showSearch = false
    @State private var refreshTimer: Timer?
    @State private var messageObservers: [NSObjectProtocol] = []

    private var messagesList: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(spacing: 12) {
                    if messages.isEmpty {
                        emptyState
                    } else {
                        ForEach(messages) { message in
                            messageRow(message)
                                .onAppear {
                                    if message.id == messages.first?.id {
                                        loadOlderMessagesIfNeeded()
                                    }
                                }
                        }
                    }
                    Color.clear
                        .frame(height: 1)
                        .id("bottom")
                }
                .padding()
            }
            .onAppear {
                proxy.scrollTo("bottom", anchor: .bottom)
            }
            .onChange(of: messages.count) { _ in
                // Keep the initial history load stable; animating every row
                // from its compact placeholder makes cards visibly "grow".
                proxy.scrollTo("bottom", anchor: .bottom)
            }
        }
    }

    private var emptyState: some View {
        VStack(spacing: 10) {
            Image(systemName: "lock.fill")
                .font(.system(size: 13, weight: .semibold))
                .foregroundColor(ZapColor.slate)
            Text("As mensagens são protegidas com criptografia de ponta a ponta. Nem o ZapLivre pode lê-las.")
                .font(ZapFont.caption)
                .foregroundColor(ZapColor.slate)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 40)
        }
        .padding(.vertical, 10)
        .padding(.horizontal, 12)
        .background(
            RoundedRectangle(cornerRadius: 10, style: .continuous)
                .fill(ZapColor.primary.opacity(0.08))
        )
        .padding(.top, 60)
    }

    private func messageRow(_ message: Message) -> some View {
        VStack(alignment: message.isOutgoing ? .trailing : .leading, spacing: 4) {
            MessageBubble(message: message, media: mediaIndex[message.id])
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
                .background(ZapColor.chatCanvas)
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
        .sheet(isPresented: $showSecurityNumber) {
            SecurityNumberView(
                peerId: conversation.peerId,
                peerName: conversation.displayName
            )
        }
        .sheet(isPresented: $showReactionPicker) {
            if let messageId = reactionPickerMessageId {
                ReactionPicker { emoji in
                    addReaction(messageId: messageId, emoji: emoji)
                }
            }
        }
        .background(
            NavigationLink(
                destination: Group {
                    if let callId = activeVideoCallId {
                        VideoCallScreen(
                            callId: callId,
                            remotePeerId: conversation.peerId,
                            peerName: conversation.displayName,
                            onHangup: { activeVideoCallId = nil }
                        )
                    }
                },
                isActive: Binding(
                    get: { activeVideoCallId != nil },
                    set: { active in
                        if !active {
                            activeVideoCallId = nil
                        }
                    }
                )
            ) { EmptyView() }
            .hidden()
        )
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
                .accessibilityIdentifier("chat_media_gallery")
                Button(action: { showSearch = true }) {
                    Image(systemName: "magnifyingglass")
                }
                .accessibilityIdentifier("chat_search")
                Button(action: { showSecurityNumber = true }) {
                    Image(systemName: "checkmark.shield")
                }
                .accessibilityLabel("Verificar segurança")
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
            startAutoRefresh()
            Task { await connectIfPossible() }
        }
        .onDisappear {
            stopAutoRefresh()
        }
        .onReceive(mediaPickerVM.$uploadState) { state in
            if state == .success {
                loadMessages()
            }
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
        HStack(spacing: 4) {
                // Image picker button
                Button(action: {
                    showingImagePicker = true
                }) {
                    Image(systemName: "photo.on.rectangle")
                        .font(.system(size: 18))
                        .foregroundColor(ZapColor.slate)
                }
                .frame(width: 28, height: 32)

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
                            let messageId = try await ZapLivreCore.shared.sendDocumentMessage(
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
                .frame(width: 28, height: 32)

                // Video picker button
                VideoPickerButton(isEnabled: true) { videoInfo in
                    Task {
                        do {
                            // Read video file data
                            let videoData = try Data(contentsOf: videoInfo.url)

                            // Send video message
                            let messageId = try await ZapLivreCore.shared.sendVideoMessage(
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
                .frame(width: 28, height: 32)

                // Text field
                TextField("Mensagem", text: $messageText)
                    .accessibilityIdentifier("chat_input")
                    .textFieldStyle(.plain)
                    .font(ZapFont.body)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 7)
                    .background(ZapColor.surface)
                    .clipShape(RoundedRectangle(cornerRadius: 22, style: .continuous))
                    .overlay(
                        RoundedRectangle(cornerRadius: 22, style: .continuous)
                            .stroke(ZapColor.hairline, lineWidth: 1)
                    )

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
                                    let messageId = try await ZapLivreCore.shared.sendVoiceMessage(
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
                        Image(systemName: "arrow.up")
                            .font(.system(size: 18, weight: .bold))
                            .foregroundColor(.white)
                            .frame(width: 46, height: 46)
                            .background(ZapColor.sparkGradient)
                            .clipShape(Circle())
                    }
                    .accessibilityIdentifier("chat_send")
                }
            }
            .padding(.horizontal, 6)
            .padding(.vertical, 8)
            .background(
                ZapColor.canvas
                    .overlay(ZapColor.hairline.frame(height: 0.5), alignment: .top)
                    .ignoresSafeArea(edges: .bottom)
            )
    }

    private func sendMessage() {
        guard !messageText.isEmpty else { return }

        Task {
            do {
                await connectIfPossible()
                let messageId = try await ZapLivreCore.shared.sendMessage(
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
                let ffiMessages = try await ZapLivreCore.shared.getConversationMessages(
                    peerId: conversation.peerId,
                    limit: 50,
                    offset: 0
                )
                let mediaItems = try await ZapLivreCore.shared.getConversationMedia(
                    conversationId: conversation.id,
                    mediaType: nil,
                    // Media rows are metadata only; keep this aligned with
                    // the initial message window and load file bytes on demand.
                    limit: 50
                )

                let localPeerId = ZapLivreCore.shared.localPeerId ?? ""
                let ordered = ffiMessages.sorted { $0.createdAt < $1.createdAt }
                var displayMessages: [FfiMessageWrapper] = []
                for message in ordered {
                    let consumed = await ZapLivreCore.shared.consumeGroupSenderKeyMessage(message)
                    if !consumed {
                        displayMessages.append(message)
                    }
                }

                await MainActor.run {
                    if let oldest = displayMessages.first { self.oldestMessageCursor = (Int64(oldest.createdAt.timeIntervalSince1970), oldest.id) }
                    self.hasOlderMessages = ffiMessages.count == 50
                    self.mediaIndex = Dictionary(uniqueKeysWithValues: mediaItems.map { ($0.messageId, $0) })
                    messages = displayMessages.map { ffiMsg in
                        Message(
                            id: ffiMsg.id,
                            content: ffiMsg.content ?? "",
                            messageType: ffiMsg.messageType,
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

    private func loadOlderMessagesIfNeeded() {
        guard hasOlderMessages, !isLoadingOlderMessages, !messages.isEmpty else { return }
        isLoadingOlderMessages = true
        guard let cursor = oldestMessageCursor else { return }
        Task {
            defer { Task { @MainActor in isLoadingOlderMessages = false } }
            do {
                let older = try await ZapLivreCore.shared.getConversationMessagesBefore(
                    peerId: conversation.peerId,
                    limit: 50,
                    beforeCreatedAt: cursor.0,
                    beforeMessageId: cursor.1
                )
                let localPeerId = ZapLivreCore.shared.localPeerId ?? ""
                let display = older.reversed().map { ffiMsg in
                    Message(
                        id: ffiMsg.id,
                        content: ffiMsg.content ?? "",
                        messageType: ffiMsg.messageType,
                        senderId: ffiMsg.senderPeerId,
                        timestamp: ffiMsg.createdAt,
                        isOutgoing: ffiMsg.senderPeerId == localPeerId,
                        status: ffiMsg.status,
                        ffiMessage: ffiMsg
                    )
                }
                await MainActor.run {
                    messages.insert(contentsOf: display, at: 0)
                    if let first = display.first { oldestMessageCursor = (Int64(first.timestamp.timeIntervalSince1970), first.id) }
                    hasOlderMessages = older.count == 50
                }
            } catch {
                print("❌ Error loading older messages: \(error)")
            }
        }
    }

    private func connectIfPossible() async {
        if let addr = UserDefaults.standard.string(forKey: "zaplivre.multiaddr.\(conversation.peerId)") {
            do {
                print("🔗 Reconnecting to peer \(conversation.peerId) at \(addr)...")
                try await ZapLivreCore.shared.connectToPeer(peerId: conversation.peerId, multiaddr: addr)
                try await Task.sleep(nanoseconds: 300_000_000)
            } catch {
                print("⚠️ Reconnect failed: \(error)")
            }
        }
    }

    /// EVT-02: eventos do core substituem o polling de 2s; o timer é só
    /// um safety net lento
    private func startAutoRefresh() {
        let received = NotificationCenter.default.addObserver(
            forName: .zapLivreMessageReceived,
            object: nil,
            queue: .main
        ) { notification in
            let fromPeerId = notification.userInfo?["from_peer_id"] as? String
            if fromPeerId == conversation.peerId {
                loadMessages()
            }
        }
        let status = NotificationCenter.default.addObserver(
            forName: .zapLivreMessageStatusChanged,
            object: nil,
            queue: .main
        ) { _ in
            loadMessages()
        }
        messageObservers = [received, status]

        refreshTimer?.invalidate()
        refreshTimer = Timer.scheduledTimer(withTimeInterval: 30.0, repeats: true) { _ in
            loadMessages()
        }
    }

    private func stopAutoRefresh() {
        refreshTimer?.invalidate()
        refreshTimer = nil
        messageObservers.forEach { NotificationCenter.default.removeObserver($0) }
        messageObservers = []
    }

    private func startVoiceCall() {
        print("📞 Starting voice call with \(conversation.peerId)")
        callManager.startCall(
            to: conversation.peerId,
            displayName: String(conversation.peerId.prefix(16))
        )
    }

    private func startVideoCall() {
        Task {
            do {
                let callId = try await ZapLivreCore.shared.startCall(to: conversation.peerId)
                await MainActor.run {
                    activeVideoCallId = callId
                }
            } catch {
                print("❌ Failed to start video call: \(error)")
            }
        }
    }

    private func deleteMessage(_ message: Message) {
        Task {
            do {
                try await ZapLivreCore.shared.deleteMessage(messageId: message.id)
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
                let newMessageId = try await ZapLivreCore.shared.forwardMessage(
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
                    let reactions = try await ZapLivreCore.shared.getMessageReactions(messageId: message.id)

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
                    try await ZapLivreCore.shared.removeReaction(messageId: messageId, emoji: emoji)
                } else {
                    // Add reaction
                    try await ZapLivreCore.shared.addReaction(messageId: messageId, emoji: emoji)
                    HapticFeedback.medium()  // Haptic feedback on reaction
                }

                // Reload reactions for this message
                let reactions = try await ZapLivreCore.shared.getMessageReactions(messageId: messageId)
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
                try await ZapLivreCore.shared.addReaction(messageId: messageId, emoji: emoji)
                HapticFeedback.medium()

                // Reload reactions for this message
                let reactions = try await ZapLivreCore.shared.getMessageReactions(messageId: messageId)
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

/// Displays the authenticated identity fingerprint for a conversation.
/// The local verification marker is intentionally kept on-device; a future
/// transparency log can replace this marker without changing the UI contract.
private struct SecurityNumberView: View {
    let peerId: String
    let peerName: String
    @Environment(\.dismiss) private var dismiss
    @State private var fingerprint = ""
    @State private var storedFingerprint: String?
    @State private var errorMessage: String?
    @State private var isLoading = true
    @State private var transparencyVerified = false

    private var storageKey: String { "verified_identity_fingerprint_\(peerId)" }
    private var isVerified: Bool { storedFingerprint == fingerprint && !fingerprint.isEmpty }
    private var hasChanged: Bool {
        guard let storedFingerprint, !storedFingerprint.isEmpty else { return false }
        return !fingerprint.isEmpty && storedFingerprint != fingerprint
    }

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 20) {
                    Image(systemName: hasChanged ? "exclamationmark.shield.fill" : (isVerified ? "checkmark.shield.fill" : "shield.lefthalf.filled"))
                        .font(.system(size: 52))
                        .foregroundColor(hasChanged ? .orange : (isVerified ? .green : ZapColor.primary))
                        .padding(.top, 18)

                    Text("Número de segurança")
                        .font(.title2.weight(.semibold))
                    Text("Compare este código com o código exibido no dispositivo de \(peerName). Se forem iguais, a identidade autenticada é a mesma.")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal)

                    Group {
                        if isLoading {
                            ProgressView("Carregando identidade…")
                        } else if let errorMessage {
                            Label(errorMessage, systemImage: "exclamationmark.triangle")
                                .foregroundColor(.orange)
                                .multilineTextAlignment(.center)
                        } else {
                            Text(fingerprint)
                                .font(.system(.title3, design: .monospaced).weight(.medium))
                                .tracking(1.5)
                                .multilineTextAlignment(.center)
                                .padding(18)
                                .frame(maxWidth: .infinity)
                                .background(Color.secondary.opacity(0.12), in: RoundedRectangle(cornerRadius: 14))
                                .textSelection(.enabled)

                            Image(uiImage: makeSecurityQRCode())
                                .interpolation(.none)
                                .resizable()
                                .scaledToFit()
                                .frame(width: 190, height: 190)
                                .padding(12)
                                .background(Color.white, in: RoundedRectangle(cornerRadius: 12))
                                .accessibilityLabel("QR Code do número de segurança")

                            Text("QR Code para comparação de segurança")
                                .font(.caption)
                                .foregroundColor(.secondary)

                            Label(
                                transparencyVerified ? "Identidade incluída no log de transparência" : "Log de transparência indisponível",
                                systemImage: transparencyVerified ? "checkmark.seal.fill" : "questionmark.diamond"
                            )
                            .font(.footnote)
                            .foregroundColor(transparencyVerified ? .green : .secondary)

                            Button {
                                UIPasteboard.general.string = fingerprint
                            } label: {
                                Label("Copiar código", systemImage: "doc.on.doc")
                            }
                            .buttonStyle(.bordered)

                            if hasChanged {
                                Label("A identidade deste contato mudou. Confirme novamente antes de confiar.", systemImage: "exclamationmark.shield.fill")
                                    .font(.footnote)
                                    .foregroundColor(.orange)
                                    .multilineTextAlignment(.center)
                            } else if isVerified {
                                Label("Identidade verificada neste dispositivo", systemImage: "checkmark.circle.fill")
                                    .foregroundColor(.green)
                            }

                            Button(isVerified ? "Verificado" : "Marcar como verificado") {
                                UserDefaults.standard.set(fingerprint, forKey: storageKey)
                                storedFingerprint = fingerprint
                            }
                            .buttonStyle(.borderedProminent)
                            .disabled(fingerprint.isEmpty || isVerified)
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Segurança")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Fechar") { dismiss() }
                }
            }
            .task { await loadFingerprint() }
        }
    }

    private func loadFingerprint() async {
        storedFingerprint = UserDefaults.standard.string(forKey: storageKey)
        do {
            fingerprint = try await ZapLivreCore.shared.contactIdentityFingerprint(peerId: peerId)
            transparencyVerified = (try? await ZapLivreCore.shared.contactTransparencyProof(peerId: peerId))?.isEmpty == false
        } catch {
            errorMessage = "Não foi possível carregar a identidade deste contato."
        }
        isLoading = false
    }

    private func makeSecurityQRCode() -> UIImage {
        let filter = CIFilter.qrCodeGenerator()
        filter.message = Data("zaplivre-safety:v1:\(peerId):\(fingerprint)".utf8)
        filter.correctionLevel = "M"
        guard let output = filter.outputImage else {
            return UIImage(systemName: "qrcode") ?? UIImage()
        }
        let scaled = output.transformed(by: CGAffineTransform(scaleX: 8, y: 8))
        guard let cgImage = CIContext().createCGImage(scaled, from: scaled.extent) else {
            return UIImage(systemName: "qrcode") ?? UIImage()
        }
        return UIImage(cgImage: cgImage)
    }
}

struct MessageBubble: View {
    let message: Message
    let media: FfiMedia?

    var body: some View {
        HStack {
            if message.isOutgoing {
                Spacer()
            }

            VStack(alignment: message.isOutgoing ? .trailing : .leading, spacing: 4) {
                if message.messageType == "image", let media = media {
                    ImageMessageBubble(media: media, isOutgoing: message.isOutgoing)
                } else {
                    Text(message.content)
                        .font(ZapFont.body)
                        .padding(.horizontal, 12)
                        .padding(.vertical, 8)
                        .background(
                            (message.isOutgoing ? ZapColor.bubbleOut : ZapColor.bubbleIn)
                                .clipShape(BubbleShape(isOutgoing: message.isOutgoing, hasTail: true))
                        )
                        .foregroundColor(message.isOutgoing ? ZapColor.bubbleOutInk : ZapColor.bubbleInInk)
                        .overlay(
                            message.isOutgoing
                                ? nil
                                : BubbleShape(isOutgoing: false, hasTail: true)
                                    .stroke(ZapColor.hairline, lineWidth: 0.5)
                        )
                        .shadow(color: Color.black.opacity(0.05), radius: 1, x: 0, y: 1)
                }

                MessageStatusIndicator(
                    message: message.ffiMessage,
                    isOwnMessage: message.isOutgoing
                )
                .padding(.horizontal, 4)
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
    let messageType: String
    let senderId: String
    let timestamp: Date
    let isOutgoing: Bool
    let status: MessageStatus
    let ffiMessage: FfiMessageWrapper?  // Keep reference to original FfiMessage

    init(id: String, content: String, messageType: String, senderId: String, timestamp: Date, isOutgoing: Bool, status: MessageStatus, ffiMessage: FfiMessageWrapper? = nil) {
        self.id = id
        self.content = content
        self.messageType = messageType
        self.senderId = senderId
        self.timestamp = timestamp
        self.isOutgoing = isOutgoing
        self.status = status
        self.ffiMessage = ffiMessage
    }
}

struct ImageMessageBubble: View {
    let media: FfiMedia
    let isOutgoing: Bool
    @State private var image: UIImage?
    @State private var loadFailed = false

    var body: some View {
        Group {
            if let image = image {
                Image(uiImage: image)
                    .resizable()
                    .scaledToFit()
                    .frame(maxWidth: 220)
                    .clipShape(RoundedRectangle(cornerRadius: 12))
            } else if loadFailed {
                Label("Imagem indisponível", systemImage: "exclamationmark.triangle")
                    .font(.caption)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(isOutgoing ? Color.blue : Color.secondary.opacity(0.2))
                    .foregroundColor(isOutgoing ? .white : .primary)
                    .cornerRadius(16)
            } else {
                Text("Carregando imagem...")
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(isOutgoing ? Color.blue : Color.secondary.opacity(0.2))
                    .foregroundColor(isOutgoing ? .white : .primary)
                    .cornerRadius(16)
            }
        }
        .task {
            guard image == nil else { return }

            var candidateURLs: [URL] = []
            if let localPath = media.localPath {
                let storedURL = URL(fileURLWithPath: localPath)
                candidateURLs.append(storedURL)

                // iOS may relocate the app data container after reinstalling an
                // update. Preserve the path below Documents when that happens.
                if let documentsRange = localPath.range(of: "/Documents/") {
                    let relativePath = String(localPath[documentsRange.upperBound...])
                    if let documentsURL = FileManager.default.urls(
                        for: .documentDirectory,
                        in: .userDomainMask
                    ).first {
                        candidateURLs.append(documentsURL.appendingPathComponent(relativePath))
                    }
                }

            }

            // Older media rows may not have local_path populated even though
            // the core wrote the file using its deterministic hash-based name.
            if let documentsURL = FileManager.default.urls(
                for: .documentDirectory,
                in: .userDomainMask
            ).first {
                let fileExtension = media.fileName
                    .map { URL(fileURLWithPath: $0).pathExtension }
                    .flatMap { $0.isEmpty ? nil : $0 } ?? "jpg"
                candidateURLs.append(
                    documentsURL
                        .appendingPathComponent("zaplivre_data/media", isDirectory: true)
                        .appendingPathComponent("\(media.mediaHash).\(fileExtension)")
                )
            }

            for url in candidateURLs {
                if let localData = try? Data(contentsOf: url),
                   let localImage = UIImage(data: localData) {
                    image = localImage
                    return
                }
            }

            let timeoutTask = Task {
                try? await Task.sleep(nanoseconds: 10_000_000_000)
                guard !Task.isCancelled else { return }
                await MainActor.run {
                    if image == nil {
                        loadFailed = true
                    }
                }
            }
            defer { timeoutTask.cancel() }

            do {
                let data = try await ZapLivreCore.shared.downloadMedia(mediaHash: media.mediaHash)
                if let img = UIImage(data: data) {
                    image = img
                } else {
                    loadFailed = true
                }
            } catch {
                print("❌ Failed to load image: \(error)")
                loadFailed = true
            }
        }
    }
}

// MessageStatus enum is provided by the Rust FFI bindings (zaplivre.swift)
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
