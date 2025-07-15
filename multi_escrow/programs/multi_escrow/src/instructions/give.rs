use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    Interface::{Mint, TokenAccount, }
};
use crate::{
    boxx, shared::*, state::boxx::*

};

#[derive(Accounts)]
pub struct Give <'info> {
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, TokenInterface>,
    pub associated_token: Interface<'info, AssociatedToken>,

    #[account(
        mut,
        payer = player_one,
        seeds = [b"buhari", id.to_le_bytes().as_ref()],
        bump = boxx_bump
    )]
    pub boxx: Account<'info, Boxx>,

    #[account(
        init_if_needed,
        payer = player_n,
        associated_token::mint = player_n_mint,
        associated_token::authority = boxx,
        associated_token::token_program = token_program,
        bump = bank_bump
    )]
    pub bank: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub player_one: Signer<'info>,
    #[account(mut)]
    pub player_n: Signer<'info>,

    #[account(mint::mint = token_program)]
    pub player_one_mint: Mint<'info, Mint>,

    #[account(mint::mint = token_program)]
    pub player_n_mint: Mint<'info, Mint>,
}

impl <'info> Give <'info> {
    pub fn tokens_transfer <'info> (
        from: &player_n,
        to: &bank,
        amount: player_n_amount,
        mint: player_n_mint,
        authority: boxx,
        token_program: token_program, 
        seed_owners: None,
    ) ->Result<()> {
        transfer_checked(player_n_amount, mint.decimals)?;
    }
}