use anchor_lang::prelude::*;
use anchor_spl::{ token::{ transfer, Mint, Token, TokenAccount, Transfer } };
use crate::state::*;
use crate::errors;

#[derive(Accounts)]
pub struct MintBadge<'info> {
    #[account(mut)]
    pub badge: Box<Account<'info, Badge>>,

    #[account(
        init_if_needed,
        seeds = [BADGE_SEED, badge.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<BadgeAccount>()
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

impl<'info> MintBadge<'info> {
    pub fn handler(&mut self, amount: u64) -> Result<()> {
        self.user_badge_account.mint(amount)?;

        let summation = get_price(self.badge.supply, amount, self.badge.decimals);

        msg!("Total Cost: {}", summation / 100000);

        // Transfer the tokens from the user's wallet to the badge vault
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.badge_vault.to_account_info(),
            authority: self.user.to_account_info(),
        });

        transfer(cpi_context, summation)?;

        self.badge.mint_badge(amount)?;

        Ok(())
    }
}
