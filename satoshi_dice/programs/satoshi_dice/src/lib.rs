#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ES2wJ7ukNH35EuhMPNKdHZUk7Q11DSZLG52Gs7cvovwu");

#[program]
pub mod satoshi_dice {
    use super::*;

    pub fn init(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        seed: u128,
        roll: u8,
        amount: u64
        ) -> Result<()> {
        ctx.accounts.place_bet(&ctx.bumps, seed, roll, amount)
    }

    pub fn refund_bet(ctx: Context<RefundBet>, ) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }

    pub fn resolve_bet(
        ctx: Context<ResolveBet>,
         sig: &[u8], 
    ) -> Result<()> {
        ctx.accounts.resolve_bet(&ctx.bumps, sig)
    }
}
