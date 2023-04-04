use solana_test_framework::*;

use solana_sdk::pubkey::Pubkey;

#[cfg(feature = "anchor")]
use {anchor_lang::AccountDeserialize, program_for_tests::CountTracker};

mod helpers;

#[tokio::test]
#[cfg(feature = "anchor")]
async fn add_account_with_anchor() {
    let (mut program, program_id) = helpers::add_program_anchor();

    let acc_pubkey = Pubkey::new_unique();
    let count = 1;
    let anchor_data = program_for_tests::CountTracker { count };
    program.add_account_with_anchor(acc_pubkey, program_id, anchor_data, false);
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let counter_acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();
    let anchor_acc_data =
        program_for_tests::CountTracker::try_deserialize(&mut counter_acc.data.as_ref()).unwrap();
    assert_eq!(count, anchor_acc_data.count);
}

#[tokio::test]
#[cfg(feature = "anchor")]
async fn add_empty_account_with_anchor() {
    let (mut program, program_id) = helpers::add_program_anchor();

    let acc_pubkey = Pubkey::new_unique();
    program.add_empty_account_with_anchor::<CountTracker>(acc_pubkey, program_id, 50); //Size of the CountTracker struct is actually 8
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let counter_acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();
    CountTracker::try_deserialize(&mut counter_acc.data.as_ref()).unwrap(); //to ensure the data can be deserialized
    assert_eq!(50 + 8, counter_acc.data.len()); //8 for discriminator and 50 for provided data size
}
