# AMM - Automated Market Maker on Solana

A powerful, composable, and extensible Automated Market Maker (AMM) program for the Solana blockchain, written in Rust using [Anchor](https://github.com/coral-xyz/anchor). This repo also provides TypeScript integration tests for localnet and deployment scripts.

---


## Overview

This project implements a **constant product AMM** similar to Uniswap, enabling permissionless trading and liquidity provision of SPL tokens on Solana. The AMM manages a liquidity pool for two tokens, allowing users to swap tokens, add/remove liquidity, and earn LP tokens. All state and operations are handled securely via Solana PDAs and SPL token program.

---

## Features

- **Pool Initialization:** Create a new AMM pool for two SPL tokens with configurable fee.
- **Liquidity Provision:** Deposit tokens into the pool and receive LP tokens representing your share.
- **Token Swap:** Swap between the two pool tokens at the current market rate.
- **Liquidity Withdrawal:** Burn LP tokens to withdraw your underlying assets from the pool.
- **Configurable Fee:** Set trading fee during pool creation.
- **Authority Option:** Optionally restrict pool updates to an authority.

---

## Architecture

### Smart Contract: Rust + Anchor

- **Entry Point:** `programs/amm/src/lib.rs`
- **Instructions:**
  - `initialize`: Create pool, vaults, config, LP mint.
  - `deposit`: Add liquidity and mint LP tokens.
  - `swap`: Swap token X for Y or Y for X.
  - `withdraw`: Burn LP tokens and withdraw liquidity.
- **State:**
  - `Config` account stores pool parameters, mints, fee, authority, and PDAs.
  - Vault accounts hold pool tokens, owned by the config PDA.
  - LP mint PDA issues LP tokens for liquidity providers.

### TypeScript Integration Tests

- **Test File:** `tests/amm.ts`
- **Actions:**
  - Sets up user accounts and mints tokens.
  - Runs the full workflow: initialize, deposit, swap, withdraw.
  - Asserts balances and prints transaction logs.

---

## Program Flow

1. **Initialize Pool**
    - Create config PDA for pool state.
    - Create vaults for both tokens.
    - Create LP mint for liquidity tokens.
    - Set fee and optional authority.

2. **Deposit Liquidity**
    - Provider deposits both tokens in the required ratio.
    - Provider receives LP tokens representing pool share.

3. **Swap Tokens**
    - Trader deposits either token X or Y.
    - Trader receives the other token, minus trading fee.

4. **Withdraw Liquidity**
    - Provider burns LP tokens.
    - Provider receives underlying assets from vault.

---

## Directory Structure

```
Amm/
├── programs/
│   └── amm/
│       ├── src/
│       │   ├── lib.rs           # Main entrypoint
│       │   ├── instructions/    # Instruction handlers (initialize, deposit, swap, withdraw)
│       │   ├── state/           # On-chain state definition (Config)
│       │   ├── error.rs         # Custom error definitions
│       │   ├── constants.rs     # Program constants
│       └── Cargo.toml
├── tests/
│   └── amm.ts                   # TypeScript integration tests
├── migrations/
│   └── deploy.ts                # Anchor deployment script
├── Anchor.toml                  # Anchor config
└── README.md                    # This file
```

---

## Getting Started

### Prerequisites

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
- [Node.js](https://nodejs.org/)
- [Rust + Cargo](https://www.rust-lang.org/)

### Installation

Clone the repo and install dependencies:

```bash
git clone https://github.com/Swarnim-Chandve/Amm.git
cd Amm
anchor install
npm install
```

### Build Program

```bash
anchor build
```

---

## Testing

Run localnet tests with Mocha using TypeScript:

```bash
anchor test
```

### Test Workflow

- Sets up user accounts and airdrops SOL.
- Mints SPL tokens for pool creation.
- Initializes new AMM pool.
- Deposits liquidity and receives LP tokens.
- Swaps tokens in pool.
- Withdraws liquidity and burns LP tokens.
- Prints transaction signatures and checks balances.

```plaintext
token_x_vault balance should be 1000000
token_y_vault balance should be 1000000
user_lp balance should be 2000000
Your transaction signature <SIG>
```

---

## Deployment

Edit `Anchor.toml` for cluster & wallet configuration.

Deploy program to cluster:

```bash
anchor deploy
```

You can customize deployment logic in `migrations/deploy.ts`.

---

## API Reference

### Smart Contract Instructions

#### `initialize(ctx, seed, authority, fee)`

- **Purpose:** Create new AMM pool and config.
- **Accounts:**
  - Initializer (Signer)
  - Token mints (X, Y)
  - Config PDA (seeded by "config" and seed)
  - LP mint PDA (seeded by "lp" and config PDA)
  - Vault token accounts (for X and Y)
  - Token program, associated token program, system program

#### `deposit(ctx, asking_lp_amount, max_x, max_y)`

- **Purpose:** Provide liquidity, mint LP tokens.
- **Accounts:**
  - User (Signer)
  - Config, LP mint, vaults, user's token accounts, user's LP account

#### `swap(ctx, is_deposit_token_x, deposit_amount)`

- **Purpose:** Swap between tokens in pool.
- **Accounts:**
  - Trader (Signer)
  - Config, vaults, trader's token accounts

#### `withdraw(ctx, lp_token_amount)`

- **Purpose:** Burn LP tokens and withdraw assets.
- **Accounts:**
  - User (Signer)
  - Config, LP mint, vaults, user's token accounts, user's LP account

### On-Chain State: `Config`

```rust
pub struct Config {
    pub seed: u64,                 // Pool unique seed
    pub authority: Option<Pubkey>, // Optional authority
    pub token_mint_x: Pubkey,      // Token X mint
    pub token_mint_y: Pubkey,      // Token Y mint
    pub fee: u16,                  // Trading fee basis points
    pub locked: bool,              // Pool lock status
    pub config_bump: u8,           // PDA bump for config
    pub lp_bump: u8                // PDA bump for LP mint
}
```

### Error Handling

Custom errors are defined in `error.rs`, covering common AMM failures:
- Pool locked
- Slippage exceeded
- Invalid token or amount
- Curve math errors (overflow/underflow)
- Invalid authority

---




## Contributing

- Fork, clone and PRs are welcome!
- Open issues for bugs or feature requests.
- Please add test coverage for new features.

---

