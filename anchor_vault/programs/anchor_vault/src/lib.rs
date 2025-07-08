#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("FduhYm6BGGhZPLcmfNmiPFbgyytH3zhoDwZUJ7KLjZ5P");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
    
    pub fn deposit(ctx: Context<Payment>) -> Result<()> {
        ctx.accounts.deposit(&ctx.bumps)
    } 
    pub fn withdraw(ctx: Context<Payment>) -> Result<()> {
        ctx.accounts.withdraw(&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {

    #[account(
        mut,
    )]
    pub user:Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = Vault::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>

}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{

        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, lamports:rent_exempt)?;

        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.vault_bump = bumps.vault_state;
        }
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAcount<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, VaultState>

    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn Deposit(&mut self, amount: u64) -> Result<()>{


        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
    };
    let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

    transfer(cpi_ctx, lamports:amount)


}

    pub fn withdraw(&mut self, amount: u64) -> Result<()>{


        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
    };

    let seeds: &[&[u8]; 3] = &[
        b"vault",
        self.vault_state.to_account_info().key.as_ref(),
        &[self.vault_state.bump]
    ]];
}




#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1*2;
}