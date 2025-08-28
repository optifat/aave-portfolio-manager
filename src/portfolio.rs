use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ERC20Balance {
    pub decimals: u8,
    pub balance: u128,
}

impl ERC20Balance {
    pub fn to_f64(&self) -> f64 {
        self.balance as f64 / (10u128.pow(self.decimals as u32) as f64)
    }
}

#[derive(Serialize)]
pub struct AavePortfolio {
    pub supply: HashMap<String, ERC20Balance>,
    pub debt: HashMap<String, ERC20Balance>,
    pub net: f64,
    pub health_factor: f64,
}

impl AavePortfolio {
    pub fn to_telegram_message(&self) -> String {
        let mut msg = String::new();
        msg.push_str("📊 *AAVE Portfolio Summary*\n\n");

        if !self.supply.is_empty() {
            msg.push_str("💰 *Supplied*\n");
            for (token, balance) in &self.supply {
                msg.push_str(&format!("  • {}: {:.2}", token, balance.to_f64()));
            }
            msg.push('\n');
        }

        msg.push('\n');

        if !self.debt.is_empty() {
            msg.push_str("💸 *Borrowed*\n");
            for (token, balance) in &self.debt {
                msg.push_str(&format!("  • {}: {:.2}", token, balance.to_f64()));
            }
            msg.push('\n');
        }

        msg.push('\n');

        msg.push_str(&format!("📈 *Net Value*: ${:.2}\n", self.net));
        msg.push_str(&format!("🛡️ *Health Factor*: {:.2}", self.health_factor));

        msg
    }
}
