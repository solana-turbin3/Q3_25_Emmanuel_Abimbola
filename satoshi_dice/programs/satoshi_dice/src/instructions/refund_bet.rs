#![allow(unexpected_cfgs)]
use anchor_lang::{
    prelude::*,
    system_program::{
        transfer, Transfer
    }
};

use crate::{
    Bet,
    error::ErrorCode
};

#[derive(Accounts)]
pub struct RefundBet <'info> {

    #[account(mut)]
    pub player: Signer<'info>,

    ///CHECK: This is safe
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(
        &mut self,
        _bumps: &RefundBetBumps
    ) -> Result<()> {
        let slot = Clock::get()?.slot;
        require!(self.bet.slot - slot > 1000, ErrorCode::TimeoutNotReached);

        let cpi_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, accounts);
        transfer(ctx, self.bet.amount)?;

        Ok(())
    }
}