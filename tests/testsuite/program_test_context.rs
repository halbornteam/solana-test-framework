use solana_test_framework::*;

use solana_sdk::{pubkey::Pubkey, sysvar::clock::Clock};

use std::str::FromStr;

#[cfg(feature = "pyth")]
use pyth_sdk_solana::state::{PriceInfo, PriceStatus, SolanaPriceAccount};

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

#[cfg(feature = "pyth")]
#[tokio::test]
async fn update_pyth_oracle() {
    let (mut program, program_id) = crate::helpers::add_program();

    let oracle = Pubkey::new_unique();

    let time_stamp: i64 = 200;
    let price_info = PriceInfo {
        price: 10,
        conf: 20,
        status: PriceStatus::Trading,
        pub_slot: 3,
        ..Default::default()
    };
    let valid_slot = 10;
    let price_account = SolanaPriceAccount {
        magic: 0xa1b2c3d4,
        ver: 2,
        expo: 5,
        atype: 3,
        agg: price_info,
        timestamp: time_stamp,
        prev_timestamp: 100,
        prev_price: 60,
        prev_conf: 70,
        prev_slot: 1,
        valid_slot,
        ..Default::default()
    };

    //add the pyth oracle to the context
    program
        .add_pyth_oracle(oracle, program_id, Some(price_account), None, None)
        .unwrap();

    let mut program_context = program.start_with_context().await;
    let mut banks_client = program_context.banks_client.clone();

    //get pyth price account data from chain
    let price_data = banks_client.get_pyth_price_account(oracle).await.unwrap();
    assert_eq!(price_data, price_account);

    let price_info2 = PriceInfo {
        price: 11,
        conf: 21,
        status: PriceStatus::Trading,
        pub_slot: 3,
        ..Default::default()
    };
    let price_account2 = SolanaPriceAccount {
        magic: 0xa1b2c3d4,
        ver: 2,
        expo: 5,
        atype: 3,
        agg: price_info2,
        timestamp: 54,
        prev_timestamp: 100,
        prev_price: 60,
        prev_conf: 70,
        prev_slot: 1,
        valid_slot: 32,
        ..Default::default()
    };

    program_context
        .update_pyth_oracle(oracle, Some(price_account2), None, None, None)
        .await
        .unwrap();

    let price_data = banks_client.get_pyth_price_account(oracle).await.unwrap();
    assert_eq!(price_data, price_account2);

    program_context
        .update_pyth_oracle(
            oracle,
            None,
            Some(price_info),
            Some(time_stamp),
            Some(valid_slot),
        )
        .await
        .unwrap();

    let price_data = banks_client.get_pyth_price_account(oracle).await.unwrap();
    assert_eq!(price_data, price_account);
}
