#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, Token}
};

use crate::{CampaignConfig, ContributorState, Escrow};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize <'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sponsor: Signer<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = sponsor,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", escrow.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = sponsor,
        space = 8 + CampaignConfig::INIT_SPACE,
        seeds = [b"CampaignConfig", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub campaign_config: Account<'info, CampaignConfig>,

    
    #[account(
        init,
        payer = user,
        space = 8 + ContributorState::INIT_SPACE,
        seeds = [b"Contributor", user.key().as_ref()],
        bump
    )]
    pub contributor: Account<'info, ContributorState>,

    #[account(
        init,
        payer = sponsor,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    
}

impl <'info> Initialize <'info> {
    pub fn initialize (
        &mut self,
        campaign_id: u8,
        total_pool_amount: u128,
        start_time: i64,
        end_time: i64,
        total_score: u128,
        no_of_contributors: u32,
        created_at: i64,
        bumps: &InitializeBumps
    ) -> Result<()> {
        self.campaign_config.set_inner( CampaignConfig {
            campaign_id,
            sponsor: self.sponsor.key(),
            vault: self.vault.key(),
            usdc_token_mint: self.usdc_token_mint.key(),
            total_pool_amount,
            start_time,
            end_time,
            is_finalized: false,
            total_score,
            no_of_contributors,
            created_at,
            bump: bumps.campaign_config
        });

        Ok(())
    }

    pub fn initialize_escrow (
        &mut self,
        campaign_id: u8,
        bumps: &InitializeBumps
    ) -> Result<()> {
        self.escrow.set_inner( Escrow {
            owner: self.sponsor.key(),
            campaign_id,
            bump: bumps.escrow
        });
        Ok(())

    }

    pub fn initialize_contributor_state (
        &mut self,
        // git_id: Vec<u64>,
        created_at: i64,
        bumps: &InitializeBumps
    ) -> Result<()> {
        self.contributor.set_inner(ContributorState {
            campaign_id: self.campaign_config.campaign_id,
            user: self.user.key(),
            // git_id,
            contribution_score: 0,
            reward_share: 0,
            claimed: false,
            //contributions: 0,
            created_at,
            // last_updated: 0,
            bump: bumps.contributor
        });

        Ok(())
    }
}
// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }
