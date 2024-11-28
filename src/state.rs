use solana_program::account_info::AccountInfo;

#[derive(Debug)]
pub struct UserAccount {
    pub deposits: u64,
    pub loan_balance: u64,
}

impl UserAccount {
    pub fn new(deposits: u64, loan_balance: u64) -> Self {
        Self {
            deposits,
            loan_balance,
        }
    }
}
