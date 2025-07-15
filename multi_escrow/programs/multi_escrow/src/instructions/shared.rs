use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, TransferChecked, InterfaceAccount
};

pub fn tokens_transfer <'info> (
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &AccountInfo<'info>,
    token_program: &Interface<'info, TokenInterface>,
    seeds_owners: Option<&[&[u8]]>
) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: authority.to_account_info()
    };

    let signer_seeds = seed_owners.map(|seeds| [seeds]);

    transfer_checked(
        if let Some(seeds_array) = signer_seeds.as_ref() {
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                transfer_accounts,
                seeds_array,
            )
        } else {
            CpiContext::new(token_program.to_account_info(), transfer_accounts)
        }, *amount, mint.decimals,
    );

    Ok(())
}