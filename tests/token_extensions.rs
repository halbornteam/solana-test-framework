use solana_test_framework::*;
use spl_token_2022::extension::{
    group_pointer::GroupPointer, interest_bearing_mint::InterestBearingConfig,
    metadata_pointer::MetadataPointer, mint_close_authority::MintCloseAuthority,
    permanent_delegate::PermanentDelegate, transfer_fee::TransferFeeConfig,
    BaseStateWithExtensions, StateWithExtensions,
};

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token_metadata_interface::state::TokenMetadata;

mod helpers;

#[tokio::test]
async fn create_token2022_mint_no_extensions() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            None,
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());
    assert!(mint_data.get_tlv_data().is_empty());
}

#[tokio::test]
async fn create_token2022_mint_with_close_authority_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let close_authority = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_mint_close_authority(close_authority.pubkey());

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let mint_close_auth_ext = mint_data.get_extension::<MintCloseAuthority>().unwrap();
    assert_eq!(
        mint_close_auth_ext.close_authority.0,
        close_authority.pubkey()
    );
}

#[tokio::test]
async fn create_token2022_mint_with_permanent_delegate_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let delegate = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_permanent_delegate(delegate.pubkey());

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let permanent_delegate_data = mint_data.get_extension::<PermanentDelegate>().unwrap();
    assert_eq!(permanent_delegate_data.delegate.0, delegate.pubkey());
}

#[tokio::test]
async fn create_token2022_mint_with_transfer_fee_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let fee_withdraw_authority = Pubkey::new_unique();
    let transfer_fee_config_authority = Pubkey::new_unique();
    let decimals = 0;
    let fee_basis_points = 20;
    let maximum_fee = 50000;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_transfer_fee(
        fee_basis_points,
        maximum_fee,
        Some(transfer_fee_config_authority),
        Some(fee_withdraw_authority),
    );

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let transfer_fee_ext = mint_data.get_extension::<TransferFeeConfig>().unwrap();
    assert_eq!(
        transfer_fee_ext
            .newer_transfer_fee
            .transfer_fee_basis_points,
        fee_basis_points.into()
    );
    assert_eq!(
        transfer_fee_ext.newer_transfer_fee.maximum_fee,
        maximum_fee.into()
    );
    assert_eq!(
        transfer_fee_ext.transfer_fee_config_authority.0,
        transfer_fee_config_authority
    );
    assert_eq!(
        transfer_fee_ext.withdraw_withheld_authority.0,
        fee_withdraw_authority
    );
}

#[tokio::test]
async fn create_token2022_mint_with_interest_bearing_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let rate_authority = Pubkey::new_unique();
    let rate = 500;
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_interest_bearing_mint(Some(rate_authority), rate);

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let interest_bearing_ext = mint_data.get_extension::<InterestBearingConfig>().unwrap();
    assert_eq!(interest_bearing_ext.rate_authority.0, rate_authority);
    assert_eq!(interest_bearing_ext.current_rate, rate.into());
}

#[tokio::test]
async fn create_token2022_mint_with_metadata_pointer_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let metadata_address = Pubkey::new_unique();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    let meta_data_pointer = TokenMetadataPointerConfig {
        update_authority: Some(payer.pubkey()),
        metadata_address,
    };
    extensions.add_metadata_pointer(meta_data_pointer);

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let metadata_pointer_ext = mint_data.get_extension::<MetadataPointer>().unwrap();
    assert_eq!(metadata_pointer_ext.metadata_address.0, metadata_address);
    assert_eq!(metadata_pointer_ext.authority.0, payer.pubkey());
}

#[tokio::test]
async fn create_token2022_mint_with_metadata_account_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let name = String::from("my new token");
    let symbol = String::from("MNT");
    let uri = String::from("some uri");
    let additional_metadata = vec![
        (
            String::from("some field name 1"),
            String::from("some custom value"),
        ),
        (
            String::from("some field name 2"),
            String::from("some custom value"),
        ),
    ];
    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    let metadata_config = TokenMetadataConfig {
        update_authority: Some(payer.pubkey()),
        mint: mint.pubkey(),
        mint_authority: payer.pubkey(),
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        additional_metadata: additional_metadata.clone(),
    };
    extensions.add_metadata_account(metadata_config);

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let metadata_pointer_ext = mint_data.get_extension::<MetadataPointer>().unwrap();
    assert_eq!(metadata_pointer_ext.metadata_address.0, mint.pubkey());
    assert_eq!(metadata_pointer_ext.authority.0, payer.pubkey());

    let mut metadata_found = false;
    // Iterate through extensions and find TokenMetadata manually
    for extension in mint_data.get_extension_types().ok().unwrap() {
        if extension == spl_token_2022::extension::ExtensionType::TokenMetadata {
            metadata_found = true;
            let metadata_ext = mint_data
                .get_variable_len_extension::<TokenMetadata>()
                .unwrap();
            assert_eq!(metadata_ext.update_authority.0, payer.pubkey());
            assert_eq!(metadata_ext.mint, mint.pubkey());
            assert_eq!(metadata_ext.name, name);
            assert_eq!(metadata_ext.symbol, symbol);
            assert_eq!(metadata_ext.uri, uri);
            assert_eq!(metadata_ext.additional_metadata, additional_metadata);
        }
    }
    assert!(metadata_found);

    // This is commented because the TokenMetadata extension does not implement the Extension trait in the early versions for Solana 1.18
    // An upgrade to Solana 2.0 is necessary
    // let metadata_ext = mint_data.get_extension::<TokenMetadata>().unwrap();
    // assert_eq!(metadata_ext.name, name);
    // assert_eq!(metadata_ext.symbol, symbol);
    // assert_eq!(metadata_ext.uri, uri);
}

#[tokio::test]
async fn create_token2022_mint_with_group_pointer_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let group_address = Pubkey::new_unique();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    let group_pointer_config = GroupPointerConfig {
        update_authority: Some(payer.pubkey()),
        group_address,
    };
    extensions.add_group_pointer(group_pointer_config);

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let group_pointer_ext = mint_data.get_extension::<GroupPointer>().unwrap();
    assert_eq!(group_pointer_ext.group_address.0, group_address);
    assert_eq!(group_pointer_ext.authority.0, payer.pubkey());
}

#[tokio::test]
async fn create_token2022_mint_with_multiple_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let close_authority = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let fee_withdraw_authority = Pubkey::new_unique();
    let transfer_fee_config_authority = Pubkey::new_unique();
    let decimals = 0;
    let fee_basis_points = 20;
    let maximum_fee = 50000;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_mint_close_authority(close_authority.pubkey());
    extensions.add_transfer_fee(
        fee_basis_points,
        maximum_fee,
        Some(transfer_fee_config_authority),
        Some(fee_withdraw_authority),
    );

    //Create mint with defaults
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    //Test mint with defaults creation
    let mint_acc = banks_client
        .get_account(mint.pubkey())
        .await
        .unwrap()
        .unwrap();

    let mint_data =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_acc.data).unwrap();
    let mint_data_base = mint_data.base;
    assert_eq!(mint_data_base.freeze_authority.unwrap(), freeze_pubkey);
    assert_eq!(mint_data_base.decimals, decimals);
    assert_eq!(mint_acc.owner, spl_token_2022::id());

    let mint_close_auth_ext = mint_data.get_extension::<MintCloseAuthority>().unwrap();
    assert_eq!(
        mint_close_auth_ext.close_authority.0,
        close_authority.pubkey()
    );
    let transfer_fee_ext = mint_data.get_extension::<TransferFeeConfig>().unwrap();
    assert_eq!(
        transfer_fee_ext
            .newer_transfer_fee
            .transfer_fee_basis_points,
        fee_basis_points.into()
    );
    assert_eq!(
        transfer_fee_ext.newer_transfer_fee.maximum_fee,
        maximum_fee.into()
    );
    assert_eq!(
        transfer_fee_ext.transfer_fee_config_authority.0,
        transfer_fee_config_authority
    );
    assert_eq!(
        transfer_fee_ext.withdraw_withheld_authority.0,
        fee_withdraw_authority
    );
}
