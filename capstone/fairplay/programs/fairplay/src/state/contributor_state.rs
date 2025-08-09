use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ContributorState {
    pub campaign_id: u8,
    pub user: Pubkey,

    // #[max_len(500, 200)]
    // pub git_id: Vec<u64>,
    pub contribution_score: u128,
    pub reward_share: u128,
    pub claimed: bool,

    #[max_len(500, 200)]
    // pub contributions: Vec<Pubkey>,
    pub created_at: i64,
    // pub last_updated: i64,
    pub bump: u8
}