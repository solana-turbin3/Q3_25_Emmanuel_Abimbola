use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAcct {
    pub ownr: Pubkey,
    pub mnt: u8,
    pub bmp: u8,
    pub stkd_at: i64,
    pub bmp: u8,
}