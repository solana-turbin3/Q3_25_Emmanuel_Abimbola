pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9Fz4DyJq7Dn8EgbYnzd9W5AzzCtLMQSyHZqskK5AgVZf");

#[program]
pub mod nft_staking_q3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        init_config::handler(ctx)
    }
}
