
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount, Token},
    associated_token::AssociatedToken,
};
use crate::{CampaignConfig, ContributorState, Escrow, error::ScoringError};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Finalize <'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sponsor: Signer<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"escrow", escrow.key().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        seeds = [b"CampaignConfig", seed.to_le_bytes().as_ref()],
        bump = campaign_config.bump
    )]
    pub campaign_config: Account<'info, CampaignConfig>,

    
    #[account(
        init_if_needed,
        payer = sponsor,
        space = 8 + ContributorState::INIT_SPACE,
        seeds = [b"Contributor", user.key().as_ref()],
        bump,
    )]
    pub contributor: Account<'info, ContributorState>,

    #[account(
        associated_token::mint = usdc_token_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,   
}

impl <'info> Finalize <'info> {
    pub fn assign_score (
        &mut self,
        contribution_score: u128,
        bumps: &FinalizeBumps
    ) -> Result<()> {
        // require!()
        self.contributor.set_inner(ContributorState {
            campaign_id: self.campaign_config.campaign_id,
            user: self.user.key(),
            // git_id,
            contribution_score,
            reward_share: 0,
            claimed: false,
            created_at: Clock::get()?.unix_timestamp,
            // last_updated // I commented out because I thought it not necessary for an MVP. I was trying to lean my production
            //bump: bumps.contributor,
            bump: bumps.contributor,
        });

        Ok(())
    }

    pub fn scoring_engine (
        &mut self,
        contribution_score: u128
    ) -> Result<()> {
        require!(self.contributor.contribution_score <= 100, ScoringError::IncorrectScores);

        self.campaign_config.total_score += self.contributor.contribution_score as u128;
        let sum = self.campaign_config.total_score;

        require!(sum > 0, ScoringError::NoTotalScore);
        let reward_shares =(contribution_score * self.campaign_config.total_pool_amount as u128)/sum;

        self.contributor.reward_share = reward_shares;
        Ok(())
    }
    
}

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }
