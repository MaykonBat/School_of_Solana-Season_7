//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
        has_one = vault_authority @ VaultError::VaultLocked,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    if amount == 0 {
        return err!(VaultError::InsufficientBalance);
    }

    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    let vault_lamports = ctx.accounts.vault.to_account_info().lamports();
    if vault_lamports < amount {
        return err!(VaultError::InsufficientBalance);
    }


    **ctx.accounts.vault.to_account_info().lamports.borrow_mut() -= amount;
    **ctx.accounts.vault_authority.to_account_info().lamports.borrow_mut() += amount;

    emit!(WithdrawEvent {
        amount,
        vault_authority: ctx.accounts.vault_authority.key(),
        vault: ctx.accounts.vault.key(),
    });

    Ok(())
}