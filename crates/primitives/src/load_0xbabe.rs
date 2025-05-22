/// `LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS` was generated using this code:
///
/// ```rust
/// use alloy::hex;
/// use alloy_primitives::Address;
/// use sha3::{Digest, Keccak256};
///
/// const PHRASE: &str = "LoadNetwork0xBabeSpecialProtocolAddress";
///
/// fn manual_special_address() -> Address {
///     let hash = Keccak256::digest(PHRASE.as_bytes());
///     let addr = Address::from_slice(&hash[12..]);
///     print!("Address::new([");
///     for (i, byte) in addr.iter().enumerate() {
///         print!("0x{:02x}", byte);
///         if i != 19 {
///             print!(", ");
///         }
///     }
///     println!("]);");
///     addr
/// }
///
/// fn alloy_special_address() -> Address {
///     let hash = alloy_primitives::keccak256(PHRASE.as_bytes());
///     Address::from_word(hash)
/// }
///
/// fn main() {
///     let a1 = manual_special_address();
///     let a2 = alloy_special_address();
///
///     println!("manual_special_address:   0x{}", hex::encode(a1));
///     println!("alloy_special_address:    0x{}", hex::encode(a2));
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn both_methods_match() {
///         let a1 = manual_special_address();
///         let a2 = alloy_special_address();
///         assert_eq!(a1, a2, "Addresses do not match!");
///     }
/// }
/// ```
///
/// Resulting address: `0x0df59bf80fe3559c00e78933107a2867b648e466`
use alloy_primitives::Address;

const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS: Address = Address::new([
    0x0d, 0xf5, 0x9b, 0xf8, 0x0f, 0xe3, 0x55, 0x9c, 0x00, 0xe7, 0x89, 0x33, 0x10, 0x7a, 0x28, 0x67,
    0xb6, 0x48, 0xe4, 0x66,
]);

mod tests {
    use super::*;
    const PHRASE: &str = "LoadNetwork0xBabeSpecialProtocolAddress";

    fn alloy_special_address() -> Address {
        let hash = alloy_primitives::keccak256(PHRASE.as_bytes());
        Address::from_word(hash)
    }

    #[test]
    fn both_methods_match() {
        let test_address = alloy_special_address();
        assert_eq!(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS, test_address,
            "Addresses do not match!"
        );
    }
}
