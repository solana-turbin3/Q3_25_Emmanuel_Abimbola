use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer: Transfer};
//use anchor_spl::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make <'info> {
    #[account(mut)]
    pub maker: Signer <'info>,
    
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount <'info, Mint>,
    
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount <'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount <'info, TokenAccount>,
    
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker, 
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface <'info, TokenInterface>,
    pub system_program: Program<'info, System>

}

impl<'info> Make <'info> {
    pub fn init_escrow(&mut self, seed: u64, receiver: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(
            Escrow {
                seed,
                maker: self.maker.key(),
                mint_a: self.mint_a.key(),
                mint_b: self.mint_b.key(),
                receiver: self.vault.key(),
                bump: bumps.escrow
            });

        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {

        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals);

        Ok(())
    }

    pub fn close_account (&mut self) -> Result<()> {
        let close_dem_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let close_account_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), close_dem_account);

        close_account(close_account_cpi_ctx);
        Ok(())
    }

}
pub fn handler(ctx: Context<Make>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}


// Token2022 extensions allow for extended functions and features to be added.
// It has been adopted by some stocks. Because it it institutional friendly.
// Every single lineof code is a vuln point

// I learnt many new stuffs here. First off, each account serves a specific purpose. Secondly, the CPI is not for fancy. It actually plays a big role in fulfulling the transaction.
