use solana_test_framework::*;

use solana_sdk::pubkey::Pubkey;

mod helpers;

#[tokio::test]
#[cfg(feature = "anchor")]
async fn get_account_with_anchor() {
    let (mut program, program_id) = helpers::add_program_anchor();
    let acc_pubkey = Pubkey::new_unique();
    let count = 1;
    let anchor_data = program_for_tests::CountTracker { count };
    program.add_account_with_anchor(acc_pubkey, program_id, anchor_data, false);
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let anchor_acc_data: program_for_tests::CountTracker = banks_client
        .get_account_with_anchor(acc_pubkey)
        .await
        .unwrap();
    assert_eq!(count, anchor_acc_data.count)
}
