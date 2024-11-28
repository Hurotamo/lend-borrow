use solana_program::{account_info::{AccountInfo}, entrypoint::ProgramResult};
use crate::{lib::{deposit, borrow, repay, withdraw, liquidate}, state::UserAccount};

pub fn process_deposit(
    program_id: &Pubkey, 
    accounts_iter: &mut dyn Iterator<Item = &AccountInfo>, 
    instruction_data: &[u8]
) -> ProgramResult {
    let user_account = &mut **accounts_iter.next().unwrap();
    let amount = instruction_data[1] as u64; // Assuming data is simple for now.
    deposit(user_account, amount);
    Ok(())
}

pub fn process_borrow(
    program_id: &Pubkey, 
    accounts_iter: &mut dyn Iterator<Item = &AccountInfo>, 
    instruction_data: &[u8]
) -> ProgramResult {
    let user_account = &mut **accounts_iter.next().unwrap();
    let amount = instruction_data[1] as u64;
    let collateral_ratio = 1.5; // Example: 150% collateralization ratio
    borrow(user_account, amount, collateral_ratio)?;
    Ok(())
}

pub fn process_repay(
    program_id: &Pubkey, 
    accounts_iter: &mut dyn Iterator<Item = &AccountInfo>, 
    instruction_data: &[u8]
) -> ProgramResult {
    let user_account = &mut **accounts_iter.next().unwrap();
    let amount = instruction_data[1] as u64;
    repay(user_account, amount)?;
    Ok(())
}

pub fn process_withdraw(
    program_id: &Pubkey, 
    accounts_iter: &mut dyn Iterator<Item = &AccountInfo>, 
    instruction_data: &[u8]
) -> ProgramResult {
    let user_account = &mut **accounts_iter.next().unwrap();
    let amount = instruction_data[1] as u64;
    let collateral_ratio = 1.5; // Example: 150% collateralization ratio
    withdraw(user_account, amount, collateral_ratio)?;
    Ok(())
}

pub fn process_liquidate(
    program_id: &Pubkey, 
    accounts_iter: &mut dyn Iterator<Item = &AccountInfo>, 
    instruction_data: &[u8]
) -> ProgramResult {
    let user_account = &mut **accounts_iter.next().unwrap();
    let liquidation_threshold = 2.0; // Example: liquidation threshold of 200%
    liquidate(user_account, liquidation_threshold)?;
    Ok(())
}
