use solana_test_framework::*;

use {
    solana_sdk::{
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction,
    },
    spl_token::state::{Account as TokenAccount, Mint},
};

#[cfg(feature = "anchor")]
use anchor_lang::{AccountDeserialize, AnchorSerialize, Discriminator};

use std::str::FromStr;

#[cfg(feature = "anchor")]
use anchor_lang::InstructionData;

mod helpers;

#[tokio::test]
async fn transaction_from_instructions() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let acc_1 = Keypair::new();
    let acc_2 = Keypair::new();
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let acc_lamports = 1_000_000;
    let ix_1 = system_instruction::create_account(
        &payer.pubkey(),
        &acc_1.pubkey(),
        acc_lamports,
        1,
        &acc_1.pubkey(),
    );
    let ix_2 = system_instruction::create_account(
        &payer.pubkey(),
        &acc_2.pubkey(),
        acc_lamports,
        1,
        &acc_2.pubkey(),
    );
    let tx = banks_client
        .transaction_from_instructions(&[ix_1, ix_2], &payer, vec![&payer, &acc_1, &acc_2])
        .await
        .unwrap();

    banks_client.process_transaction(tx).await.unwrap();
    let acc1_data = banks_client
        .get_account(acc_1.pubkey())
        .await
        .unwrap()
        .unwrap();
    let acc2_data = banks_client
        .get_account(acc_2.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(acc1_data.owner, acc_1.pubkey());
    assert_eq!(acc2_data.owner, acc_2.pubkey());
}

#[tokio::test]
async fn transaction_from_instructions_upgradeable() {
    let mut program = ProgramTest::default();
    program.add_bpf_program(
        "tests/artifacts/program_for_tests",
        Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap(),
        Some(Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap()),
        None
    );
    let payer = helpers::add_payer(&mut program);
    let acc_1 = Keypair::new();
    let acc_2 = Keypair::new();
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let acc_lamports = 1_000_000;
    let ix_1 = system_instruction::create_account(
        &payer.pubkey(),
        &acc_1.pubkey(),
        acc_lamports,
        1,
        &acc_1.pubkey(),
    );
    let ix_2 = system_instruction::create_account(
        &payer.pubkey(),
        &acc_2.pubkey(),
        acc_lamports,
        1,
        &acc_2.pubkey(),
    );
    let tx = banks_client
        .transaction_from_instructions(&[ix_1, ix_2], &payer, vec![&payer, &acc_1, &acc_2])
        .await
        .unwrap();

    banks_client.process_transaction(tx).await.unwrap();
    let acc1_data = banks_client
        .get_account(acc_1.pubkey())
        .await
        .unwrap()
        .unwrap();
    let acc2_data = banks_client
        .get_account(acc_2.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(acc1_data.owner, acc_1.pubkey());
    assert_eq!(acc2_data.owner, acc_2.pubkey());
}

#[tokio::test]
#[cfg(feature = "anchor")]
async fn get_account_with_anchor() {
    let (mut program, program_id) = helpers::add_program();
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

#[tokio::test]
async fn get_account_with_borsh() {
    let (mut program, program_id) = helpers::add_program();

    let acc_pubkey = Pubkey::new_unique();
    let counter = 1;
    let acc_data = program_for_tests::GreetingAccount { counter: counter };
    program.add_account_with_borsh(acc_pubkey, program_id, acc_data);
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    let greeting_acc_data: program_for_tests::GreetingAccount = banks_client
        .get_account_with_borsh(acc_pubkey)
        .await
        .unwrap();
    assert_eq!(counter, greeting_acc_data.counter);
}

#[tokio::test]
async fn create_account() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let lamports = 1_000_000;
    let new_acc = Keypair::new();
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    banks_client
        .create_account(&payer, &new_acc, lamports, 10, payer.pubkey())
        .await
        .unwrap();

    let acc = banks_client
        .get_account(new_acc.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(acc.lamports, lamports);
}

#[tokio::test]
async fn create_token_mint() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    //Create mint with defaults
    banks_client
        .create_token_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
        )
        .await
        .unwrap();
    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();
    let mint_data = Mint::unpack(&mint_acc.data).unwrap();
    assert_eq!(mint_data.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token::id());
}

#[tokio::test]
async fn create_token_account() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint_pubkey = Pubkey::new_unique();
    //Create mint with defaults
    program.add_token_mint(mint_pubkey, None, 10, 0, None);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    banks_client
        .create_token_account(&token_account, &payer.pubkey(), &mint_pubkey, &payer)
        .await
        .unwrap();

    let token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data = TokenAccount::unpack(&token_account.data).unwrap();

    assert_eq!(token_account_data.mint, mint_pubkey);
    assert_eq!(token_account_data.owner, payer.pubkey());
}

#[tokio::test]
async fn create_associated_token_account() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint_pubkey = Pubkey::new_unique();
    let token_program_id = spl_token::ID; //could also use token-2022 ID
    //Create mint with defaults
    program.add_token_mint(mint_pubkey, None, 10, 0, None);

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let token_account = banks_client
        .create_associated_token_account(&payer.pubkey(), &mint_pubkey, &payer, &token_program_id)
        .await
        .unwrap();

    let token_account = banks_client
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();

    let token_account_data = TokenAccount::unpack(&token_account.data).unwrap();

    assert_eq!(token_account_data.mint, mint_pubkey);
    assert_eq!(token_account_data.owner, payer.pubkey());
}

#[tokio::test]
async fn deploy_program() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);

    let program_keypair = Keypair::new();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    banks_client
        .deploy_program(
            "tests/artifacts/program_for_tests.so",
            &program_keypair,
            &payer,
        )
        .await
        .unwrap();
    let deployed_program_account = banks_client
        .get_account(program_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        deployed_program_account.owner,
        Pubkey::from_str("BPFLoader2111111111111111111111111111111111").unwrap()
    );
}

#[tokio::test]
async fn deploy_upgradable_program() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);

    let program_keypair = Keypair::new();
    let buffer_keypair = Keypair::new();
    let buffer_authority_signer = Keypair::new();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;
    banks_client
        .deploy_upgradable_program(
            "tests/artifacts/program_for_tests.so",
            &buffer_keypair,
            &buffer_authority_signer,
            &program_keypair,
            &payer,
        )
        .await
        .unwrap();
    let deployed_program_account = banks_client
        .get_account(program_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        deployed_program_account.owner,
        Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111").unwrap()
    );
}
