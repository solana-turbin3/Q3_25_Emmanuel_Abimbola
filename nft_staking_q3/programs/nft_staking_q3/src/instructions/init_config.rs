use anchor_lang::prelude::*;

use crate::StakeConfig;

#[derive(Accounts)]
pub struct Initialize {

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub administrator: Signer<'info>,
    
    #[account(
        init,
        payer = administrator,
        seeds = [b"buhari"],
        bump,
        space = StakeConfig::DISCRIMINATOR.len() + StakeConfig::INIT_SPACE,
    )]
    pub config: Account <'info, StakeConfig>,

    #[account(
        init_if_needed,
        payer = administrator,
        seeds = [b"rewards", config.key().as_ref()]
    )]
    pub reward_mint: Account<'info, Mint>
}

impl<'info> InitConfig<'info> {
    pub fn initialize_config (&mut self, points_per_stake: u8, max_staked: u8, freeze_period: u8, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake: u8,
            max_staked: u8,
            freeze_period: u32,
            reward_bump: u8,
            bump: u8,
        });

        Ok(())
    }
}

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }
