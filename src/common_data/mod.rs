use ethers::types::Address;

// https://etherscan.io/address/0x54586bE62E3c3580375aE3723C145253060Ca0C2#readContract#F3
pub const AAVE_ORACLE_BASE_UNIT: u128 = 100_000_000;

pub const AAVE_WAD: u128 = u128::pow(10, 18);

pub fn aave_pool_addresses_provider() -> Address {
    "0x2f39d218133AFaB8F2B819B1066c7E434Ad94E9e"
        .parse()
        .unwrap()
}

pub fn aave_ui_pool_data_provider() -> Address {
    "0x3F78BBD206e4D3c504Eb854232EdA7e47E9Fd8FC"
        .parse()
        .unwrap()
}
