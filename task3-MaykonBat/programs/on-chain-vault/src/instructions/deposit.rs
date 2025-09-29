//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if amount == 0 {
        return err!(VaultError::InsufficientBalance);
    }

    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    let user_lamports = ctx.accounts.user.lamports();
    if user_lamports < amount {
        return err!(VaultError::InsufficientBalance);
    }

    invoke(
        &transfer(&ctx.accounts.user.key(), &ctx.accounts.vault.key(), amount),
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    emit!(DepositEvent {
        amount,
        user: ctx.accounts.user.key(),
        vault: ctx.accounts.vault.key(),
    });

    Ok(())
}