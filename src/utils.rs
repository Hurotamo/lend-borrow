use solana_program::program_error::ProgramError;

pub fn check_collateralization(deposits: u64, loan_balance: u64, collateral_ratio: f64) -> Result<(), ProgramError> {
    let collateral_value = (deposits as f64) * collateral_ratio;
    if loan_balance as f64 > collateral_value {
        return Err(ProgramError::InsufficientFunds);
    }
    Ok(())
}
