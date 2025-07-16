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

use crate::StakeAcct;

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
        bump = user_acct.bump,
    )]
    pub user_account: Account<'info, StakeAcct>,

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
        // space = StakeAcct::DISCRIMINATOR.to_len() + StakeAcct::INIT_SPACE, // not needed here
        mut,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump = stake_acct.bump,
        
    )]
    pub stake_acct: Account<'info, StakeAcct>, //use same convention
    pub metadata_program: Account<'info, Metadata>,
    pub system_program: Program<'info, Program>,
    pub token_program: Program<'info, Token>
}

impl<'info> Unstake <'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let signer_seeds = &[&seeds[..]];
        let delegate = &self.stake_acct.to_account_info();
        let token_account = &self.token_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let delegate = &self.stake_acct.to_account_info();
        let delegate = &self.stake_acct.to_account_info();

    }
}