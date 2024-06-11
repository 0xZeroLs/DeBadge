use anchor_lang::prelude::*;
use anchor_spl::{ token::{ Mint, Token, TokenAccount } };
use crate::state::*;
use crate::errors;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [PLATFORM_INFO_SEED],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<PlatformInfo>()
    )]
    pub platform_info: Box<Account<'info, PlatformInfo>>,

    #[account(
        init,
        seeds = [TREASURY_VAULT_SEED],
        bump,
        payer = user,
        token::mint = fee_token_mint,
        token::authority = treasury_vault
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub fee_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self) -> Result<()> {
        self.platform_info.is_initialized = true;
        self.platform_info.badge_count = 0;

        if self.user.key() != ADMIN_PUBKEY {
            return Err(errors::ErrorCode::InvalidAdmin.into());
        }

        Ok(())
    }
}
