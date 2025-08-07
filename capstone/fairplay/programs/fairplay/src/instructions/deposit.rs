#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount, Token, transfer, Transfer}
};

use crate::{CampaignConfig, ContributorState, Escrow};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit <'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub sponsor: Signer<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"escrow", escrow.key().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        seeds = [b"CampaignConfig", seed.to_le_bytes().as_ref()],
        bump = campaign_config.bump
    )]
    pub campaign_config: Account<'info, CampaignConfig>,

    
    #[account(
        seeds = [b"Contributor", user.key().as_ref()],
        bump = contributor.bump
    )]
    pub contributor: Account<'info, ContributorState>,

    #[account(
        associated_token::mint = usdc_token_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,   
}

impl<'info> Deposit <'info> {
    pub fn deposit (
        &mut self,
        amount: u64
    ) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account = Transfer {
            from: self.usdc_token_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program, cpi_account);
        transfer(ctx, amount)?;

        Ok(())
    }
}