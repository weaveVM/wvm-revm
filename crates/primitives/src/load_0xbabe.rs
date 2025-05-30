#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
/// `LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1` and `LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2`
/// were generated using the following code:
///
/// ```rust
/// use alloy::hex;
/// use alloy_primitives::Address;
///
/// /// Pads a short suffix like `0xBABE0001` into a full 20-byte Ethereum address
/// fn vanity_address_from_suffix(suffix: [u8; 4]) -> Address {
///     let mut full = [0u8; 20];
///     full[16..20].copy_from_slice(&suffix);
///     Address::new(full)
/// }
///
/// fn main() {
///     let a1 = vanity_address_from_suffix([0x00, 0xBA, 0xBE, 0x01]);
///     let a2 = vanity_address_from_suffix([0x00, 0xBA, 0xBE, 0x02]);
///
///     println!("Address 0xBABE0001: 0x{}", hex::encode(a1));
///     println!("Address 0xBABE0002: 0x{}", hex::encode(a2));
/// }
/// ```
///
/// Resulting addresses:
/// - `0xBABE0001` → `0x0000000000000000000000000000000000babe01`
/// - `0xBABE0002` → `0x0000000000000000000000000000000000babe02`
use alloy_primitives::Address;
#[cfg(feature = "std")]
use std::vec::Vec;

/// LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1 : 0x0000000000000000000000000000000000babe01
pub const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1: Address = Address::new([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0xba, 0xbe, 0x01,
]);

/// LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2: 0x0000000000000000000000000000000000babe02
pub const LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2: Address = Address::new([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0xba, 0xbe, 0x02,
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

    #[test]
    fn test_address_generation() {
        let expected1 = {
            let mut b = [0u8; 20];
            b[16..20].copy_from_slice(&[0x00, 0xBA, 0xBE, 0x01]);
            Address::new(b)
        };

        let expected2 = {
            let mut b = [0u8; 20];
            b[16..20].copy_from_slice(&[0x00, 0xBA, 0xBE, 0x02]);
            Address::new(b)
        };

        assert_eq!(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE1, expected1,
            "0xBabe1 address does not match expected suffix layout"
        );
        assert_eq!(
            LOAD_NETWORK_0XBABE_SPECIAL_ADDRESS_0XBABE2, expected2,
            "0xBabe2 address does not match expected suffix layout"
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
