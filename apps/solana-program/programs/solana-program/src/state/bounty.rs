use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum BountyStatus {
    Active,
    Completed,
    Expired,
}


#[account]
pub struct Bounty {
    pub game_name: String,
    pub bounty_name: String,
    pub creator: Pubkey,
    pub asset_type: String,
    pub description: String,
    pub deadline: i64,
    pub bounty_amount: u64,
    pub escrow: Pubkey,
    pub status: String,
}

impl Bounty {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 4 + 200 + 8 + 8 + 32 + 10;
}
