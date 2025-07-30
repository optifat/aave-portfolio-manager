use ethers::prelude::*;
use std::sync::Arc;

use crate::common_data::{aave_pool_addresses_provider, aave_ui_pool_data_provider};

abigen!(
    IUiPoolDataProvider,
    r#"[
        struct UserReserveData { address underlyingAsset; uint256 scaledATokenBalance; bool usageAsCollateralEnabledOnUser; uint256 scaledVariableDebt; }
        function getUserReservesData(address provider, address user) external view returns (UserReserveData[] memory, uint8)
    ]"#,
);

type UserReserveDataOutput = Vec<(Address, U256, bool, U256)>;

pub async fn get_user_reserves(
    provider: Arc<Provider<Http>>,
    user: Address,
) -> anyhow::Result<super::UserAaveTokens> {
    let pool_data_provider = IUiPoolDataProvider::new(aave_ui_pool_data_provider(), provider);

    let (all_reserves, _): (UserReserveDataOutput, u8) = pool_data_provider
        .get_user_reserves_data(aave_pool_addresses_provider(), user)
        .await?;

    let collateral = all_reserves
        .iter()
        .filter(|x| !x.1.is_zero() && x.2)
        .map(|x| x.0)
        .collect();

    let debt = all_reserves
        .iter()
        .filter(|x| !x.3.is_zero())
        .map(|x| x.0)
        .collect();

    Ok(super::UserAaveTokens { collateral, debt })
}
