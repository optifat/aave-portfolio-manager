use ethers::prelude::*;
use std::{collections::HashMap, sync::Arc, u128};

use crate::portfolio::ERC20Balance;

abigen!(
    IERC20,
    r#"[
        function balanceOf(address owner) external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string memory)
    ]"#,
);

pub(super) async fn get_erc20_user_data(
    provider: Arc<Provider<Http>>,
    user: Address,
    token_addresses: &Vec<Address>,
    multicall_address: Option<Address>,
) -> anyhow::Result<HashMap<String, ERC20Balance>> {
    let tokens: Vec<IERC20<Provider<Http>>> = token_addresses
        .iter()
        .map(|token| IERC20::new(token.clone(), provider.clone()))
        .collect();

    let user_balances: Vec<u128> =
        Multicall::<Provider<Http>>::new(provider.clone(), multicall_address)
            .await?
            .version(MulticallVersion::Multicall3)
            .add_calls(false, tokens.iter().map(|token| token.balance_of(user)))
            .call::<Vec<U256>>()
            .await?
            .iter()
            .map(|balance| {
                if balance.gt(&u128::MAX.into()) {
                    anyhow::bail!("{} balance parsing overflow", balance);
                }
                Ok(balance.as_u128())
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

    let token_decimals: Vec<u8> =
        Multicall::<Provider<Http>>::new(provider.clone(), multicall_address)
            .await?
            .version(MulticallVersion::Multicall3)
            .add_calls(false, tokens.iter().map(|token| token.decimals()))
            .call()
            .await?;

    let token_symbols: Vec<String> =
        Multicall::<Provider<Http>>::new(provider.clone(), multicall_address)
            .await?
            .version(MulticallVersion::Multicall3)
            .add_calls(false, tokens.iter().map(|token| token.symbol()))
            .call()
            .await?;

    let erc20_data = user_balances
        .into_iter()
        .zip(token_decimals)
        .zip(token_symbols)
        .map(|((balance, decimals), symbol)| (symbol, ERC20Balance { decimals, balance }))
        .collect();

    Ok(erc20_data)
}
