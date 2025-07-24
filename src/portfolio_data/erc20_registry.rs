use std::collections::HashSet;
use std::sync::OnceLock;

use super::erc20::Erc20;

pub fn a_token_set() -> &'static HashSet<Erc20> {
    static SET: OnceLock<HashSet<Erc20>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert(Erc20::new(
            "USDT",
            6,
            "0x23878914EFE38d27C4D67Ab83ed1b93A74D4086a",
        ));
        set
    })
}

pub fn variable_debt_token_set() -> &'static HashSet<Erc20> {
    static SET: OnceLock<HashSet<Erc20>> = OnceLock::new();
    SET.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert(Erc20::new(
            "WBTC",
            8,
            "0x40aabef1aa8f0eec637e0e7d92fbffb2f26a8b7b",
        ));
        set.insert(Erc20::new(
            "USDe",
            8,
            "0x015396e1f286289ae23a762088e863b3ec465145",
        ));
        set
    })
}
