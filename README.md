# Aave Portfolio Manager

Rust-based services for managing positions on the AAVE v3 protocol, including a Telegram bot for real-time notifications.

## Overview

This project monitors a specified Ethereum address's AAVE v3 portfolio and sends Telegram notifications when the health factor drops below a predefined threshold.

## Features

- Real-time tracking of AAVE v3 positions
- Telegram bot integration for alerting users
- Customizable notification thresholds
- Configurable via `.env` and TOML config file

> **Note:** The current implementation requires a dedicated Telegram bot instance **per user**.

---

## Getting Started

### 1. Environment Setup

Create a `.env` file based on the [`.env.example`](./.env.example):

| Variable       | Description |
|----------------|-------------|
| `BOT_TOKEN`    | Your Telegram bot token ([get one via BotFather](https://core.telegram.org/bots/features#botfather)) |
| `TG_USER_ID`   | Your Telegram user ID (can be obtained using third-party tools like [`@userinfobot`](https://t.me/userinfobot)) |
| `NODE_URI`     | Ethereum node URI for on-chain data access |
| `ETH_ADDRESS`  | Ethereum wallet address to monitor |

### 2. Config File (`Config.toml`)

Configure runtime options in the [`Config.toml`](./Config.toml) file. Below is an example and explanation of available fields:

```toml
cron_schedule = "0 */5 * * * *"

[logging]
level = "info"

[aave_portfolio_tracker]
health_factor_notification_limit = 1.1
```

| Key | Description |
|-----|-------------|
| `cron_schedule` | Cron expression for how often the AAVE portfolio should be checked. The example runs every 5 minutes. Use [Quartz cron generators](https://www.freeformatter.com/cron-expression-generator-quartz.html) to create expressions compatible with the scheduler. |
| `[logging].level` | Log verbosity level. Options: `trace`, `debug`, `info`, `warn`, `error`. |
| `[aave_portfolio_tracker].health_factor_notification_limit` | Threshold below which the bot will notify the user. AAVE positions below this health factor are considered at risk. **If health factor drops below 1.0, the position becomes eligible for liquidation.** |

### 3. Token Repository

Ensure the [token registry file](./src/portfolio_data/erc20_registry.rs) includes the relevant `aToken`s and `variableDebtToken`s.  
> *Note: This list is currently hardcoded and tailored to a specific use case. Automatic token discovery is planned.*

---

## Running the Service

### Locally (via Cargo)

```bash
cargo run --release
```

### Docker

Docker support is planned for a future release.

---

## Roadmap

- ✅ Telegram notifications on position health factor
- ⏳ Add command support to the Telegram bot
- ⏳ Broaden notification types (e.g. liquidation alerts, rate changes)
- ⏳ Enable automated portfolio actions without compromising user fund custody:
  - Use a custom smart contract where this service acts as a `Manager` with limited permissions (e.g. no fund withdrawal)
  - Support for submitting pre-signed user transactions
- ⏳ Automatic token discovery for ERC20, aTokens, and debt tokens

---

## License

This project is licensed under the terms of the [GNU General Public License v3.0](./LICENSE).

You are free to use, modify, and distribute the software under the same license, provided that all derivative works also remain open-source under GPL-3.0. Contributions and forks are welcome, but please preserve authorship and license information.

---
