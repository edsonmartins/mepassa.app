import Foundation

extension Notification.Name {
    static let zapLivreWebRtcSignal = Notification.Name("zapLivreWebRtcSignal")
}

enum WebRtcSignalKind: String {
    case offer
    case answer
    case ice
}

/// Converts UniFFI callbacks into process-local events consumed by the active media session.
final class WebRtcSignalingBridge: NSObject, FfiWebRtcSignalingCallback, @unchecked Sendable {
    static let shared = WebRtcSignalingBridge()

    func onOffer(callId: String, sdp: String) {
        post(.offer, callId: callId, values: ["sdp": sdp])
    }

    func onAnswer(callId: String, sdp: String) {
        post(.answer, callId: callId, values: ["sdp": sdp])
    }

    func onIceCandidate(
        callId: String,
        candidate: String,
        sdpMid: String?,
        sdpMLineIndex: UInt16?
    ) {
        post(.ice, callId: callId, values: [
            "candidate": candidate,
            "sdpMid": sdpMid as Any,
            "sdpMLineIndex": sdpMLineIndex as Any
        ])
    }

    private func post(_ kind: WebRtcSignalKind, callId: String, values: [String: Any]) {
        var userInfo = values
        userInfo["kind"] = kind.rawValue
        userInfo["callId"] = callId
        DispatchQueue.main.async {
            NotificationCenter.default.post(
                name: .zapLivreWebRtcSignal,
                object: self,
                userInfo: userInfo
            )
        }
    }
}
