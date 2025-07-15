// use std::alloc::System;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{Mint, TokenAccount, TokenInterface, InterfaceAccount}
};

#[derive(Accounts)]
pub struct Initialize {
    pub system_program: Program<'info, System>,
    pub associated_token_program: Interface<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,


    #[account(
        init,
        payer = player_one,
        seeds = [b"game", id.to_le_byte().as_ref()],
        bump = box_bump
    )]
    pub boxx: Account<'info, Boxx>,

    #[account(
        init,
        payer = player_one,
        associated_token::mint = token_program,
        associated_token::authority = boxx,
        associated_token::token_program = token_program
    )]
    pub bank: TokenInterface<'info, TokenAccount>,

    #[account(mint::mint = token_program)]
    pub player_one_mint: Mint<'info, Mint>,

    #[account(mint::mint = token_program)]
    pub player_n_mint: Mint<'info, Mint>,

    #[account(mut)]
    pub player_one: Signer<'info>,
    #[account(mut)]
    pub player_n: Signer<'info>,
}

pub fn handler(ctx: Context<Initialize>, bumps: &InitializeBumps) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
