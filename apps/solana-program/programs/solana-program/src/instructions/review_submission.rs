use crate::helpers::enums::ErrorCode;
use crate::state::bounty::Bounty;
use crate::state::escrow::Escrow;
use crate::state::nft_submission::NFTSubmission;
use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;

#[derive(Accounts)]
pub struct ReviewNFT<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // Game studio

    #[account(
        mut,
        seeds = [b"bounty", creator.key().as_ref(), bounty.bounty_name.as_bytes()],
        bump
    )]
    pub bounty: Account<'info, Bounty>, // Reference to the bounty

    #[account(
        mut,
        seeds = [b"submission", bounty.key().as_ref(), submission.designer.key().as_ref()],
        bump
    )]
    pub submission: Account<'info, NFTSubmission>, // NFT submission record

    #[account(
        mut,
        seeds = [b"escrow", bounty.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>, // Escrow account

    #[account(mut)]
    pub designer: SystemAccount<'info>, // Designer's account (Ensures `to_account_info()` works)

    pub system_program: Program<'info, System>,
}

pub fn review_nft(ctx: Context<ReviewNFT>, approve: bool) -> Result<()> {
    let submission = &mut ctx.accounts.submission;
    let bounty = &ctx.accounts.bounty;
    let escrow = &mut ctx.accounts.escrow;

    require!(
        bounty.creator == ctx.accounts.creator.key(),
        ErrorCode::NotBountyCreator
    );

    if approve {
        submission.status = "Approved".to_string();

        // Transfer bounty amount to designer
        let transfer_result = anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: escrow.to_account_info(),
                    to: ctx.accounts.designer.to_account_info(), // Corrected
                },
            ),
            escrow.balance,
        );

        if transfer_result.is_err() {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Empty escrow balance
        escrow.balance = 0;

        msg!(
            "NFT Approved: {} - Bounty {} paid to {}",
            submission.nft_mint,
            bounty.bounty_name,
            ctx.accounts.designer.key() // Corrected
        );
    } else {
        submission.status = "Rejected".to_string();
        msg!("NFT Rejected: {}", submission.nft_mint);
    }

    Ok(())
}
