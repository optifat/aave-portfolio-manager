use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        function balanceOf(address owner) external view returns (uint256)
    ]"#,
);

pub async fn get_token_balance(
    provider: &Arc<Provider<Http>>,
    token: Address,
    wallet: Address,
) -> anyhow::Result<U256> {
    let token = IERC20::new(token, provider.clone());
    Ok(token.balance_of(wallet).call().await?)
}
