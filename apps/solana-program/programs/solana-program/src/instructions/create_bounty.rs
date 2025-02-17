use crate::state::{Bounty, BountyStatus, Escrow};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

#[derive(Accounts)]
#[instruction(bounty_name: String)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // Game studio wallet

    #[account(
        init,
        payer = creator,
        space = 8 + Bounty::INIT_SPACE,
        seeds = [b"bounty", creator.key().as_ref(), bounty_name.as_bytes()],
        bump
    )]
    pub bounty: Account<'info, Bounty>, // Bounty PDA

    #[account(
        init,
        payer = creator,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", bounty.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>, // Escrow PDA to store funds

    pub system_program: Program<'info, System>,
}

pub fn create_bounty(
    ctx: Context<CreateBounty>,
    game_name: String,
    bounty_name: String,
    asset_type: String,
    description: String,
    deadline: i64,
    bounty_amount: u64,
) -> Result<()> {
    let bounty = &mut ctx.accounts.bounty;
    let escrow = &mut ctx.accounts.escrow;
    let creator = &ctx.accounts.creator;

    // Store bounty details
    bounty.game_name = game_name;
    bounty.bounty_name = bounty_name;
    bounty.creator = creator.key();
    bounty.asset_type = asset_type;
    bounty.description = description;
    bounty.deadline = deadline;
    bounty.bounty_amount = bounty_amount;
    bounty.escrow = escrow.key();
    bounty.status = BountyStatus::Active;

    // Store escrow details
    escrow.bounty_id = bounty.key();
    escrow.balance = bounty_amount;

    // Transfer bounty funds to escrow
    let transfer_result = anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: creator.to_account_info(),
                to: escrow.to_account_info(),
            },
        ),
        bounty_amount,
    );

    if transfer_result.is_err() {
        return Err(ProgramError::InsufficientFunds.into());
    }

    // Emit Event
    msg!(
        "Bounty Created: {} for Game: {}",
        bounty.bounty_name,
        bounty.game_name
    );

    Ok(())
}
