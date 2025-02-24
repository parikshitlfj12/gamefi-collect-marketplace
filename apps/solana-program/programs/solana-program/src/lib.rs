pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::create_bounty::*;

declare_id!("J2j5zx55PvnChm8haN87WZuoBBKME5txpStvNtSGyhgN");

#[program]
pub mod bounty_program {
    use super::*;

    pub fn initialize(
        ctx: Context<CreateBounty>,
        game_name: String,
        bounty_name: String,
        asset_type: String,
        description: String,
        deadline: i64,
        bounty_amount: u64,
    ) -> Result<()> {
        create_bounty(
            ctx,
            game_name,
            bounty_name,
            asset_type,
            description,
            deadline,
            bounty_amount,
        )
    }
}
