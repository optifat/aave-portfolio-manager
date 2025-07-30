use ethers::prelude::*;
use std::{sync::Arc, u128};

abigen!(
    IERC20,
    r#"[
        function balanceOf(address owner) external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
    ]"#,
);

pub(super) async fn get_token_balance(
    provider: Arc<Provider<Http>>,
    token: Address,
    wallet: Address,
) -> anyhow::Result<u128> {
    let token = IERC20::new(token, provider.clone());
    let balance_u256: U256 = token.balance_of(wallet).call().await?;
    if balance_u256.gt(&u128::MAX.into()) {
        anyhow::bail!("{} balance parsing overflow", balance_u256);
    }
    Ok(balance_u256.as_u128())
}

pub(super) async fn get_token_decimals(
    provider: Arc<Provider<Http>>,
    token: Address,
) -> anyhow::Result<u8> {
    let token = IERC20::new(token, provider.clone());
    Ok(token.decimals().await?)
}

pub(super) async fn get_token_symbol(
    provider: Arc<Provider<Http>>,
    token: Address,
) -> anyhow::Result<String> {
    let token = IERC20::new(token, provider.clone());
    Ok(token.symbol().await?)
}
