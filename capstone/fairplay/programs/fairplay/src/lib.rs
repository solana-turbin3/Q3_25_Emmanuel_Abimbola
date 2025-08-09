#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3qwWMVMuLXq6TXA7QFEXPL8Ajwua6nZ8a6odXqE8431E");

#[program]
pub mod fairplay {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        campaign_id: u8,
        total_pool_amount: u128,
        start_time: i64,
        end_time: i64,
        total_score: u128,
        no_of_contributors: u32,
        created_at: i64,
    ) -> Result<()> {
        ctx.accounts.initialize(campaign_id, total_pool_amount, start_time, end_time, total_score, no_of_contributors, created_at, &ctx.bumps)
    }

    pub fn initialize_escrow(
        ctx: Context<Initialize>,
        campaign_id: u8
    ) -> Result<()> {
        ctx.accounts.initialize_escrow(campaign_id, &ctx.bumps)
    }

    pub fn initialize_contributor_state(
        ctx: Context<Initialize>,
        created_at: i64
    ) -> Result<()> {
        ctx.accounts.initialize_contributor_state(created_at, &ctx.bumps)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64
    ) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn assign_score(
        ctx: Context<Finalize>,
        contribution_score: u128,
    ) -> Result<()> {
        ctx.accounts.assign_score(contribution_score, &ctx.bumps)
    }

    
    pub fn scoring_engine(
        ctx: Context<Finalize>,
        contribution_score: u128,
    ) -> Result<()> {
        ctx.accounts.scoring_engine(contribution_score)
    }

    pub fn claim_reward(
        ctx: Context<Claim>,
        reward_share: u64,
    ) -> Result<()> {
        ctx.accounts.claim_reward(reward_share)
    }


}
