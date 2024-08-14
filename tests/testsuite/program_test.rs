use solana_test_framework::*;

use std::borrow::Borrow;

use {
    solana_sdk::{
        native_token::sol_to_lamports, program_option::COption, program_pack::Pack, pubkey::Pubkey,
        signature::Signer,
    },
    spl_token::state::{Account as TokenAccount, Mint},
};

use borsh::BorshDeserialize;

#[cfg(feature = "anchor")]
use {anchor_lang::AccountDeserialize, program_for_tests::CountTracker};

use crate::helpers;
#[cfg(feature = "pyth")]
use pyth_sdk_solana::state::{PriceAccount, PriceInfo, PriceStatus};

#[tokio::test]
async fn generate_accounts() {
    let (mut program, _) = helpers::add_program();
    let number_of_accounts = 10;
    let initial_lamports = sol_to_lamports(1_000.0);
    let accounts = program.generate_accounts(10);
    let first_account = &accounts[0];
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let first_account_data = banks_client
        .get_account(first_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(accounts.len(), number_of_accounts as usize);
    assert_eq!(first_account_data.lamports, initial_lamports);
}

#[tokio::test]
async fn add_account_with_data() {
    let (mut program, _) = helpers::add_program();

    let acc_pubkey = Pubkey::new_unique();
    let owner = Pubkey::new_unique();

    // USDC Mint from mainnet
    // got using solana account --output-file usdc_mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    let (data, _) =
        solana_test_framework::util::load_file_to_bytes("tests/testsuite/artifacts/usdc_mint");

    program.add_account_with_data(acc_pubkey, owner, &data[..], false);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    //Test mint with defaults creation
    let acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();

    assert_eq!(acc.data, data);
}

#[tokio::test]
#[cfg(feature = "anchor")]
async fn add_account_with_anchor() {
    let (mut program, program_id) = helpers::add_program();

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
    let (mut program, program_id) = helpers::add_program();

    let acc_pubkey = Pubkey::new_unique();
    program.add_empty_account_with_anchor::<CountTracker>(acc_pubkey, program_id, 50); //Size of the CountTracker struct is actually 8
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let counter_acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();
    CountTracker::try_deserialize(&mut counter_acc.data.as_ref()).unwrap(); //to ensure the data can be deserialized
    assert_eq!(50 + 8, counter_acc.data.len()); //8 for discriminator and 50 for provided data size
}

#[tokio::test]
async fn add_account_with_lamports() {
    let (mut program, _) = helpers::add_program();

    let acc_pubkey = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let lamports = 1000;

    program.add_account_with_lamports(acc_pubkey, owner, lamports);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();

    assert_eq!(acc.lamports, lamports);
}

#[tokio::test]
async fn add_account_with_packable() {
    let (mut program, _) = helpers::add_program();

    let mint_authority = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let supply = 1000;
    let decimals = 10;
    let freeze_authority = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();

    let mint = spl_token::state::Mint {
        mint_authority: COption::from(mint_authority),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: COption::from(freeze_authority),
    };

    program.add_account_with_packable(mint_pubkey, owner, mint);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    //Test mint with defaults creation
    let acc = banks_client
        .get_account(mint_pubkey)
        .await
        .unwrap()
        .unwrap();
    let mint_data = Mint::unpack(&acc.data).unwrap();
    assert_eq!(mint_data.freeze_authority.unwrap(), freeze_authority);
    assert_eq!(mint_data.supply, supply);
    assert_eq!(mint_data.decimals, decimals);
    assert!(mint_data.is_initialized);
    assert_eq!(mint_data.mint_authority.unwrap(), mint_authority);
}

#[tokio::test]
async fn add_account_with_borsh() {
    let (mut program, program_id) = helpers::add_program();

    let acc_pubkey = Pubkey::new_unique();
    let counter = 1;
    let acc_data = program_for_tests::GreetingAccount { counter };
    program.add_account_with_borsh(acc_pubkey, program_id, acc_data);
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let greeting_acc = banks_client.get_account(acc_pubkey).await.unwrap().unwrap();
    let greeting_acc_data =
        program_for_tests::GreetingAccount::try_from_slice(greeting_acc.data.borrow()).unwrap();
    assert_eq!(counter, greeting_acc_data.counter);
}

#[tokio::test]
async fn add_token_mint() {
    let (mut program, _) = helpers::add_program();

    let freeze_pubkey = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    //Create mint with defaults
    program.add_token_mint(mint_pubkey, None, 10, 0, Some(freeze_pubkey));

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint_pubkey)
        .await
        .unwrap()
        .unwrap();
    let mint_data = Mint::unpack(&mint_acc.data).unwrap();
    assert_eq!(mint_data.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_acc.owner, spl_token::id());
}

#[tokio::test]
async fn add_token_account() {
    let (mut program, _) = helpers::add_program();

    let token_account_pubkey = Pubkey::new_unique();
    let owner = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    //Create mint with defaults
    program.add_token_mint(mint_pubkey, None, 10, 0, None);
    let amount = 1;
    program.add_token_account(
        token_account_pubkey,
        mint_pubkey,
        owner,
        amount,
        None,
        None,
        0,
        None,
    );

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let token_account = banks_client
        .get_account(token_account_pubkey)
        .await
        .unwrap()
        .unwrap();

    let token_account_data = TokenAccount::unpack(&token_account.data).unwrap();

    assert_eq!(token_account_data.amount, amount);
    assert_eq!(token_account_data.mint, mint_pubkey);
    assert_eq!(token_account_data.owner, owner);
}

#[tokio::test]
async fn add_associated_token_account() {
    let (mut program, _) = helpers::add_program();

    let owner = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    //Create mint with defaults
    program.add_token_mint(mint_pubkey, None, 10, 0, None);
    let amount = 1;
    let associated_token_account =
        program.add_associated_token_account(mint_pubkey, owner, amount, None, None, 0, None);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let associated_token_account = banks_client
        .get_account(associated_token_account)
        .await
        .unwrap()
        .unwrap();

    let associated_token_account_data =
        TokenAccount::unpack(&associated_token_account.data).unwrap();

    assert_eq!(associated_token_account_data.amount, amount);
    assert_eq!(associated_token_account_data.mint, mint_pubkey);
    assert_eq!(associated_token_account_data.owner, owner);
}

#[tokio::test]
#[cfg(feature = "pyth")]
async fn add_pyth_price_feed() {
    let (mut program, program_id) = helpers::add_program();

    let oracle = Pubkey::new_unique();
    let oracle2 = Pubkey::new_unique();
    let time_stamp: i64 = 200;
    let price_info = PriceInfo {
        price: 10,
        conf: 20,
        status: PriceStatus::Trading,
        pub_slot: 3,
        ..Default::default()
    };
    let price_account = PriceAccount {
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
        ..Default::default()
    };

    //add the pyth oracle to the context
    program
        .add_pyth_oracle(oracle, program_id, Some(price_account), None, None)
        .unwrap();
    program
        .add_pyth_oracle(
            oracle2,
            program_id,
            None,
            Some(price_info),
            Some(time_stamp),
        )
        .unwrap();

    let (mut banks_client, _, _) = program.start().await;

    //get pyth price account data from chain
    let price_data = banks_client.get_pyth_price_account(oracle).await.unwrap();
    assert_eq!(price_data, price_account);

    let price_data = banks_client.get_pyth_price_account(oracle2).await.unwrap();
    assert_eq!(price_data, price_account);
}
