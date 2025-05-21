/// LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS was generated from this code
/// use alloy_primitives::Address;
/// use sha3::{Digest, Keccak256};

/// const SPECIAL_ADDR_PHRASE: &str = "0xBabe_special_protocol_address_on_load_network";

/// pub fn special_protocol_address() -> Address {
///     let hash = Keccak256::digest(SPECIAL_ADDR_PHRASE.as_bytes());
///    Address::from_slice(&hash[12..]) // Take the last 20 bytes
/// }
use alloy_primitives::Address;

const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS: Address = Address::new([
    0x3d, 0x38, 0xeb, 0xe3, 0xeb, 0xd1, 0xe9, 0x79, 0x01, 0x25, 0x71, 0x7b, 0xcc, 0x61, 0x9d, 0x08,
    0xb2, 0xf8, 0xe4, 0x13,
]);
