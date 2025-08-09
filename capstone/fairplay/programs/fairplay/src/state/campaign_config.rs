use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CampaignConfig {
   pub campaign_id: u8,
   pub sponsor: Pubkey,
   pub vault: Pubkey,
   pub usdc_token_mint: Pubkey,
   pub total_pool_amount: u128,
   pub start_time: i64,
   pub end_time: i64,
   pub is_finalized: bool,
   pub total_score: u128,
   pub no_of_contributors: u32,
   pub created_at: i64,
   pub bump: u8
}