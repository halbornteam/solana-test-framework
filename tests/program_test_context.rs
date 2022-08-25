use solana_test_framework::*;

use solana_sdk::{pubkey::Pubkey, sysvar::clock::Clock};

use std::str::FromStr;

#[tokio::test]
async fn transaction_from_instructions() {
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program = ProgramTest::new(
        "program_for_tests",
        program_id,
        processor!(program_for_tests::entry),
    );

    let mut program_context = program.start_with_context().await;

    let clock_from_program_context: Clock =
        program_context.banks_client.get_sysvar().await.unwrap();

    let timestamp_before = clock_from_program_context.unix_timestamp;
    let moving_time = 60 * 60 * 24 * 30;
    program_context
        .warp_to_timestamp(clock_from_program_context.unix_timestamp + moving_time)
        .await
        .unwrap();
    let clock_from_program_context: Clock =
        program_context.banks_client.get_sysvar().await.unwrap();

    let timestamp_now = clock_from_program_context.unix_timestamp;

    assert_eq!(timestamp_before + moving_time, timestamp_now)
}
