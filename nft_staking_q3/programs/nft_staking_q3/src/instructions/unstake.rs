use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        },
        MasterEditionAccount,
        Metadata,
        MetadataAccount,
    },
    token::{
        revoke,
        Revoke,
        Mint,
        Token,TokenAccount
    },
};


use crate::state::stake_account::*;
use crate::state::stake_config::*;

#[derive(Accounts)]
pub struct Unstake<'info> {

    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    // pub collection_mint: Account<'info, Mint>, //Not needed!

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata: Account<'info, TokenAccount>, // needed to reverse the process


    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()]
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [b"edition", b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()], // why do we need both?
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>, // the masterEdition proves non-fungibility of the asset. That's why we use it. Means no one can mint other supplies of it... What's the alternative???

     #[account(
        seeds = [b"buhari".as_ref()],
        bump = config.bump,
    )]
    pub config: Account <'info, ConfigState>,

    #[account(
        // space = StakeAccount::DISCRIMINATOR.to_len() + StakeAccount::INIT_SPACE, // not needed here
        mut,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump = stake_acct.bump,
        
    )]
    pub stake_acct: Account<'info, StakeAccount>, //use same convention
    pub metadata_program: Account<'info, Metadata>,
    pub system_program: Program<'info, Program>,
    pub token_program: Program<'info, Token>
}

impl<'info> Unstake <'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let time_elapsed = ((Clock::get()?.unix_timestamp*self.stake_acct.staked_at)/86400);
        require!(time_elapsed>self.state_config.freeze_period, StakeError::TimeElapsedError);
        self.user_config.points +=(self.stake_config.points_per_stake as u32)*time_elapsed;

        let seeds = &[
            &[b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_acct.bump]]
        ];
        let signer_seeds = &[&seeds[..]];

        let program = self.token_program.to_account_info();

        let delegate = &self.stake_acct.to_account_info();
        let token_account = &self.token_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let delegate = &self.stake_acct.to_account_info();
        let delegate = &self.stake_acct.to_account_info();

        ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), (delegate, token_account, mint, token_program, edition).invoke_signed(signer_seeds));
        let account = Revoke{
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(program, account);
        revoke(cpi_ctx)?;
        Ok(())

    }

}