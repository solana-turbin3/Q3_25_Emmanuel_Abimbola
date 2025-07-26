#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{
    Token
};
use crate::{UserAccount};

#[derive(Accounts)]
pub struct InitUser <'info> {

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"user", user.key.as_ref()],
        space = UserAccount::DISCRIMINATOR.len() + UserAccount::INIT_SPACE,
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> InitUser <'info> {
    pub fn init_user(&mut self, bump: &InitUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0,
            amount_staked: 0,
            bump: bump.user_account,
        });
        Ok(())
    }

}