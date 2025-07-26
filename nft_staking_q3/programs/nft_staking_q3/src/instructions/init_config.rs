#![allow(unexpected_cfgs)]
// #![allow()]
use anchor_lang::prelude::*;
use anchor_spl::token:: {Mint, Token};
use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig <'info> {

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub administrator: Signer<'info>,
    
    #[account(
        init,
        payer = administrator,
        seeds = [b"config"],
        bump,
        space = StakeConfig::DISCRIMINATOR.len() + StakeConfig::INIT_SPACE,
    )]
    pub config: Account <'info, StakeConfig>,

    #[account(
        init_if_needed,
        payer = administrator,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config
    )]
    pub reward_mint: Account<'info, Mint>
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config (
        &mut self,
        points_per_stake: i64, 
        max_staked: u8, 
        freeze_period: i64, 
        bump: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake,
            max_staked,
            freeze_period,
            reward_bump: bump.reward_mint,
            bump: bump.config,
        });

        Ok(())
    }
}

// pub fn handler(ctx: Context<Initialize>) -> Result<()> {
//     msg!("Greetings from: {:?}", ctx.program_id);
//     Ok(())
// }
