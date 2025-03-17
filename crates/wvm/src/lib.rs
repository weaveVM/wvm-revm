use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static NON_ZERO_BYTE_COST: Lazy<u64> = Lazy::new(|| {
    let original =
        std::env::var("NON_ZERO_BYTE_COST_ORIGINAL").unwrap_or_else(|_| "false".to_string());

    if original == "true" {
        16
    } else {
        8
    }
});

#[derive(Deserialize, Serialize, Debug)]
pub struct Bundlers {
    bundler_0xbabe1: String,
    bundler_0xbabe2: String,
}

impl Bundlers {
    pub fn verify_target(&self, target: String) -> bool {
        self.bundler_0xbabe1 == target || self.bundler_0xbabe2 == target
    }
}

pub const ADDRESS_BOOK_URL: &str =
    "https://raw.githubusercontent.com/weaveVM/miscalleneous/refs/heads/main/address_book.json";

pub static BUNDLER_ADDRESSES: Lazy<Bundlers> = Lazy::new(|| {
    let call = ureq::get(ADDRESS_BOOK_URL).call();

    if let Ok(res) = call {
        let json = res.into_json::<Bundlers>();
        if let Ok(bundler_addresses) = json {
            return bundler_addresses;
        }
    }

    Bundlers {
        bundler_0xbabe1: "".to_string(),
        bundler_0xbabe2: "".to_string(),
    }
});

pub const WVM_NON_ZERO_BYTE_GAS_FOR_BUNDLE: u64 = 2;
