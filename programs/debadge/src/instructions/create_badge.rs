use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::{ token::{ transfer, Mint, Token, TokenAccount, Transfer } };
use crate::state::*;
use crate::errors;

#[derive(Accounts)]
#[instruction(_supply: u64, _decimals: u8, name: String, _symbol: String)]
pub struct CreateBadge<'info> {
    #[account(
        mut,
        seeds = [PLATFORM_INFO_SEED],
        bump,
    )]
    pub platform_info: Box<Account<'info, PlatformInfo>>,

    #[account(
        mut,
        seeds = [TREASURY_VAULT_SEED],
        bump,
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [BADGE_SEED, user.key().as_ref(), name.as_bytes()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<Badge>()
    )]
    pub badge: Box<Account<'info, Badge>>,

    #[account(
        init,
        seeds = [BADGE_VAULT_SEED, badge.key().as_ref()],
        bump,
        payer = user,
        token::mint = fee_token_mint,
        token::authority = badge_vault
    )]
    pub badge_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = fee_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub fee_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateBadge<'info> {
    pub fn handler(
        &mut self,
        max_supply: u64,
        decimals: u8,
        name: String,
        symbol: String
    ) -> Result<()> {
        self.badge.create_badge(self.user.key(), max_supply, decimals, name, symbol)?;
        self.platform_info.badge_count += 1;

        // Transfer the tokens from the user's wallet to fee vault
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.treasury_vault.to_account_info(),
            authority: self.user.to_account_info(),
        });

        transfer(cpi_context, BADGE_CREATION_FEE)?;

        Ok(())
    }
}
