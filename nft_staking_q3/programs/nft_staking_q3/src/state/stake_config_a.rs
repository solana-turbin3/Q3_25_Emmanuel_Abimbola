anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfigState {
    pub points_per_stake: u8,
    pub max_staked: u8,
    pub freeze_period: u32,
    pub reward_bump: u8,
    pub bump: u8,
}