use solana_test_framework::*;

use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use std::str::FromStr;

pub fn correct_entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    program_for_tests::entry(
        program_id,
        unsafe { &*(accounts as *const [AccountInfo]) },
        data,
    )
}

pub fn add_program() -> (ProgramTest, Pubkey) {
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program = ProgramTest::new(
        "program_for_tests",
        program_id,
        solana_program_test::processor!(correct_entry),
    );

    (program, program_id)
}

pub fn add_payer(program: &mut ProgramTest) -> Keypair {
    let payer = Keypair::new();
    program.add_account(
        payer.pubkey(),
        Account {
            lamports: 1_000_000_000_000,
            ..Account::default()
        },
    );

    payer
}
