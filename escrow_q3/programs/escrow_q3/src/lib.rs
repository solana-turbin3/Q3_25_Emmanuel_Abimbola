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
    // use crate::instructions::Initialize;
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     make::handler(ctx)
    // }

    pub fn make(ctx: Context<Make>) -> Result<()> {
        make::handler(ctx)
    }
}
