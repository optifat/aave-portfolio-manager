use anyhow::Ok;
use ethers::prelude::*;
use std::sync::Arc;

use super::{AaveReserveData, AaveUserData, pool_addresses_provider::get_aave_pool};

abigen!(
    IAavePool,
    r#"[
        struct ReserveData { uint256 configuration; uint128 liquidityIndex; uint128 currentLiquidityRate; uint128 variableBorrowIndex; uint128 currentVariableBorrowRate; uint128 currentStableBorrowRate; uint40 lastUpdateTimestamp; uint16 id; address aTokenAddress; address stableDebtTokenAddress; address variableDebtTokenAddress; address interestRateStrategyAddress; uint128 accruedToTreasury; uint128 unbacked; uint128 isolationModeTotalDebt;}
        function getUserAccountData(address user) external view returns (uint256 totalCollateralBase, uint256 totalDebtBase, uint256 availableBorrowsBase, uint256 currentLiquidationThreshold, uint256 ltv, uint256 healthFactor)
        function getReserveData(address asset) external view returns (ReserveData memory)
    ]"#,
);

type ReservesCallOutput = (
    U256,
    u128,
    u128,
    u128,
    u128,
    u128,
    u64,
    u16,
    Address,
    Address,
    Address,
    Address,
    u128,
    u128,
    u128,
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

pub async fn get_aave_reserve_data(
    provider: Arc<Provider<Http>>,
    asset: Address,
) -> anyhow::Result<AaveReserveData> {
    let aave_pool_address = get_aave_pool(provider.clone()).await?;
    let aave_pool = IAavePool::new(aave_pool_address, provider);

    let reserve_data: ReserveData = aave_pool.get_reserve_data(asset).await?.into();
    Ok(AaveReserveData {
        a_token: reserve_data.a_token_address,
        variable_debt_token: reserve_data.variable_debt_token_address,
    })
}

impl From<ReservesCallOutput> for ReserveData {
    fn from(t: ReservesCallOutput) -> Self {
        Self {
            configuration: t.0,
            liquidity_index: t.1,
            current_liquidity_rate: t.2,
            variable_borrow_index: t.3,
            current_variable_borrow_rate: t.4,
            current_stable_borrow_rate: t.5,
            last_update_timestamp: t.6,
            id: t.7,
            a_token_address: t.8,
            stable_debt_token_address: t.9,
            variable_debt_token_address: t.10,
            interest_rate_strategy_address: t.11,
            accrued_to_treasury: t.12,
            unbacked: t.13,
            isolation_mode_total_debt: t.14,
        }
    }
}
