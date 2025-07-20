#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6gu1wQAnJ7jF2anuPMCW1SBn6JH9DjxxxTnfaRiHn5sU");

#[program]
pub mod amm {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, seed: u64, authority: Option<Pubkey>, fee: u16) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, asking_lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(asking_lp_amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_deposit_token_x: bool, deposit_amount: u64) -> Result<()> {
        ctx.accounts.swap(is_deposit_token_x, deposit_amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_token_amount: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_token_amount)
    }
}