use anchor_lang::{prelude::*, init_if_needed};
use anchor_spl::{associated::AssociatedToken, token::Mint, Token, TokenAccount};
use constant_product_curve::ConstantProduct;

use crate::{state::Config};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {

    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump,
    )]
    pub mint_lp: Account<'info, Mint>,

     #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,


    
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint_y.,
        associated_token::authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,

    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}

impl<'info> Deposit<'info> {

    pub fn deposit(
        &mut self, 
        amount: u64, 
        max_x: u64, 
        max_y: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);
        

        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    x: self.vault_x.amount,
                    y: self.vault_y.amount,
                    l: self.mint_lp.supply,
                    a: amount,
                precision: 6).unwrap();
                (amount.x, amount.y)
            }
        };
    }
    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program =  self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(ctx, amount)
    }

    pub fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.config.to_account_info()
        };
        let signer_seeds = &[&seeds[..]];
        
        let ctx = CpiContext::new_with_signer(self.token_program, cpi_accounts, signer_seeds);
        mint_to(ctx, amount)
    }
}
