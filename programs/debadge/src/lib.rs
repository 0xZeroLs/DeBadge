mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("9M3tQTBp8va7JgF9EUghTz5cbVVyug9TXvqwb8ewz37u");

#[program]
pub mod debadge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn create_badge(
        ctx: Context<CreateBadge>,
        max_supply: u64,
        decimals: u8,
        name: String,
        symbol: String
    ) -> Result<()> {
        ctx.accounts.handler(max_supply, decimals, name, symbol)
    }

    pub fn mint_badge(ctx: Context<MintBadge>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount)
    }

    pub fn burn_badge(ctx: Context<BurnBadge>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount, ctx.bumps.badge_vault)
    }
}
