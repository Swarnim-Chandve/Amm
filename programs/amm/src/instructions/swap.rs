use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer, transfer}
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,
    pub token_mint_x: Account<'info, Mint>,
    pub token_mint_y: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint_x,
        associated_token::authority = config
    )]
    pub token_x_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_y,
        associated_token::authority = config
    )]
    pub token_y_vault: Account<'info, TokenAccount>,
    #[account(
        has_one = token_mint_x,
        has_one = token_mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = trader,
        associated_token::mint = token_mint_x,
        associated_token::authority = trader
    )]
    pub trader_token_x: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = trader,
        associated_token::mint = token_mint_y,
        associated_token::authority = trader
    )]
    pub trader_token_y: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&self, is_deposit_token_x: bool, deposit_amount: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(deposit_amount > 0, AmmError::InvalidAmount);

        let withdraw_amount = match is_deposit_token_x {
            true => {
                ConstantProduct::delta_y_from_x_swap_amount(self.token_x_vault.amount, self.token_y_vault.amount, deposit_amount).unwrap()
            }
            false => {
                ConstantProduct::delta_x_from_y_swap_amount(self.token_x_vault.amount, self.token_y_vault.amount, deposit_amount).unwrap()
            }
        };

        self.deposit_token(is_deposit_token_x, deposit_amount)?;
        self.withdraw_token(!is_deposit_token_x, withdraw_amount)
    }

    pub fn deposit_token(&self, is_deposit_token_x: bool, deposit_amount: u64) -> Result<()> {
        let (from, to) = match is_deposit_token_x {
            true => (self.trader_token_x.to_account_info(), self.token_x_vault.to_account_info()),
            false => (self.trader_token_y.to_account_info(), self.token_y_vault.to_account_info())
        };

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.trader.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, deposit_amount)
    }

    pub fn withdraw_token(&self, is_withdraw_token_x: bool, withdraw_amount: u64) -> Result<()> {
        let (from, to) = match is_withdraw_token_x {
            true => (self.token_x_vault.to_account_info(), self.trader_token_x.to_account_info()),
            false => (self.token_y_vault.to_account_info(), self.trader_token_y.to_account_info())
        };

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info()
        };

        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &signer_seeds);

        transfer(cpi_ctx, withdraw_amount)
    }
}