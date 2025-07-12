use anchor_lang::{
    prelude::*,
    //init_if_needed,
};

use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, close_account, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take <'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

        
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,


    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program

    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,



    
    #[account(
        init,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::token_program = associated_token_program,
        associated_token::authority = escrow,

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = mint_a,
        has_one = mint_b,
        has_one = taker,
        close = taker,
        seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
             from: self.taker_ata_b.to_account_info(),
             to: self.taker_ata_b.to_account_info(),
             mint: self.mint_b.to_account_info(),
             authority: self.taker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), close_accounts);
        close_account(cpi_ctx)?;

        Ok(())
    }

    
}