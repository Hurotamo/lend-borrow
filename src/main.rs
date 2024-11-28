pub mod processor;
pub mod state;
pub mod utils;
pub mod instructions;
pub mod error;

use solana_program::{pubkey::Pubkey};
use crate::{state::UserAccount, utils::check_collateralization};

pub fn deposit(user_account: &mut UserAccount, amount: u64) {
    user_account.deposits += amount;
}

pub fn borrow(user_account: &mut UserAccount, amount: u64, collateral_ratio: f64) -> Result<(), ProgramError> {
    let collateral_value = (user_account.deposits as f64) * collateral_ratio;
    if amount as f64 > collateral_value {
        return Err(ProgramError::InsufficientFunds);
    }
    user_account.loan_balance += amount;
    Ok(())
}

pub fn repay(user_account: &mut UserAccount, amount: u64) -> Result<(), ProgramError> {
    if amount > user_account.loan_balance {
        return Err(ProgramError::InvalidArgument);
    }
    user_account.loan_balance -= amount;
    Ok(())
}

pub fn withdraw(user_account: &mut UserAccount, amount: u64, collateral_ratio: f64) -> Result<(), ProgramError> {
    let collateral_value = (user_account.deposits as f64) * collateral_ratio;
    if user_account.loan_balance as f64 > collateral_value - (amount as f64) {
        return Err(ProgramError::InvalidArgument);
    }
    user_account.deposits -= amount;
    Ok(())
}

pub fn liquidate(user_account: &mut UserAccount, liquidation_threshold: f64) -> Result<(), ProgramError> {
    let collateral_value = (user_account.deposits as f64);
    if user_account.loan_balance as f64 > collateral_value * liquidation_threshold {
        user_account.deposits = 0;  // Liquidate the user's deposits
        return Ok(());
    }
    Err(ProgramError::InsufficientFunds)
}
