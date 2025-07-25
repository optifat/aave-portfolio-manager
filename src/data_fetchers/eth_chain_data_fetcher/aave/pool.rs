use ethers::prelude::*;
use std::sync::Arc;

use super::{AaveUserData, pool_addresses_provider::get_aave_pool};

abigen!(
    IAavePool,
    r#"[
        function getUserAccountData(address user) external view returns (uint256 totalCollateralBase, uint256 totalDebtBase, uint256 availableBorrowsBase, uint256 currentLiquidationThreshold, uint256 ltv, uint256 healthFactor)
    ]"#,
);

pub async fn get_user_aave_data(
    provider: Arc<Provider<Http>>,
    user: Address,
) -> anyhow::Result<AaveUserData> {
    let aave_pool_address = get_aave_pool(provider.clone()).await?;
    let aave_pool = IAavePool::new(aave_pool_address, provider);

    let data: (U256, U256, U256, U256, U256, U256) = aave_pool.get_user_account_data(user).await?;
    Ok(AaveUserData {
        total_collateral_base: data.0,
        total_debt_base: data.1,
        available_borrows_base: data.2,
        current_liquidation_threshold: data.3,
        ltv: data.4,
        health_factor: data.5,
    })
}
