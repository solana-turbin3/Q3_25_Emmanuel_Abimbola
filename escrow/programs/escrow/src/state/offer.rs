use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub bump: u8,
    // Details of the offer made, e.g. what who made it and what they want in return.
}

impl Offer {
    pub const INIT_SPACE: usize = 8+32+32+32+8+1;
    // pub const DISCRIMINATOR: usize = 8;
}