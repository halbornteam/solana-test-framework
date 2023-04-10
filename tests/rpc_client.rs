use solana_program::{native_token::sol_to_lamports, system_program};
use solana_test_framework::*;

use {
    solana_sdk::{
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction,
    },
    spl_token::state::Mint,
};

use solana_test_validator::{ProgramInfo, TestValidatorGenesis};

use spl_token::state::Account as TokenAccount;

use std::str::FromStr;

#[tokio::test(flavor = "multi_thread")]
async fn transaction_from_instructions() {
    let mut genesis_config = TestValidatorGenesis::default();
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program_path = "tests/artifacts/program_for_tests.so";

    genesis_config.add_programs_with_path(&[ProgramInfo {
        program_id,
        loader: solana_sdk::bpf_loader::id(),
        program_path: std::path::PathBuf::from(program_path),
    }]);

    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    let acc_1 = Keypair::new();
    let acc_2 = Keypair::new();
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
    let tx = rpc_client
        .transaction_from_instructions(&[ix_1, ix_2], &payer, vec![&payer, &acc_1, &acc_2])
        .await
        .unwrap();

    assert!(rpc_client.send_and_confirm_transaction(&tx).is_ok());
    let acc1_data = rpc_client.get_account(&acc_1.pubkey()).unwrap();
    let acc2_data = rpc_client.get_account(&acc_2.pubkey()).unwrap();
    assert_eq!(acc1_data.owner, acc_1.pubkey());
    assert_eq!(acc2_data.owner, acc_2.pubkey());
}

#[tokio::test(flavor = "multi_thread")]
async fn create_account() {
    let mut genesis_config = TestValidatorGenesis::default();
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program_path = "tests/artifacts/program_for_tests.so";

    genesis_config.add_programs_with_path(&[ProgramInfo {
        program_id,
        loader: solana_sdk::bpf_loader::id(),
        program_path: std::path::PathBuf::from(program_path),
    }]);

    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    let lamports = 1_000_000;
    let new_acc = Keypair::new();

    rpc_client
        .create_account(&payer, &new_acc, lamports, 10, payer.pubkey())
        .await
        .unwrap();

    let acc = rpc_client.get_account(&new_acc.pubkey()).unwrap();
    assert_eq!(acc.lamports, lamports);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn create_token_mint() {
    use tokio::time::{sleep, Duration};

    let mut genesis_config = TestValidatorGenesis::default();
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program_path = "tests/artifacts/program_for_tests.so";

    genesis_config.add_programs_with_path(&[ProgramInfo {
        program_id,
        loader: solana_sdk::bpf_loader::id(),
        program_path: std::path::PathBuf::from(program_path),
    }]);

    let (test_validator, payer) = genesis_config.start_async().await;

    let mut rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(
        test_validator.rpc_url(),
        solana_sdk::commitment_config::CommitmentConfig::finalized(),
    );

    let admin = Keypair::new();
    rpc_client
        .create_account(
            &payer,
            &admin,
            sol_to_lamports(100.0),
            0,
            system_program::id(),
        )
        .await
        .unwrap();

    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    sleep(Duration::from_millis(1000)).await;

    //Create mint with defaults
    rpc_client
        .create_token_mint(
            &mint,
            &admin.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &admin,
        )
        .await
        .unwrap();

    let hash = rpc_client.get_latest_blockhash().unwrap();
    rpc_client.get_new_latest_blockhash(&hash).unwrap();

    sleep(Duration::from_millis(100)).await;

    //Test mint with defaults creation
    let mint_acc = rpc_client
        .get_account_with_commitment(
            &mint.pubkey(),
            solana_sdk::commitment_config::CommitmentConfig::confirmed(),
        )
        .unwrap()
        .value
        .unwrap();

    println!("mint_acc: {:?}", mint_acc);

    let mint_data = Mint::unpack_unchecked(&mint_acc.data).unwrap();
    assert_eq!(mint_data.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token::id());
}

#[tokio::test(flavor = "multi_thread")]
async fn create_token_account() {
    let mut genesis_config = TestValidatorGenesis::default();
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program_path = "tests/artifacts/program_for_tests.so";

    genesis_config.add_programs_with_path(&[ProgramInfo {
        program_id,
        loader: solana_sdk::bpf_loader::id(),
        program_path: std::path::PathBuf::from(program_path),
    }]);

    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    //Create mint with defaults
    rpc_client
        .create_token_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
        )
        .await
        .unwrap();

    let token_account = Keypair::new();

    rpc_client
        .create_token_account(&token_account, &payer.pubkey(), &mint.pubkey(), &payer)
        .await
        .unwrap();

    let token_account = rpc_client.get_account(&token_account.pubkey()).unwrap();

    let token_account_data = TokenAccount::unpack(&token_account.data).unwrap();

    assert_eq!(token_account_data.mint, mint.pubkey());
    assert_eq!(token_account_data.owner, payer.pubkey());
}

#[tokio::test(flavor = "multi_thread")]
async fn create_associated_token_account() {
    let mut genesis_config = TestValidatorGenesis::default();
    let program_id = Pubkey::from_str("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr").unwrap();
    let program_path = "tests/artifacts/program_for_tests.so";
    let token_program_id = spl_token::ID; //could also use token-2022 ID

    genesis_config.add_programs_with_path(&[ProgramInfo {
        program_id,
        loader: solana_sdk::bpf_loader::id(),
        program_path: std::path::PathBuf::from(program_path),
    }]);

    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    //Create mint with defaults
    rpc_client
        .create_token_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
        )
        .await
        .unwrap();

    let token_account = rpc_client
        .create_associated_token_account(&payer.pubkey(), &mint.pubkey(), &payer, &token_program_id)
        .await
        .unwrap();

    let token_account = rpc_client.get_account(&token_account).unwrap();

    let token_account_data = TokenAccount::unpack(&token_account.data).unwrap();

    assert_eq!(token_account_data.mint, mint.pubkey());
    assert_eq!(token_account_data.owner, payer.pubkey());
}

#[tokio::test(flavor = "multi_thread")]
async fn deploy_program() {
    let genesis_config = TestValidatorGenesis::default();
    let program_keypair = Keypair::new();

    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    rpc_client
        .deploy_program(
            "tests/artifacts/program_for_tests.so",
            &program_keypair,
            &payer,
        )
        .await
        .unwrap();
    let deployed_program_account = rpc_client.get_account(&program_keypair.pubkey()).unwrap();

    assert_eq!(
        deployed_program_account.owner,
        Pubkey::from_str("BPFLoader2111111111111111111111111111111111").unwrap()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn deploy_upgradable_program() {
    let genesis_config = TestValidatorGenesis::default();
    let (test_validator, payer) = genesis_config.start_async().await;
    let mut rpc_client = test_validator.get_rpc_client();

    let program_keypair = Keypair::new();
    let buffer_keypair = Keypair::new();
    let buffer_authority_signer = Keypair::new();

    rpc_client
        .deploy_upgradable_program(
            "tests/artifacts/program_for_tests.so",
            &buffer_keypair,
            &buffer_authority_signer,
            &program_keypair,
            &payer,
        )
        .await
        .unwrap();
    let deployed_program_account = rpc_client.get_account(&program_keypair.pubkey()).unwrap();

    assert_eq!(
        deployed_program_account.owner,
        Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111").unwrap()
    );
}
