use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("This amount is not enough.")]
    AmountNotEnough,

    #[msg("This stake is already running.")]
    AlreadyInitialized,

    #[msg("This stake is already claimed.")]
    AlreadyClaimed,

    #[msg("This stake is still locked.")]
    Locked,

    #[msg("Invalid mint.")]
    InvalidMint,

    #[msg("Invalid admin.")]
    InvalidAdmin,

    #[msg("Invalid token account.")]
    InvalidTokenAccount,

    #[msg("Insufficient Supply.")]
    InsufficientSupply,

    #[msg("Insufficient Funds.")]
    InsufficientFunds,
}
