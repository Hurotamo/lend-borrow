use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey};
use crate::{processor::process_deposit, processor::process_borrow, processor::process_repay, processor::process_withdraw, processor::process_liquidate};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    match instruction_data[0] {
        0 => process_deposit(program_id, accounts_iter, instruction_data), // Deposit
        1 => process_borrow(program_id, accounts_iter, instruction_data), // Borrow
        2 => process_repay(program_id, accounts_iter, instruction_data),  // Repay
        3 => process_withdraw(program_id, accounts_iter, instruction_data), // Withdraw
        4 => process_liquidate(program_id, accounts_iter, instruction_data), // Liquidate
        _ => Err(solana_program::program_error::ProgramError::InvalidInstructionData),
    }
}
