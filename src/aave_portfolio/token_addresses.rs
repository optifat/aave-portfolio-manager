use ethers::abi::Address;
use std::collections::HashMap;
use std::sync::OnceLock;

pub(super) fn a_token_set() -> &'static HashMap<&'static str, Address> {
    static SET: OnceLock<HashMap<&'static str, Address>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashMap::new();
        set.insert(
            "Usdt",
            "0x23878914EFE38d27C4D67Ab83ed1b93A74D4086a"
                .parse()
                .expect("Failed to parse aEthUsdt address"),
        );
        set
    })
}

pub(super) fn variable_debt_token_set() -> &'static HashMap<&'static str, Address> {
    static SET: OnceLock<HashMap<&'static str, Address>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashMap::new();
        set.insert(
            "WBTC",
            "0x40aabef1aa8f0eec637e0e7d92fbffb2f26a8b7b"
                .parse()
                .expect("Failed to parse variableDebtEthWBTC address"),
        );
        set.insert(
            "USDe",
            "0x015396e1f286289ae23a762088e863b3ec465145"
                .parse()
                .expect("Failed to parse variableDebtEthUSDe address"),
        );
        set
    })
}
