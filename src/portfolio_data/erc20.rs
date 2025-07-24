use ethers::types::Address;

#[derive(Eq, Hash, PartialEq)]
pub struct Erc20 {
    pub symbol: String,
    pub decimals: u8,
    pub address: Address,
}

impl Erc20 {
    pub(super) fn new(symbol: &str, decimals: u8, address: &str) -> Self {
        let address_formatted = address
            .parse()
            .expect(format!("Failed to parse {} address", symbol).as_str());

        Self {
            symbol: symbol.to_string(),
            decimals,
            address: address_formatted,
        }
    }
}
