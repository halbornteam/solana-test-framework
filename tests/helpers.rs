use solana_test_framework::*;

use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use std::str::FromStr;

pub fn add_program() -> (ProgramTest, Pubkey) {
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program = ProgramTest::new(
        "program_for_tests",
        program_id,
        processor!(program_for_tests::entry),
    );

    return (program, program_id);
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

    return payer;
}
