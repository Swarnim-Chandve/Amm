use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{Transfer, transfer, Mint, Token, TokenAccount, Burn, burn}};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint_x: Account<'info, Mint>,
    pub token_mint_y: Account<'info, Mint>,
     #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub token_mint_lp: Account<'info, Mint>,
    #[account(
        has_one = token_mint_x,
        has_one = token_mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

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
        mut,
        associated_token::mint = token_mint_x,
        associated_token::authority = user
    )]
    pub user_token_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_y,
        associated_token::authority = user
    )]
    pub user_token_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, lp_token_amount: u64) -> Result<()> {

        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(lp_token_amount > 0, AmmError::InvalidAmount);

        let xy_amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.token_x_vault.amount, 
            self.token_y_vault.amount, 
            self.token_mint_lp.supply, 
            lp_token_amount, 
            6
        ).unwrap();

        self.burn_lp_token(lp_token_amount)?;
        self.withdraw_token(true, xy_amounts.x)?;
        self.withdraw_token(false, xy_amounts.y)
    }

    pub fn burn_lp_token(&self, lp_token_amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.token_mint_lp.to_account_info(),
            from: self.user_lp.to_account_info(),
            authority: self.user.to_account_info()
        };

        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &signer_seeds);

        burn(cpi_ctx, lp_token_amount)
    }

    pub fn withdraw_token(&self, is_x: bool, withdraw_amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.token_x_vault.to_account_info(), self.user_token_x.to_account_info()),
            false => (self.token_y_vault.to_account_info(), self.user_token_y.to_account_info())
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