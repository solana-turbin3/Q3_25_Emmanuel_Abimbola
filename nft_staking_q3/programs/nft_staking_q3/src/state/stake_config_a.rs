anchor_lang::prelude::*;

#[account]
pub struct ConfigState {
    pub pts_p_stk: u8,
    pub max_stk: u8,
    pub frz_prd: u32,
    pub rwd_bmp: u8,
    pub bmp: u8,
}