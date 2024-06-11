use anchor_lang::prelude::*;
use anchor_spl::{ token::{ transfer, Mint, Token, TokenAccount, Transfer } };
use crate::state::*;
use crate::errors;

#[derive(Accounts)]
pub struct BurnBadge<'info> {
    #[account(mut)]
    pub badge: Box<Account<'info, Badge>>,

    #[account(
        mut,
        seeds = [BADGE_SEED, badge.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_badge_account: Box<Account<'info, BadgeAccount>>,

    #[account(
        mut,
        seeds = [TREASURY_VAULT_SEED],
        bump,
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [BADGE_VAULT_SEED, badge.key().as_ref()],
        bump,
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
    pub system_program: Program<'info, System>,
}

impl<'info> BurnBadge<'info> {
    pub fn handler(&mut self, amount: u64, badge_vault_bump: u8) -> Result<()> {
        self.user_badge_account.burn(amount)?;

        let summation = get_price(self.badge.supply - amount, amount, self.badge.decimals);

        // transfer tokens from badge vault to user's wallet
        let badge_key = self.badge.key();
        let signer: &[&[&[u8]]] = &[&[BADGE_VAULT_SEED, badge_key.as_ref(), &[badge_vault_bump]]];

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from: self.badge_vault.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.badge_vault.to_account_info(),
            },
            signer
        );

        transfer(cpi_context, summation)?;
        self.badge.burn_badge(amount)?;

        Ok(())
    }
}
