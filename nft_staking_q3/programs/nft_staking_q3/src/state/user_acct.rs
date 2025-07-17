use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AccountUser {
    pub pts: u32,
    pub amt_stkd: u8,
    pub bmp: u8,
}