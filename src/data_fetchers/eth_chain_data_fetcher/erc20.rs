use ethers::prelude::*;
use std::{sync::Arc, u128};

abigen!(
    IERC20,
    r#"[
        function balanceOf(address owner) external view returns (uint256)
    ]"#,
);

pub(super) async fn get_token_balance(
    provider: &Arc<Provider<Http>>,
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
