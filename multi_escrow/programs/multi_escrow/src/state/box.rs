use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Boxx {
    pub id: u64,
    pub player_one: Pubkey,
    pub player_one_mint: Pubkey,
    pub player_one_amount: u64,
    pub player_n: Vec<Pubkey>,
    pub player_n_amount: Vec<u64>,
    pub player_n_mint: Vec<Pubkey>,
    pub bank: Pubkey,
    pub boxx: Pubkey,
    pub box_bump: u8,
    pub bank_bump: u8,
}

impl Boxx {
    pub const INIT_SPACE: usize = 
    id.len() +
    player_one.len() +
    player_one_mint.len() +
    player_one_amount.len() +
    player_n.len() +
    player_n_amount.len() +
    player_n_mint.len() +
    bank.len() +
    boxx.len() +
    box_bump.len() +
    bank_bump.len();
}