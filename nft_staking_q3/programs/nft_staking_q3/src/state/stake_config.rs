use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake: i64,
    pub max_staked: u8,
    pub freeze_period: i64,
    pub reward_bump: u8,
    pub bump: u8,
}