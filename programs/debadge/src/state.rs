use anchor_lang::prelude::*;
use solana_program::pubkey;
use crate::errors;

pub const ADMIN_PUBKEY: Pubkey = pubkey!("2g9K42Pt5y58cejTHFLhqoQWKDUcB3s3AnGESmV9ySBW");
pub const TOKEN_MINT_PUBKEY: Pubkey = pubkey!("44WAtPrJrbh9a7cw2PesBAvntf5xiqgKtmoBLT4CjMVL");
// pub const TREASURY_VAULT_PUBKEY: Pubkey = pubkey!("AWeKy64ajVHemQWfpeMxmb3sK3mu1V6eV4PhBjdhNE4E");

pub const USER_BADGE_ACCOUNT_SEED: &[u8] = b"user_badge_account";
pub const TREASURY_VAULT_SEED: &[u8] = b"treasury_vault";
pub const PLATFORM_INFO_SEED: &[u8] = b"platform_info";
pub const BADGE_SEED: &[u8] = b"badge";
pub const BADGE_VAULT_SEED: &[u8] = b"badge_vault";
pub const BADGE_CREATION_FEE: u64 = 5000;

#[account]
pub struct PlatformInfo {
    pub badge_count: u64,
    pub is_initialized: bool,
}

#[account]
pub struct Badge {
    pub owner: Pubkey,
    pub supply: u64,
    pub max_supply: u64,
    pub price: u64,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
}

#[account]
pub struct BadgeAccount {
    pub badge: Pubkey,
    pub owner: Pubkey,
    pub balance: u64,
}

impl Badge {
    pub fn create_badge(
        &mut self,
        owner: Pubkey,
        max_supply: u64,
        decimals: u8,
        name: String,
        symbol: String
    ) -> Result<()> {
        self.owner = owner;
        self.supply = 1 * (10_u64).pow(decimals as u32);
        self.max_supply = max_supply;
        self.price = get_price(1, 1, decimals);
        self.decimals = decimals;
        self.name = name;
        self.symbol = symbol;

        Ok(())
    }

    pub fn mint_badge(&mut self, amount: u64) -> Result<()> {
        self.supply += amount;
        self.price = get_price(self.supply, 1, self.decimals);

        Ok(())
    }

    pub fn burn_badge(&mut self, amount: u64) -> Result<()> {
        self.supply -= amount;
        self.price = get_price(self.supply, 1, self.decimals);

        Ok(())
    }
}

impl BadgeAccount {
    pub fn create_badge_account(&mut self, badge: Pubkey, owner: Pubkey) -> Result<()> {
        self.badge = badge;
        self.owner = owner;
        self.balance = 0;

        Ok(())
    }

    pub fn mint(&mut self, amount: u64) -> Result<()> {
        self.balance += amount;

        Ok(())
    }

    pub fn burn(&mut self, amount: u64) -> Result<()> {
        if self.balance < amount {
            return Err(errors::ErrorCode::InsufficientFunds.into());
        }

        self.balance -= amount;

        Ok(())
    }

    pub fn transfer(&mut self, amount: u64, mut recipient: BadgeAccount) -> Result<()> {
        if self.balance < amount {
            return Err(errors::ErrorCode::InsufficientFunds.into());
        }

        self.balance -= amount;
        recipient.balance += amount;

        Ok(())
    }
}

pub fn get_price(supply: u64, amount: u64, decimals: u8) -> u64 {
    let diviser = (10_u64).pow(decimals as u32);
    let amount = amount / diviser + 1;

    let sum_one = |n: u64| {
        if n > 1 { ((n - 1) * n * (2 * (n - 1) + 1)) / 6 } else { 0 }
    };

    let sum_two = |n: u64| {
        if n > 1 {
            ((n - 1 + amount) * (n + amount) * (2 * (n - 1 + amount) + 1)) / 6
        } else {
            ((amount - 1) * amount * (2 * (amount - 1) + 1)) / 6
        }
    };

    return sum_two(supply / diviser) - sum_one(supply / diviser);
}
