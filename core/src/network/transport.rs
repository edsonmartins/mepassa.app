//! Transport Layer
//!
//! Manages libp2p transport with TCP, QUIC, Noise encryption, and Yamux multiplexing.

use libp2p::{
    core::{muxing::StreamMuxerBox, transport::Boxed, upgrade},
    identity::Keypair,
    noise, quic, tcp, yamux, PeerId, Transport,
};
use std::time::Duration;

use crate::utils::error::{MePassaError, Result};

/// Build a libp2p transport with:
/// - TCP + QUIC (dual-stack)
/// - Noise encryption
/// - Yamux multiplexing
pub fn build_transport(keypair: &Keypair) -> Result<Boxed<(PeerId, StreamMuxerBox)>> {
    // TCP transport with Noise + Yamux
    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(keypair).map_err(|e| {
            MePassaError::Network(format!("Failed to create Noise config: {}", e))
        })?)
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    // QUIC transport (built-in encryption + multiplexing)
    let quic_transport = quic::async_std::Transport::new(quic::Config::new(keypair))
        .map(|(peer_id, muxer), _| (peer_id, StreamMuxerBox::new(muxer)))
        .boxed();

    // Combine transports (try QUIC first, fallback to TCP)
    let transport = quic_transport
        .or_transport(tcp_transport)
        .map(|either, _| either.into_inner())
        .boxed();

    Ok(transport)
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity;

    #[test]
    fn test_build_transport() {
        let keypair = identity::Keypair::generate_ed25519();
        let transport = build_transport(&keypair);

        assert!(transport.is_ok());
    }

    #[test]
    fn test_transport_with_different_keypairs() {
        let keypair1 = identity::Keypair::generate_ed25519();
        let keypair2 = identity::Keypair::generate_ed25519();

        let transport1 = build_transport(&keypair1);
        let transport2 = build_transport(&keypair2);

        assert!(transport1.is_ok());
        assert!(transport2.is_ok());
    }
}
