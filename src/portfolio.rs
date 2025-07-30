use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AavePortfolio {
    pub supply: HashMap<String, u128>,
    pub debt: HashMap<String, u128>,
    pub net: f64,
    pub health_factor: f64,
}

impl AavePortfolio {
    pub fn to_telegram_message(&self) -> String {
        let mut msg = String::new();
        msg.push_str("📊 *AAVE Portfolio Summary*\n\n");

        if !self.supply.is_empty() {
            msg.push_str("💰 *Supplied*\n");
            for (token, amount) in &self.supply {
                msg.push_str(&format!("  • {}: {:.2}", token, amount));
            }
            msg.push('\n');
        }

        msg.push('\n');

        if !self.debt.is_empty() {
            msg.push_str("💸 *Borrowed*\n");
            for (token, amount) in &self.debt {
                msg.push_str(&format!("  • {}: {:.2}", token, amount));
            }
            msg.push('\n');
        }

        msg.push('\n');

        msg.push_str(&format!("📈 *Net Value*: ${:.2}\n", self.net));
        msg.push_str(&format!("🛡️ *Health Factor*: {:.2}", self.health_factor));

        msg
    }
}
