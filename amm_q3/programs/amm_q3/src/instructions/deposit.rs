use anchor_lang::prelude::*;
use anchor_spl::{associated::AssociatedToken, token::{Mint, Token, TokenAccount}};

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
    pub user_y: Account<'info, TokenAccount>,

    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}
