#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        },
        MasterEditionAccount,
        Metadata,
        //MetadataAccount,
    },
    token::{
        revoke,
        Revoke,
        Mint,
        Token,TokenAccount
    },
};


use crate::{StakeConfig, StakeAccount, UserAccount};
use crate::error::*;

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
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [
            b"edition", 
            b"metadata", 
            metadata_program.key().as_ref(), 
            mint.key().as_ref()], // why do we need both?
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>, // the masterEdition proves non-fungibility of the asset. That's why we use it. Means no one can mint other supplies of it... What's the alternative???

     #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account <'info, StakeConfig>,

    
    #[account(
        // space = StakeAccount::DISCRIMINATOR.to_len() + StakeAccount::INIT_SPACE, // not needed here
        mut,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
        
    )]
    pub stake_account: Account<'info, StakeAccount>, //use same convention
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

impl<'info> Unstake <'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let time_elapsed = (Clock::get()?.unix_timestamp - self.stake_account.staked_at)/86400;
        require!(time_elapsed>self.config.freeze_period, StakeError::TimeElapsedError);
        self.user_account.points +=(self.config.points_per_stake)*time_elapsed;

        let seeds = &[
            &b"stake".as_ref(),
            self.mint.to_account_info().key.as_ref(),
            //self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        
        let program = &self.metadata_program.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.stake_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        
        ThawDelegatedAccountCpi::new(
            program, 
        ThawDelegatedAccountCpiAccounts {
            delegate, token_account, edition, mint, token_program
        }, ).invoke_signed(signer_seeds)?;
        
        let account = Revoke{
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), account);
        revoke(cpi_ctx)?;
        Ok(())

    }

}