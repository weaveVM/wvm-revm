#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
/// `LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1` and `LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2`
/// was generated using this code:
/// ```rust
/// use alloy::hex;
///
/// use alloy_primitives::Address;
/// use sha3::{Digest, Keccak256};
///
/// const PHRASE_0XBABE1: &str = "LoadNetwork0xBabeSpecialProtocolAddress0xBabe1";
/// const PHRASE_0XBABE2: &str = "LoadNetwork0xBabeSpecialProtocolAddress0xBabe2";
///
/// fn manual_special_address(phrase: &str) -> Address {
///     let hash = Keccak256::digest(phrase.as_bytes());
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
/// fn main() {
///     let a1 = manual_special_address(PHRASE_0XBABE1);
///     let a2 = manual_special_address(PHRASE_0XBABE2);
///
///     println!("manual_special_address 0xbabe1:   0x{}", hex::encode(a1));
///     println!("manual_special_address 0xbabe2:   0x{}", hex::encode(a2));
/// }
/// ```
///
/// Resulting address 0xBabe1: `0x727e365e80e0770b1328ff6ec9d5685bf61d5b74`
/// Resulting address 0xBabe1: `0x976ffe04ddf4761df7eb0019a8edb6da0e190965`
use alloy_primitives::Address;
#[cfg(feature = "std")]
use std::vec::Vec;

pub const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1: Address = Address::new([
    0x72, 0x7e, 0x36, 0x5e, 0x80, 0xe0, 0x77, 0x0b, 0x13, 0x28, 0xff, 0x6e, 0xc9, 0xd5, 0x68, 0x5b,
    0xf6, 0x1d, 0x5b, 0x74,
]);

pub const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2: Address = Address::new([
    0x97, 0x6f, 0xfe, 0x04, 0xdd, 0xf4, 0x76, 0x1d, 0xf7, 0xeb, 0x00, 0x19, 0xa8, 0xed, 0xb6, 0xda,
    0x0e, 0x19, 0x09, 0x65,
]);

pub const LOAD_0XBABE_CALLDATA_TOKEN_COST: u64 = 2;

// For std test environments
#[cfg(feature = "std")]
thread_local! {
    pub(crate) static TEST_ADDRESSES: core::cell::RefCell<Option<Vec<Address>>> = core::cell::RefCell::new(None);
}

// For no_std test environments
#[cfg(not(feature = "std"))]
pub(crate) static mut TEST_ADDRESSES: Option<Vec<Address>> = None;

// For no_std test environments, use a static with unsafe access
#[cfg(all(test, not(feature = "std")))]
static mut TEST_ADDRESSES: Option<Vec<Address>> = None;

/// Check if an address is reserved for the 0xBabe protocol
#[inline]
pub fn is_reserved_address(address: Address) -> bool {
    #[cfg(feature = "std")]
    {
        if let Some(addresses) = TEST_ADDRESSES.with(|t| t.borrow().clone()) {
            return addresses.contains(&address);
        }
    }
    #[cfg(not(feature = "std"))]
    unsafe {
        if let Some(ref addresses) = TEST_ADDRESSES {
            return addresses.contains(&address);
        }
    }
    address == LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1
        || address == LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2
}

// Test helpers module - integrated into this file
pub mod test_helpers {
    use super::*;

    pub struct ReservedAddressGuard {
        #[cfg(feature = "std")]
        _phantom: core::marker::PhantomData<()>,
    }

    impl Drop for ReservedAddressGuard {
        fn drop(&mut self) {
            // Reset test addresses when guard is dropped
            #[cfg(feature = "std")]
            {
                super::TEST_ADDRESSES.with(|t| *t.borrow_mut() = None);
            }

            #[cfg(not(feature = "std"))]
            unsafe {
                super::TEST_ADDRESSES = None;
            }
        }
    }

    /// Set custom reserved addresses for testing
    /// Returns a guard that will reset the addresses when dropped
    pub fn with_test_reserved_addresses(addresses: Vec<Address>) -> ReservedAddressGuard {
        #[cfg(feature = "std")]
        {
            TEST_ADDRESSES.with(|t| *t.borrow_mut() = Some(addresses));
        }

        #[cfg(not(feature = "std"))]
        unsafe {
            super::TEST_ADDRESSES = Some(addresses);
        }

        ReservedAddressGuard {
            #[cfg(feature = "std")]
            _phantom: core::marker::PhantomData,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::test_helpers::with_test_reserved_addresses;
    use super::*;

    const PHRASE_0XBABE1: &str = "LoadNetwork0xBabeSpecialProtocolAddress0xBabe1";
    const PHRASE_0XBABE2: &str = "LoadNetwork0xBabeSpecialProtocolAddress0xBabe2";

    fn alloy_special_address_0xbabe1() -> Address {
        let hash = alloy_primitives::keccak256(PHRASE_0XBABE1.as_bytes());
        Address::from_word(hash)
    }

    fn alloy_special_address_0xbabe2() -> Address {
        let hash = alloy_primitives::keccak256(PHRASE_0XBABE2.as_bytes());
        Address::from_word(hash)
    }

    #[test]
    fn test_address_generation() {
        let test_address = alloy_special_address_0xbabe1();
        assert_eq!(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1, test_address,
            "0xBabe1 addresses do not match!"
        );

        let test_address = alloy_special_address_0xbabe2();
        assert_eq!(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2, test_address,
            "0xBabe2 addresses do not match!"
        );
    }

    #[test]
    fn test_is_reserved_address_production() {
        // Test with production addresses
        assert!(is_reserved_address(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1
        ));
        assert!(is_reserved_address(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2
        ));

        // Test with random address
        let random_addr = Address::new([0x11; 20]);
        assert!(!is_reserved_address(random_addr));
    }

    #[test]
    fn test_with_custom_reserved_addresses() {
        let custom_addr1 = Address::new([0x01; 20]);
        let custom_addr2 = Address::new([0x02; 20]);

        // Set custom addresses
        let _guard = with_test_reserved_addresses(vec![custom_addr1, custom_addr2]);

        // Now these should be reserved
        assert!(is_reserved_address(custom_addr1));
        assert!(is_reserved_address(custom_addr2));

        // Production addresses should NOT be reserved while testing
        assert!(!is_reserved_address(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1
        ));
        assert!(!is_reserved_address(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2
        ));

        // Random address still not reserved
        let random_addr = Address::new([0x33; 20]);
        assert!(!is_reserved_address(random_addr));
    }

    #[test]
    fn test_guard_cleanup() {
        let custom_addr = Address::new([0x99; 20]);

        // Before setting custom addresses
        assert!(!is_reserved_address(custom_addr));

        {
            let _guard = with_test_reserved_addresses(vec![custom_addr]);
            // Inside scope, custom address is reserved
            assert!(is_reserved_address(custom_addr));
        }

        // After guard is dropped, back to normal
        assert!(!is_reserved_address(custom_addr));
        assert!(is_reserved_address(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1
        ));
    }
}
