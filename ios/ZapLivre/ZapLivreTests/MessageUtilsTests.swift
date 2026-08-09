//
//  MessageUtilsTests.swift
//  ZapLivreTests
//
//  Copyright © 2026 ZapLivre. All rights reserved.
//
//  Pure-logic tests for MessageUtils (timestamp formatting and
//  message status presentation). No FFI client involved.

import XCTest
@testable import ZapLivre

final class MessageUtilsTests: XCTestCase {

    private func timestamp(secondsAgo: TimeInterval) -> Int64 {
        Int64(Date().addingTimeInterval(-secondsAgo).timeIntervalSince1970)
    }

    // MARK: - formatTimestamp (relative)

    func testFormatTimestampJustNow() {
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 5)), "agora")
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 50)), "agora")
    }

    /// F6: o defeito "sempre 'agora'" foi corrigido comparando o intervalo
    /// total (`now.timeIntervalSince(messageDate)`) em vez dos restos por
    /// unidade. Este teste agora valida o comportamento correto.
    func testFormatTimestampMinutesHoursAndDate() {
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 5 * 60 + 2)), "5min")
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 59 * 60 + 59)), "59min")
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 3 * 3600 + 2)), "3h")
        XCTAssertEqual(MessageUtils.formatTimestamp(timestamp(secondsAgo: 23 * 3600 + 59 * 60)), "23h")
        // Timestamp de 2020 (ano diferente) -> dd/MM/yyyy
        let formatted = MessageUtils.formatTimestamp(1_584_014_400)
        XCTAssertTrue(formatted.hasSuffix("/2020"), "Expected dd/MM/yyyy for 2020, got \(formatted)")
    }

    // MARK: - formatFullTimestamp

    func testFormatFullTimestampToday() {
        let result = MessageUtils.formatFullTimestamp(timestamp(secondsAgo: 60))
        XCTAssertTrue(result.hasPrefix("Hoje "), "Expected 'Hoje HH:mm', got \(result)")
    }

    func testFormatFullTimestampYesterday() {
        // Same wall-clock time yesterday is always "yesterday"
        let result = MessageUtils.formatFullTimestamp(timestamp(secondsAgo: 24 * 3600))
        XCTAssertTrue(result.hasPrefix("Ontem "), "Expected 'Ontem HH:mm', got \(result)")
    }

    // MARK: - Status presentation

    func testStatusIcons() {
        XCTAssertEqual(MessageUtils.getStatusIcon(.pending), "⏱️")
        XCTAssertEqual(MessageUtils.getStatusIcon(.sent), "✓")
        XCTAssertEqual(MessageUtils.getStatusIcon(.delivered), "✓✓")
        XCTAssertEqual(MessageUtils.getStatusIcon(.read), "✓✓")
        XCTAssertEqual(MessageUtils.getStatusIcon(.failed), "❌")
    }

    func testStatusDescriptions() {
        XCTAssertEqual(MessageUtils.getStatusDescription(.pending), "Enviando...")
        XCTAssertEqual(MessageUtils.getStatusDescription(.sent), "Enviado")
        XCTAssertEqual(MessageUtils.getStatusDescription(.delivered), "Entregue")
        XCTAssertEqual(MessageUtils.getStatusDescription(.read), "Lido")
        XCTAssertEqual(MessageUtils.getStatusDescription(.failed), "Falha no envio")
    }

    func testStatusColors() {
        XCTAssertEqual(MessageUtils.getStatusColor(.read), "blue")
        XCTAssertEqual(MessageUtils.getStatusColor(.failed), "red")
        XCTAssertEqual(MessageUtils.getStatusColor(.pending), "gray")
        XCTAssertEqual(MessageUtils.getStatusColor(.sent), "gray")
        XCTAssertEqual(MessageUtils.getStatusColor(.delivered), "gray")
    }
}
