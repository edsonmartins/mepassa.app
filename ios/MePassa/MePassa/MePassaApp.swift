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
        // TODO: Initialize UniFFI bindings and MePassa core library
        print("📱 Initializing MePassa Core...")
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

    func login(peerId: String) {
        // TODO: Implement login with UniFFI
        self.isAuthenticated = true
        print("✅ Logged in as: \(peerId)")
    }

    func logout() {
        self.isAuthenticated = false
        self.currentUser = nil
        self.conversations = []
        self.groups = []
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
