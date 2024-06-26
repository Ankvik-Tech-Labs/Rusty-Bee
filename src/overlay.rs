use alloy_primitives::{keccak256, Address, FixedBytes};
use alloy_signer_wallet::LocalWallet;

pub type DistAddr = FixedBytes<HASH_SIZE>;

pub(crate) const HASH_SIZE: usize = 32;

pub type OverlayAddress = DistAddr;
pub(crate) type Nonce = FixedBytes<HASH_SIZE>;

pub trait Overlay {
    fn overlay(&self, network_id: u64, nonce: Option<Nonce>) -> OverlayAddress;
}

impl Overlay for LocalWallet {
    // Generates the overlay address for the signer.
    fn overlay(&self, network_id: u64, nonce: Option<Nonce>) -> OverlayAddress {
        calc_overlay(self.address(), network_id, nonce)
    }
}

impl Overlay for Address {
    // Generates the overlay address for the address.
    fn overlay(&self, network_id: u64, nonce: Option<Nonce>) -> OverlayAddress {
        calc_overlay(*self, network_id, nonce)
    }
}

fn calc_overlay(address: Address, network_id: u64, nonce: Option<Nonce>) -> OverlayAddress {
    let mut data = [0u8; 20 + 8 + 32];
    data[..20].copy_from_slice(address.0.as_slice());
    data[20..28].copy_from_slice(&network_id.to_le_bytes());
    if let Some(nonce) = nonce {
        data[28..60].copy_from_slice(nonce.as_slice());
    }

    keccak256(data)
}
