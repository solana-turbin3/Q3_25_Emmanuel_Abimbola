#![allow(deprecated, unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7mQrCXzh6BWz7dsUyWydj1JDB7H4fFbEJsZbSywe7xzp");

#[program]
pub mod escrow_q3 {
    use super::*;
    pub fn initialize(ctx: Context<Make>, seed:u64, deposit:u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }


    // pub fn make(ctx: Context<Make>) -> Result<()> {
    //     make::handler(ctx)
    // }
}
