use anchor_lang::prelude::*;

declare_id!("J2j5zx55PvnChm8haN87WZuoBBKME5txpStvNtSGyhgN");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
