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

declare_id!("9Fz4DyJq7Dn8EgbYnzd9W5AzzCtLMQSyHZqskK5AgVZf");

#[program]
pub mod nft_staking_q3 {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: i64, 
        max_staked: u8, 
        freeze_period: i64, 
        //bump: &InitializeConfigBumps,
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_staked, freeze_period, &ctx.bumps)
    }

    pub fn init_user(
        ctx: Context<InitUser>,
        //bump: &InitUserBumps
    ) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }
    
    pub fn stake(
        ctx: Context<Stake>,
        //bump: u8
    ) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(
        ctx: Context<Unstake>
    ) -> Result<()> {
        ctx.accounts.unstake()
    }

}
