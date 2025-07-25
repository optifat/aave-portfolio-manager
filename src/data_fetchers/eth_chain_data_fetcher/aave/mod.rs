use ethers::types::{Address, U256};
use serde::Serialize;

mod pool;
mod pool_addresses_provider;
mod ui_pool_data_provider;

pub use pool::get_user_aave_data;
pub use ui_pool_data_provider::get_user_reserves;

#[derive(Debug, Serialize)]
pub struct AaveUserData {
    pub total_collateral_base: U256,
    pub total_debt_base: U256,
    pub available_borrows_base: U256,
    pub current_liquidation_threshold: U256,
    pub ltv: U256,
    pub health_factor: U256,
}

#[derive(Debug, Serialize)]
pub struct UserAaveTokens {
    pub collateral: Vec<Address>,
    pub debt: Vec<Address>,
}
