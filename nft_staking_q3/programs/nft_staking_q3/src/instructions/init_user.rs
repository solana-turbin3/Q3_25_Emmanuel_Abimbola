use anchor_lang::prelude::*;

use crate::ConfigState;

#[derive(Accounts)]
pub struct InitUser <'info> {

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub usr: Signer<'info>,

    #[account(
        init,
        payer = usr,
        seeds = [b"usr", usr.key.as_ref()],
        bump,
        space = AccountUser::DISCRIMINATOR.to_len() + AccountUser::INIT_SPACE,
    )]
    pub usr_acct: Account<'info, AccountUser>,
}

impl<'info> InitUsr <'info> {
    pub fn init_usr(&mut self, bumps: &InitializeUsrBumps) -> Result<()> {
        self.user_acct.set_inner(AccountUser {
            pts: 0,
            amt_stkd: 0,
            bump: bumps.usr_acct,
        });
        Ok(())
    }

}