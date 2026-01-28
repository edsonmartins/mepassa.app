# Issues Backlog (Prioritized)

## P0 — Critical (Blockers)
1) **Send ACK responses over the network**
   - Fix TODO in `core/src/network/swarm.rs#L496` (ACK created but not sent).
   - Ensure message status transitions (Sent → Delivered) are observed in clients.

2) **Implement event bus from core → apps**
   - Wire `MessageHandler` event channel to Client callbacks/FFI.
   - Avoid UI polling; push updates on incoming messages/acks.

3) **Identity backup/restore (iOS + Android)**
   - Implement export/import in `ios/MePassa/MePassa/Core/MePassaCore.swift`.
   - Provide UI flows to restore identity and keep stable peerId.

4) **Bootstrap peers configured in apps**
   - Provide default bootstrap node list (env/config).
   - Add to ClientBuilder via `add_bootstrap_peer` on desktop/iOS/Android.

## P1 — High
5) **NAT detection + relay strategy**
   - Replace placeholder in `core/src/network/nat_detection.rs`.
   - Ensure relay reservation and circuit dial flow are stable.

6) **Group security verification**
   - Verify signatures in `core/src/group/manager.rs`.
   - Reject invalid sender keys.

7) **Media pipeline completeness**
   - Replace media TODOs in `core/src/api/client.rs` (send/download/store).
   - Add media storage paths and cleanup.

8) **VoIP stubs cleanup or feature gating**
   - Either implement or hide UI; remove stubs in FFI when disabled.

## P2 — Medium
9) **DHT address refresh loop**
   - Periodic republish of own address; handle TTL/expiry.

10) **Outbound message retry/queue**
   - Persist outbound queue and retry with backoff if offline.

11) **Push notifications (iOS/Android)**
   - Use production push URL, include peerId, wire navigation.

12) **Message ordering and sorting spec**
   - Define ordering by timestamp + tie-breaker across platforms.

## P3 — Low / UI polish
13) **Remove UI mocks/placeholders**
   - Groups, profile, settings, media empty states.

14) **Settings cache management**
   - Implement clear image/video cache actions.

15) **Docs and architecture diagrams**
   - Update architecture docs to reflect DHT + bootstrap strategy.
