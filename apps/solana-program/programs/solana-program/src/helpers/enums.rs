use anchor_lang::prelude::*;


#[error_code]
pub enum ErrorCode {
    #[msg("You are not the creator of this bounty.")]
    NotBountyCreator,

    #[msg("Insufficient funds in escrow.")]
    InsufficientFunds,
}
