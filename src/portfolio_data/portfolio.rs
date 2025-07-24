use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AavePortfolio {
    pub supply: HashMap<String, u128>,
    pub debt: HashMap<String, u128>,
    pub net: f64,
    pub health_factor: f64,
}
