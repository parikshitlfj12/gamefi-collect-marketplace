use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub bounty_id: Pubkey,
    pub balance: u64,
}

impl Escrow {
    pub const INIT_SPACE: usize = 32 + 8;
}
