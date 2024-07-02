pub mod swarm {
    pub mod handshake {
        include!(concat!(env!("OUT_DIR"), "/swarm.handshake.rs"));
    }
}
// https://docs.rs/prost-build/latest/prost_build/

extern crate prost;
// extern crate libp2p;
// use libp2p::StreamProtocol;
pub use swarm::handshake;

// const HANDSHAKE_PROTOCOL: StreamProtocol = StreamProtocol::new("/swarm/handshake/11.0.0/handshake");
