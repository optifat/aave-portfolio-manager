use ethers::prelude::*;
use std::sync::Arc;

use crate::common_data::aave_pool_addresses_provider;

abigen!(
    IAavePoolAddressesProvider,
    r#"[
        function getPool() external view returns (address)
    ]"#,
);

pub(super) async fn get_aave_pool(provider: Arc<Provider<Http>>) -> anyhow::Result<Address> {
    let contract =
        IAavePoolAddressesProvider::new(aave_pool_addresses_provider(), provider.clone());
    Ok(contract.get_pool().await?)
}
