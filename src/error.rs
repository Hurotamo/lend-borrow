use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LendingError {
    #[error("Insufficient funds to perform the action.")]
    InsufficientFunds,
    
    #[error("Invalid argument passed.")]
    InvalidArgument,
    
    #[error("Collaterals are insufficient for the loan.")]
    CollateralNotSufficient,
}

impl From<LendingError> for ProgramError {
    fn from(e: LendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
