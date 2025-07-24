#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi,
            FreezeDelegatedAccountCpiAccounts
        },
        MasterEditionAccount,
        //Metadata,
        MetadataAccount,
    },
    token::{
        Approve,
        approve,
        Mint,
        Token,TokenAccount
    },
};

use crate::{state::StakeAccount, StakeConfig};
use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata: Account<'info, TokenAccount>,


    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        
        //I don't understand this. Will have to come back to it.
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>, // the masterEdition proves non-fungibility of the asset. That's why we use it. Means no one can mint other supplies of it... What's the alternative???

     #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account <'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        space = StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub metadata_program: Account<'info, MetadataAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    //pub clock: Sysvar<'info, Clock>
}

impl<'info> Stake <'info> {
    pub fn stake(&mut self, bump: &StakeBumps) -> Result<()> {
        assert!(self.user_account.amount_staked < self.config.max_staked);

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bump.stake_account
        });

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?; //why did Berg use 1 here?

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        // all the accounts are necessary for the Freeze... to function

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts{
                delegate,
                token_account,
                edition,
                mint,
                token_program
            }
        ).invoke_signed(signer_seeds)?;
        //what does invoke_signed do here???

        self.user_account.amount_staked += 1;

        Ok(())
    }
}

// all the functions work to >> have the user stake assets.
// quite similar to tokenTransfer. The only difference is the NFTs..
// the User gives a 3rdParty ability to perform txns on their behalf (DELEGATING)
