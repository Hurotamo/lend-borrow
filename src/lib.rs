use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

// Constants
const COLLATERAL_RATIO: u64 = 150; // 150% collateralization ratio
const LIQUIDATION_THRESHOLD: u64 = 110; // 110% liquidation threshold

// UserAccount structure
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserAccount {
    pub is_initialized: bool,
    pub deposits: u64,
    pub borrowed: u64,
}

impl Sealed for UserAccount {}

impl IsInitialized for UserAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for UserAccount {
    const LEN: usize = 17;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        let is_initialized = src[0] != 0;
        let deposits = u64::from_le_bytes(src[1..9].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        let borrowed = u64::from_le_bytes(src[9..17].try_into().map_err(|_| ProgramError::InvalidAccountData)?);
        Ok(Self {
            is_initialized,
            deposits,
            borrowed,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..9].copy_from_slice(&self.deposits.to_le_bytes());
        dst[9..17].copy_from_slice(&self.borrowed.to_le_bytes());
    }
}

// Instruction enum
#[repr(u8)]
pub enum Instruction {
    Deposit = 0,
    Borrow = 1,
    Repay = 2,
    Withdraw = 3,
    Liquidate = 4,
}

// Entry point
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];
    match instruction {
        0 => deposit(program_id, accounts, instruction_data),
        1 => borrow(program_id, accounts, instruction_data),
        2 => repay(program_id, accounts, instruction_data),
        3 => withdraw(program_id, accounts, instruction_data),
        4 => liquidate(program_id, accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Deposit function
fn deposit(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing deposit...");
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let depositor = next_account_info(accounts_iter)?;

    if !depositor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut user_data = UserAccount::unpack(&user_account.try_borrow_data()?)?;
    user_data.deposits += u64::from_le_bytes(
        instruction_data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    UserAccount::pack(user_data, &mut user_account.try_borrow_mut_data()?)?;
    Ok(())
}

// Borrow function
fn borrow(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing borrow...");
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let borrower = next_account_info(accounts_iter)?;

    if !borrower.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut user_data = UserAccount::unpack(&user_account.try_borrow_data()?)?;
    let borrow_amount = u64::from_le_bytes(
        instruction_data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    // Check if user has enough collateral to borrow
    if user_data.deposits * 100 < borrow_amount * COLLATERAL_RATIO {
        return Err(ProgramError::InsufficientFunds);
    }

    user_data.borrowed += borrow_amount;

    UserAccount::pack(user_data, &mut user_account.try_borrow_mut_data()?)?;
    Ok(())
}

// Repay function
fn repay(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing repay...");
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let repayer = next_account_info(accounts_iter)?;

    if !repayer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut user_data = UserAccount::unpack(&user_account.try_borrow_data()?)?;
    let repay_amount = u64::from_le_bytes(
        instruction_data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    if repay_amount > user_data.borrowed {
        return Err(ProgramError::InvalidInstructionData);
    }

    user_data.borrowed -= repay_amount;

    UserAccount::pack(user_data, &mut user_account.try_borrow_mut_data()?)?;
    Ok(())
}

// Withdraw function
fn withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing withdraw...");
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let withdrawer = next_account_info(accounts_iter)?;

    if !withdrawer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut user_data = UserAccount::unpack(&user_account.try_borrow_data()?)?;
    let withdraw_amount = u64::from_le_bytes(
        instruction_data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    // Ensure enough collateral remains after withdrawal
    if (user_data.deposits - withdraw_amount) * 100 < user_data.borrowed * COLLATERAL_RATIO {
        return Err(ProgramError::InsufficientFunds);
    }

    user_data.deposits -= withdraw_amount;

    UserAccount::pack(user_data, &mut user_account.try_borrow_mut_data()?)?;
    Ok(())
}

// Liquidate function
fn liquidate(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing liquidation...");
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let liquidator = next_account_info(accounts_iter)?;

    let mut user_data = UserAccount::unpack(&user_account.try_borrow_data()?)?;

    // Check if liquidation conditions are met
    if user_data.deposits * 100 >= user_data.borrowed * LIQUIDATION_THRESHOLD {
        return Err(ProgramError::InvalidAccountData);
    }

    let liquidation_amount = u64::from_le_bytes(
        instruction_data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    if liquidation_amount > user_data.borrowed {
        return Err(ProgramError::InvalidInstructionData);
    }

    user_data.deposits -= liquidation_amount;
    user_data.borrowed -= liquidation_amount;

    UserAccount::pack(user_data, &mut user_account.try_borrow_mut_data()?)?;
    Ok(())
}
