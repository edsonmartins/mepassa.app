//
//  MessageStatusIndicator.swift
//  MePassa
//
//  Created by MePassa Team
//  Copyright © 2026 MePassa. All rights reserved.
//

import SwiftUI

/// MessageStatusIndicator - Shows message status and timestamp
struct MessageStatusIndicator: View {
    let message: FfiMessage
    let isOwnMessage: Bool

    var body: some View {
        HStack(spacing: 4) {
            // Timestamp
            Text(message.formattedTime)
                .font(.caption2)
                .foregroundColor(.secondary.opacity(0.8))

            // Status indicator (only for own messages)
            if isOwnMessage {
                Text(message.statusIcon)
                    .font(.caption2)
                    .foregroundColor(statusColor)
            }
        }
    }

    private var statusColor: Color {
        switch message.status {
        case .read:
            return Color(red: 0.01, green: 0.53, blue: 0.82) // Blue
        case .failed:
            return .red
        default:
            return .secondary.opacity(0.8)
        }
    }
}

/// Full message status with description
struct MessageStatusFull: View {
    let message: FfiMessage

    var body: some View {
        HStack(spacing: 4) {
            Text(message.statusDescription)
                .font(.caption)
                .foregroundColor(statusColor)

            Text(message.statusIcon)
                .font(.caption)
                .foregroundColor(statusColor)
        }
    }

    private var statusColor: Color {
        switch message.status {
        case .read:
            return Color(red: 0.01, green: 0.53, blue: 0.82)
        case .failed:
            return .red
        default:
            return .secondary
        }
    }
}

#Preview {
    VStack(spacing: 16) {
        MessageStatusIndicator(
            message: FfiMessage(
                messageId: "1",
                conversationId: "conv1",
                senderPeerId: "peer1",
                recipientPeerId: "peer2",
                messageType: "text",
                contentPlaintext: "Hello",
                createdAt: Int64(Date().timeIntervalSince1970),
                sentAt: nil,
                receivedAt: nil,
                readAt: nil,
                status: .sent,
                isDeleted: false
            ),
            isOwnMessage: true
        )

        MessageStatusIndicator(
            message: FfiMessage(
                messageId: "2",
                conversationId: "conv1",
                senderPeerId: "peer1",
                recipientPeerId: "peer2",
                messageType: "text",
                contentPlaintext: "Hello",
                createdAt: Int64(Date().timeIntervalSince1970),
                sentAt: nil,
                receivedAt: nil,
                readAt: Int64(Date().timeIntervalSince1970),
                status: .read,
                isDeleted: false
            ),
            isOwnMessage: true
        )
    }
    .padding()
}
