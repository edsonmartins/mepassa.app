//
//  MePassaCore.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//
//  Swift wrapper around UniFFI generated bindings
//  This provides a cleaner API for SwiftUI views

import Foundation
import mepassa

/// Swift wrapper for MePassa Core FFI
class MePassaCore: ObservableObject {
    private var client: MePassaClient?

    private let dataDir: String
    @Published var isInitialized = false
    @Published var localPeerId: String?

    init(dataDir: String) {
        self.dataDir = dataDir
    }

    // MARK: - Initialization

    /// Initialize the MePassa core library
    func initialize() async throws {
        print("📱 MePassa Core initializing at: \(dataDir)")

        client = try await MePassaClient(dataDir: dataDir)
        localPeerId = try await client?.localPeerId()

        DispatchQueue.main.async {
            self.isInitialized = true
        }

        print("✅ MePassa Core initialized with peer ID: \(localPeerId ?? "unknown")")
    }

    // MARK: - Identity Management

    /// Generate new identity (keypair)
    func generateNewIdentity() async throws -> String {
        // This is done during initialization
        // The peer ID is derived from the public key
        return try await client?.localPeerId() ?? ""
    }

    /// Import existing identity from backup
    func importIdentity(backup: String) async throws {
        // TODO: Implement identity import
        throw MePassaCoreError.notImplemented("Identity import not yet implemented")
    }

    /// Export current identity for backup
    func exportIdentity() async throws -> String {
        // TODO: Export keypair securely
        throw MePassaCoreError.notImplemented("Identity export not yet implemented")
    }

    // MARK: - Networking

    /// Start listening on default multiaddrs
    func startListening() async throws {
        try await client?.listenOn(multiaddr: "/ip4/0.0.0.0/tcp/0")
        try await client?.listenOn(multiaddr: "/ip6/::/tcp/0")
        print("📡 Started listening on P2P network")
    }

    /// Connect to bootstrap nodes
    func bootstrap() async throws {
        try await client?.bootstrap()
        print("🌐 Connected to bootstrap nodes")
    }

    /// Connect to a specific peer
    func connectToPeer(peerId: String, multiaddr: String) async throws {
        try await client?.connectToPeer(peerId: peerId, multiaddr: multiaddr)
        print("🔗 Connecting to peer: \(peerId)")
    }

    /// Get count of connected peers
    func connectedPeersCount() async throws -> Int {
        return try await Int(client?.connectedPeersCount() ?? 0)
    }

    // MARK: - Messaging

    /// Send text message to peer
    func sendMessage(to peerId: String, content: String) async throws -> String {
        let messageId = try await client?.sendTextMessage(toPeerId: peerId, content: content) ?? ""
        print("📨 Sent message to \(peerId): \(content)")
        return messageId
    }

    /// Get messages for a conversation
    func getMessages(for peerId: String, limit: Int? = nil) async throws -> [FfiMessageWrapper] {
        let messages = try await client?.getConversationMessages(
            peerId: peerId,
            limit: limit.map { UInt32($0) },
            offset: nil
        ) ?? []

        return messages.map { FfiMessageWrapper(ffi: $0) }
    }

    /// Get all conversations
    func listConversations() async throws -> [FfiConversationWrapper] {
        let conversations = try await client?.listConversations() ?? []
        return conversations.map { FfiConversationWrapper(ffi: $0) }
    }

    /// Mark conversation as read
    func markAsRead(peerId: String) async throws {
        try await client?.markConversationRead(peerId: peerId)
        print("✅ Marked conversation as read: \(peerId)")
    }

    // MARK: - VoIP Calls

    /// Start voice call
    func startCall(to peerId: String) async throws -> String {
        let callId = try await client?.startCall(toPeerId: peerId) ?? ""
        print("📞 Starting call to: \(peerId)")
        return callId
    }

    /// Accept incoming call
    func acceptCall(callId: String) async throws {
        try await client?.acceptCall(callId: callId)
        print("✅ Accepted call: \(callId)")
    }

    /// Reject incoming call
    func rejectCall(callId: String, reason: String? = nil) async throws {
        try await client?.rejectCall(callId: callId, reason: reason)
        print("❌ Rejected call: \(callId)")
    }

    /// Hang up active call
    func hangupCall(callId: String) async throws {
        try await client?.hangupCall(callId: callId)
        print("📴 Hung up call: \(callId)")
    }

    /// Toggle mute status
    func toggleMute(callId: String) async throws {
        try await client?.toggleMute(callId: callId)
        print("🔇 Toggled mute for call: \(callId)")
    }

    /// Toggle speakerphone
    func toggleSpeaker(callId: String) async throws {
        try await client?.toggleSpeakerphone(callId: callId)
        print("🔊 Toggled speaker for call: \(callId)")
    }

    // MARK: - Groups (FASE 15)

    /// Create a new group
    func createGroup(name: String, description: String?) async throws -> FfiGroupWrapper {
        let group = try await client?.createGroup(name: name, description: description)
        print("👥 Created group: \(name)")
        return FfiGroupWrapper(ffi: group!)
    }

    /// Join an existing group
    func joinGroup(groupId: String, groupName: String) async throws {
        try await client?.joinGroup(groupId: groupId, groupName: groupName)
        print("✅ Joined group: \(groupName)")
    }

    /// Leave a group
    func leaveGroup(groupId: String) async throws {
        try await client?.leaveGroup(groupId: groupId)
        print("👋 Left group: \(groupId)")
    }

    /// Add member to group (admin only)
    func addGroupMember(groupId: String, peerId: String) async throws {
        try await client?.addGroupMember(groupId: groupId, peerId: peerId)
        print("➕ Added member to group \(groupId): \(peerId)")
    }

    /// Remove member from group (admin only)
    func removeGroupMember(groupId: String, peerId: String) async throws {
        try await client?.removeGroupMember(groupId: groupId, peerId: peerId)
        print("➖ Removed member from group \(groupId): \(peerId)")
    }

    /// Get all groups
    func getGroups() async throws -> [FfiGroupWrapper] {
        let groups = try await client?.getGroups() ?? []
        return groups.map { FfiGroupWrapper(ffi: $0) }
    }

    /// Get group messages
    func getGroupMessages(groupId: String, limit: Int? = nil) async throws -> [FfiMessageWrapper] {
        // TODO: Implement when group messaging is available
        /*
        let messages = try await client?.getGroupMessages(
            groupId: groupId,
            limit: limit.map { UInt32($0) },
            offset: nil
        ) ?? []
        return messages.map { FfiMessageWrapper(ffi: $0) }
        */

        return [] // Mock
    }

    /// Send message to group
    func sendGroupMessage(groupId: String, content: String) async throws -> String {
        // TODO: Implement when group messaging is available
        /*
        return try await client?.sendGroupMessage(groupId: groupId, content: content) ?? ""
        */

        // Mock
        let messageId = UUID().uuidString
        print("📨 Sent group message to \(groupId): \(content)")
        return messageId
    }
}

// MARK: - Wrapper Types

/// Swift wrapper for FfiMessage (from UniFFI)
struct FfiMessageWrapper: Identifiable {
    let id: String
    let conversationId: String
    let senderPeerId: String
    let recipientPeerId: String?
    let content: String?
    let createdAt: Date
    let status: MessageStatus

    init(id: String, conversationId: String, senderPeerId: String, recipientPeerId: String?, content: String?, createdAt: Date, status: MessageStatus) {
        self.id = id
        self.conversationId = conversationId
        self.senderPeerId = senderPeerId
        self.recipientPeerId = recipientPeerId
        self.content = content
        self.createdAt = createdAt
        self.status = status
    }

    init(ffi: FfiMessage) {
        self.id = ffi.messageId
        self.conversationId = ffi.conversationId
        self.senderPeerId = ffi.senderPeerId
        self.recipientPeerId = ffi.recipientPeerId
        self.content = ffi.contentPlaintext
        self.createdAt = Date(timeIntervalSince1970: TimeInterval(ffi.createdAt) / 1000.0)
        self.status = ffi.status
    }
}

/// Swift wrapper for FfiConversation (from UniFFI)
struct FfiConversationWrapper: Identifiable {
    let id: String
    let peerId: String?
    let displayName: String?
    let lastMessageId: String?
    let lastMessageAt: Date?
    let unreadCount: Int

    init(id: String, peerId: String?, displayName: String?, lastMessageId: String?, lastMessageAt: Date?, unreadCount: Int) {
        self.id = id
        self.peerId = peerId
        self.displayName = displayName
        self.lastMessageId = lastMessageId
        self.lastMessageAt = lastMessageAt
        self.unreadCount = unreadCount
    }

    init(ffi: FfiConversation) {
        self.id = ffi.id
        self.peerId = ffi.peerId
        self.displayName = ffi.displayName
        self.lastMessageId = ffi.lastMessageId
        self.lastMessageAt = ffi.lastMessageAt.map { Date(timeIntervalSince1970: TimeInterval($0) / 1000.0) }
        self.unreadCount = Int(ffi.unreadCount)
    }
}

/// Swift wrapper for FfiGroup (from UniFFI)
struct FfiGroupWrapper: Identifiable {
    let id: String
    let name: String
    let description: String?
    let avatarHash: String?
    let creatorPeerId: String
    let memberCount: Int
    let isAdmin: Bool
    let createdAt: Date

    init(id: String, name: String, description: String?, avatarHash: String?, creatorPeerId: String, memberCount: Int, isAdmin: Bool, createdAt: Date) {
        self.id = id
        self.name = name
        self.description = description
        self.avatarHash = avatarHash
        self.creatorPeerId = creatorPeerId
        self.memberCount = memberCount
        self.isAdmin = isAdmin
        self.createdAt = createdAt
    }

    init(ffi: FfiGroup) {
        self.id = ffi.id
        self.name = ffi.name
        self.description = ffi.description
        self.avatarHash = ffi.avatarHash
        self.creatorPeerId = ffi.creatorPeerId
        self.memberCount = Int(ffi.memberCount)
        self.isAdmin = ffi.isAdmin
        self.createdAt = Date(timeIntervalSince1970: TimeInterval(ffi.createdAt))
    }
}

// MARK: - Errors

enum MePassaCoreError: LocalizedError {
    case notInitialized
    case notImplemented(String)
    case networkError(String)
    case storageError(String)
    case cryptoError(String)

    var errorDescription: String? {
        switch self {
        case .notInitialized:
            return "MePassa Core not initialized"
        case .notImplemented(let feature):
            return "Feature not yet implemented: \(feature)"
        case .networkError(let message):
            return "Network error: \(message)"
        case .storageError(let message):
            return "Storage error: \(message)"
        case .cryptoError(let message):
            return "Crypto error: \(message)"
        }
    }
}

// MARK: - Helper Extensions

extension MessageStatus {
    var displayText: String {
        switch self {
        case .pending: return "Pending"
        case .sent: return "Sent"
        case .delivered: return "Delivered"
        case .read: return "Read"
        case .failed: return "Failed"
        }
    }
}
