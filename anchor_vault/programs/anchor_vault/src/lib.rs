#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_lang::system_program::{{transfer, Transfer}};

declare_id!("FduhYm6BGGhZPLcmfNmiPFbgyytH3zhoDwZUJ7KLjZ5P");

#[program]
pub mod anchor_vault {
    use crate::anchor_vault::__cpi_client_accounts_close::Close;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
    
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    } 
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
    pub fn close_account(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close_account()
    }

#[derive(Accounts)]
pub struct Initialize <'info> {

    #[account(mut)]
    pub user:Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
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

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        transfer(cpi_ctx, rent_exempt)?;

        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.vault_bump = bumps.vault_state;

        
        Ok(())
        }
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,


    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()>{

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
    };
    // let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);
    let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

    transfer(cpi_ctx, amount)?;

    Ok(())
}
    pub fn withdraw(&mut self, amount: u64) -> Result<()>{


        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_account: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
    };

    let seeds: &[&[u8]; 3] = &[
        b"vault",
        self.vault_state.to_account_info().key.as_ref(),
        &[self.vault_state.vault_bump]
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];    
    let cpi_ctx= CpiContext::new_with_signer(cpi_program, cpi_account, signer_seeds);

     transfer(cpi_ctx,  amount)?;

     Ok(())

}

}

// #[derive(Accounts)]
// pub struct Close <'info> {

//     #[account(mut)]
//     pub user: Signer<'info>,
    
//     #[account(
//         seeds = [b"state", user.key().as_ref()],
//         bump = vault_state.vault_bump
//     )]
//     pub vault_state: Account<'info, VaultState>,

//     #[account(
//         mut,
//         seeds = [b"vault", vault_state.key().as_ref()],
//         bump = vault_state.vault_bump
//     )]    
//     pub vault: SystemAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct Close<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
        // Still some issues with implementation here. I need some help with it.
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl <'info> Close<'info> {
    fn close_account (&mut self) -> Result<()> {
        let cpi_account = Close 
    // pub trait AccountsClose<'info>: ToAccountInfos<'info> {
    // fn close(&self, user: AccountInfo<'info>) -> Result<()>;}
    
    Ok(())
    }
}

#[account]
pub struct VaultState { // this macro doesnt consider the discriminator. do it manually.
    pub vault_bump: u8,
    pub state_bump: u8,
}
impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1*2;
}}

// Assignment
// Use a close constraint to close the account. Close the vault account manually
// Check that the withdraw leaves the vault with a rent-exempt balance, and 
// Check that the account has enough funds for the user to withdraw.
// the account closes when you transfer out everything from it.
