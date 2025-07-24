#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9Fz4DyJq7Dn8EgbYnzd9W5AzzCtLMQSyHZqskK5AgVZf");

#[program]
pub mod nft_staking_q3 {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8, 
        max_staked: u8, 
        freeze_period: i64, 
        reward_bump: u8, 
        bump: u8,
    ) -> Result<()> {
        init_config::handler(ctx, points_per_stake, max_staked, freeze_period, reward_bump, bump)
    }

    pub fn init_user(
        ctx: Context<InitUser>,
        bump: u8
    ) -> Result<()> {
        init_user::handler(ctx, bump)
    }
    
    pub fn stake(
        ctx: Context<Stake>,
        bump: u8
    ) -> Result<()> {
        stake::handler(ctx, bump)
    }

    pub fn unstake(
        ctx: Context<Unstake>
    ) -> Result<()> {
        unstake::handler(ctx)
    }

}
