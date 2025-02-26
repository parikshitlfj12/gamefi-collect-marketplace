use crate::state::bounty::Bounty;
use crate::state::nft_submission::NFTSubmission;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SubmitNFT<'info> {
    #[account(mut)]
    pub designer: Signer<'info>, // NFT creator

    #[account(
        mut,
        seeds = [b"bounty", bounty.creator.as_ref(), bounty.bounty_name.as_bytes()],
        bump
    )]
    pub bounty: Account<'info, Bounty>, // Reference to the bounty

    #[account(
        init,
        payer = designer,
        space = 8 + NFTSubmission::INIT_SPACE,
        seeds = [b"submission", bounty.key().as_ref(), designer.key().as_ref()],
        bump
    )]
    pub submission: Account<'info, NFTSubmission>, // NFT submission record

    pub system_program: Program<'info, System>,
}

pub fn submit_nft(ctx: Context<SubmitNFT>, nft_mint: Pubkey) -> Result<()> {
    let submission = &mut ctx.accounts.submission;
    let designer = &ctx.accounts.designer;

    submission.bounty_id = ctx.accounts.bounty.key();
    submission.designer = designer.key();
    submission.nft_mint = nft_mint;
    submission.status = "Pending".to_string();

    msg!(
        "NFT Submitted for Bounty {} by {}",
        submission.bounty_id,
        submission.designer
    );

    Ok(())
}
