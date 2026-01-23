//
//  MePassaApp.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI
import CallKit

@main
struct MePassaApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    @StateObject private var appState = AppState()
    @StateObject private var callManager = CallManager()
    @StateObject private var pushManager = PushNotificationManager()

    init() {
        // Initialize MePassa Core
        initializeMePassaCore()

        // Setup CallKit
        setupCallKit()
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(appState)
                .environmentObject(callManager)
                .environmentObject(pushManager)
                .onAppear {
                    // Connect AppDelegate with PushManager
                    appDelegate.pushManager = pushManager

                    // Request push notification permissions
                    pushManager.requestAuthorization()
                }
        }
    }
    
    private func initializeMePassaCore() {
        print("📱 Initializing MePassa Core...")

        Task {
            do {
                try await MePassaCore.shared.initialize()
                print("✅ MePassa Core initialized successfully")
            } catch {
                print("❌ Failed to initialize MePassa Core: \(error)")
            }
        }
    }
    
    private func setupCallKit() {
        // Configure CallKit provider
        callManager.configure()
    }
}

/// App-wide state management
class AppState: ObservableObject {
    @Published var isAuthenticated = false
    @Published var currentUser: User?
    @Published var conversations: [Conversation] = []
    @Published var groups: [ChatGroup] = []

    private var refreshTimer: Timer?

    func login(peerId: String) {
        // TODO: Implement login with UniFFI
        self.isAuthenticated = true
        print("✅ Logged in as: \(peerId)")

        // Start auto-refresh when logged in
        startAutoRefresh()
    }

    func logout() {
        self.isAuthenticated = false
        self.currentUser = nil
        self.conversations = []
        self.groups = []

        // Stop auto-refresh when logged out
        stopAutoRefresh()
    }

    /// Load conversations from MePassaCore
    func loadConversations() {
        Task {
            do {
                let convs = try await MePassaCore.shared.listConversations()

                // Convert FFI conversations to local model
                await MainActor.run {
                    self.conversations = convs.compactMap { ffiConv in
                        // Only include conversations that have a peer_id
                        guard let peerId = ffiConv.peerId else { return nil }

                        let displayName: String
                        if let name = ffiConv.displayName {
                            displayName = name
                        } else {
                            displayName = String(peerId.prefix(12)) + "..."
                        }

                        return Conversation(
                            id: ffiConv.id,
                            peerId: peerId,
                            displayName: displayName,
                            lastMessage: nil, // FFI doesn't include message text, only ID
                            unreadCount: Int(ffiConv.unreadCount)
                        )
                    }

                    print("✅ Loaded \(self.conversations.count) conversations")
                }
            } catch {
                print("❌ Failed to load conversations: \(error)")
            }
        }
    }

    /// Start auto-refresh timer (every 5 seconds)
    private func startAutoRefresh() {
        // Load immediately
        loadConversations()

        // Then refresh every 5 seconds
        refreshTimer?.invalidate()
        refreshTimer = Timer.scheduledTimer(withTimeInterval: 5.0, repeats: true) { [weak self] _ in
            self?.loadConversations()
        }
    }

    /// Stop auto-refresh timer
    private func stopAutoRefresh() {
        refreshTimer?.invalidate()
        refreshTimer = nil
    }
}

/// Temporary models (will be replaced by UniFFI generated types)
struct User: Identifiable {
    let id: String
    let username: String?
    let peerId: String
}

struct Conversation: Identifiable {
    let id: String
    let peerId: String
    let displayName: String
    let lastMessage: String?
    let unreadCount: Int
}

struct ChatGroup: Identifiable {
    let id: String
    let name: String
    let description: String?
    let memberCount: Int
    let isAdmin: Bool
    let createdAt: Date
}
