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
[logging]
level = "info"

[aave_portfolio_tracker]
health_factor_notification_lower_limit = 1.1
health_factor_notification_upper_limit = 1.5
cron_schedule = "0 */5 * * * *"
```

| Key | Description |
|-----|-------------|
| `[logging].level` | Log verbosity level. Options: `trace`, `debug`, `info`, `warn`, `error`. |
| `[aave_portfolio_tracker].health_factor_notification_lower_limit` | Threshold below which the bot will notify the user. AAVE positions below this health factor are considered at risk. **If health factor drops below 1.0, the position becomes eligible for liquidation.** |
| `[aave_portfolio_tracker].health_factor_notification_upper_limit` | Threshold above which the bot will notify the user. Useful for profit tracking |
| `[aave_portfolio_tracker].cron_schedule` | Cron expression for how often the AAVE portfolio should be checked. The example runs every 5 minutes. |

### 3. Telegram Commands

The Telegram bot supports the following commands:

```
/help — Display this text.
/portfolio — Get current portfolio.
```

The command list will be expanded in future versions to include more detailed portfolio insights, configuration management, and alert customization.

---

## Running the Service

> Make sure your `.env` file is in the current working directory and contains all required variables (see [Environment Setup](#1-environment-setup)).

### Locally (via Cargo)

```bash
cargo run --release
```

This will launch the service with the default configuration found in `Config.toml` and the environment from `.env`.

### Docker

You can run the Aave Portfolio Tracker as a Docker container.

#### Build the Image

```bash
docker build -t aave-portfolio-tracker .
```

#### Run the Container

```bash
docker run -d --name aave-portfolio-tracker --env-file .env aave-portfolio-tracker
```

#### Start an Existing (Previously Stopped) Container

```bash
docker start aave-portfolio-tracker
```

#### Stop the Container

```bash
docker stop aave-portfolio-tracker
```

#### Remove the Container

```bash
docker rm aave-portfolio-tracker
```
---

## Roadmap

- ✅ Telegram notifications on position health factor
- ✅ Add command support to the Telegram bot
  - Extend supported commands list
- ⏳ Broaden notification types (e.g. liquidation alerts, rate changes)
- ⏳ Enable automated portfolio actions without compromising user fund custody:
  - Use a custom smart contract where this service acts as a `Manager` with limited permissions (e.g. no fund withdrawal)
  - Support for submitting pre-signed user transactions
- ✅ Automatic token discovery for ERC20, aTokens, and debt tokens
- ⏳ Add EVM L2 chains support

---

## License

This project is licensed under the terms of the [GNU General Public License v3.0](./LICENSE).

You are free to use, modify, and distribute the software under the same license, provided that all derivative works also remain open-source under GPL-3.0. Contributions and forks are welcome, but please preserve authorship and license information.

---
