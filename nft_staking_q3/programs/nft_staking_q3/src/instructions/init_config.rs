use anchor_lang::prelude::*;

use crate::ConfigState;

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
        space = ConfigState::DISCRIMINATOR.len() + ConfigState::INIT_SPACE,
    )]
    pub config: Account <'info, ConfigState>,

    #[account(
        init_if_needed,
        payer = administrator,
        seeds = [b"rwds", configg.key().as_ref()]
    )]
    pub rwd_mnt: Account<'info, Mint>
}

impl<'info> InitConfig<'info> {
    pub fn initialize_configg (&mut self, points_per_stake: u8, max_staked: u8, freeze_period: u8, bumps: &InitializeConfiggBumps) {
        self.config.set_inner(ConfigState {
            points_per_stake: u8,
            max_staked: u8,
            freeze_period: u32,
            reward_bump: u8,
            bump: u8,
        });
    }
}

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }
