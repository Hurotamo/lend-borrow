use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub enum LendingInstruction {
    Deposit(u64),
    Borrow(u64),
    Repay(u64),
    Withdraw(u64),
    Liquidate,
}

impl LendingInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        match instruction_data[0] {
            0 => Ok(Self::Deposit(instruction_data[1] as u64)),
            1 => Ok(Self::Borrow(instruction_data[1] as u64)),
            2 => Ok(Self::Repay(instruction_data[1] as u64)),
            3 => Ok(Self::Withdraw(instruction_data[1] as u64)),
            4 => Ok(Self::Liquidate),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
