use anchor_lang::prelude::*;

#[account]
pub struct NFTSubmission {
    pub bounty_id: Pubkey,
    pub designer: Pubkey,
    pub nft_mint: Pubkey,
    pub status: String,
}

impl NFTSubmission {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 10;
}
