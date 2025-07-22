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

    pub fn init_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8, 
        max_staked: u8, 
        freeze_period: u8
    ) -> Result<()> {
        init_config::handler(ctx, points_per_stake, max_staked, freeze_period)
    }

    pub fn init_user(
        ctx: Context<Initialize>,
        bump: u8
    ) -> Result<()> {
        init_config::handler(ctx, bump)
    }
    
    pub fn stake(
        ctx: Context<Initialize>,
        bump: u8
    ) -> Result<()> {
        stake::handler(ctx, bump)
    }

    pub fn unstake(
        ctx: Context<Initialize>
    ) -> Result<()> {
        stake::handler(ctx)
    }

}
