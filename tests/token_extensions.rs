use solana_test_framework::*;
use spl_token_2022::{
    extension::{
        default_account_state::DefaultAccountState, group_member_pointer::GroupMemberPointer,
        group_pointer::GroupPointer, immutable_owner::ImmutableOwner,
        interest_bearing_mint::InterestBearingConfig, memo_transfer::MemoTransfer,
        metadata_pointer::MetadataPointer, mint_close_authority::MintCloseAuthority,
        permanent_delegate::PermanentDelegate, transfer_fee::TransferFeeConfig,
        transfer_hook::TransferHook, BaseStateWithExtensions, StateWithExtensions,
        StateWithExtensionsMut,
    },
    state::AccountState,
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
async fn create_token2022_mint_with_group_account_ext() {
    let (mut program, _) = helpers::add_program();
    // It is necessary to use custom token-2022 program version, because the one used by solana-test-program
    // is outdated and does not have the support for token group extension
    program.prefer_bpf(true);
    program.add_program("token_2022", spl_token_2022::id(), None);
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let max_size = 100;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    let group_config = GroupConfig {
        update_authority: Some(payer.pubkey()),
        mint: mint.pubkey(),
        mint_authority: payer.pubkey(),
        max_size,
    };
    extensions.add_group_account(group_config);
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
    assert_eq!(group_pointer_ext.group_address.0, mint.pubkey());
    assert_eq!(group_pointer_ext.authority.0, payer.pubkey());

    // Comment out due to compatibility issues. Token group interface introduced breaking changes in 0.3.0
    // https://github.com/solana-labs/solana-program-library/pull/7130
    // However this version is not compatible with solana-program 1.18 as this introduces other dependencies that have
    // been yanked. So probably the only way would be to update solana-program to v2.0 which is out of scope now.
    // let group_ext = mint_data.get_extension::<TokenGroup>().unwrap();
    // assert_eq!(group_ext.update_authority.0, payer.pubkey());
    // assert_eq!(group_ext.mint, mint.pubkey());
    // assert_eq!(group_ext.max_size, max_size.into());

    // Only confirm that the group extension was initialized. Due to breaking changes in the
    // TokenGroup state account layout it cannot be deserialized via `get_extension::<TokenGroup>()`
    // and it would have to be done manually, which is not in scope of this test now
    let mut group_found = false;
    // Iterate through extensions and find TokenGroup manually
    for extension in mint_data.get_extension_types().ok().unwrap() {
        if extension == spl_token_2022::extension::ExtensionType::TokenGroup {
            group_found = true;
        }
    }
    assert!(group_found);
}

#[tokio::test]
async fn create_token2022_mint_with_member_pointer_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let member_address = Pubkey::new_unique();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    let member_pointer_config = MemberPointerConfig {
        update_authority: Some(payer.pubkey()),
        member_address,
    };
    extensions.add_member_pointer(member_pointer_config);

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

    let member_pointer_ext = mint_data.get_extension::<GroupMemberPointer>().unwrap();
    assert_eq!(member_pointer_ext.member_address.0, member_address);
    assert_eq!(member_pointer_ext.authority.0, payer.pubkey());
}

#[tokio::test]
async fn create_token2022_mint_with_member_account_ext() {
    let (mut program, _) = helpers::add_program();
    // It is necessary to use custom token-2022 program version, because the one used by solana-test-program
    // is outdated and does not have the support for token member extension
    program.prefer_bpf(true);
    program.add_program("token_2022", spl_token_2022::id(), None);
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let group_mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let max_size = 10;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    // First create a new mint with group extension
    let mut group_mint_extensions = MintExtensions::new();
    let group_config = GroupConfig {
        update_authority: Some(payer.pubkey()),
        mint: group_mint.pubkey(),
        mint_authority: payer.pubkey(),
        max_size,
    };
    group_mint_extensions.add_group_account(group_config);
    banks_client
        .create_token2022_mint(
            &group_mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&group_mint_extensions),
        )
        .await
        .unwrap();

    // Now create a new mint with the member extension. This mint will be a member of the group created above.
    let mut member_mint_extensions = MintExtensions::new();
    let member_config = MemberConfig {
        mint: mint.pubkey(),
        mint_authority: payer.pubkey(),
        group_update_authority: payer.pubkey(),
        group_address: group_mint.pubkey(),
    };
    member_mint_extensions.add_member_account(member_config);
    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&member_mint_extensions),
        )
        .await
        .unwrap();

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

    let member_pointer_ext = mint_data.get_extension::<GroupMemberPointer>().unwrap();
    assert_eq!(member_pointer_ext.member_address.0, mint.pubkey());
    assert_eq!(member_pointer_ext.authority.0, payer.pubkey());

    // Comment out due to compatibility issues. Token group interface introduced breaking changes in 0.3.0
    // https://github.com/solana-labs/solana-program-library/pull/7130
    // However this version is not compatible with solana-program 1.18 as this introduces other dependencies that have
    // been yanked. So probably the only way would be to update solana-program to v2.0 which is out of scope now.
    // let group_ext = mint_data.get_extension::<TokenGroupMember>().unwrap();

    // Only confirm that the member extension was initialized. Due to breaking changes in the
    // TokenGroupMember state account layout it cannot be deserialized via `get_extension::<TokenTokenGroupMemberoup>()`
    // and it would have to be done manually, which is not in scope of this test now
    let mut group_found = false;
    // Iterate through extensions and find TokenGroupMember manually
    for extension in mint_data.get_extension_types().ok().unwrap() {
        if extension == spl_token_2022::extension::ExtensionType::TokenGroupMember {
            group_found = true;
        }
    }
    assert!(group_found);
}

#[tokio::test]
async fn create_token2022_mint_with_transfer_hook_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;
    let transfer_hook_program_id = Pubkey::new_unique();

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut extensions = MintExtensions::new();
    extensions.add_transfer_hook(transfer_hook_program_id, Some(payer.pubkey()));

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

    let transfer_hook_ext = mint_data.get_extension::<TransferHook>().unwrap();
    assert_eq!(transfer_hook_ext.program_id.0, transfer_hook_program_id);
    assert_eq!(transfer_hook_ext.authority.0, payer.pubkey());
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

#[tokio::test]
async fn create_token2022_account_with_no_extensions() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
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

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            None,
        )
        .await
        .unwrap();

    let token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&token_account.data).unwrap();
    assert_eq!(token_account_data.base.mint, mint.pubkey());
    assert_eq!(token_account_data.base.owner, payer.pubkey());
    assert!(token_account_data.get_tlv_data().is_empty());
}

#[tokio::test]
async fn create_token2022_account_with_required_memo_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

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

    let mut extensions = TokenExtensions::new();
    extensions.add_memo_required_on_transfer();

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    let token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&token_account.data).unwrap();
    let token_data_base = token_account_data.base;
    assert_eq!(token_data_base.mint, mint.pubkey());

    let memo_ext = token_account_data.get_extension::<MemoTransfer>().unwrap();
    assert_eq!(memo_ext.require_incoming_transfer_memos, true.into());
}

#[tokio::test]
async fn create_token2022_account_with_immutable_owner_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

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

    let mut extensions = TokenExtensions::new();
    extensions.add_immutable_owner();

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    let token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&token_account.data).unwrap();
    let token_data_base = token_account_data.base;
    assert_eq!(token_data_base.mint, mint.pubkey());

    let _ = token_account_data
        .get_extension::<ImmutableOwner>()
        .unwrap();
}

#[tokio::test]
async fn create_token2022_account_with_cpi_guard_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

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

    let mut extensions = TokenExtensions::new();
    extensions.add_enable_cpi_guard();

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    let mut token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack(&mut token_account.data)
            .unwrap();
    let token_data_base = token_account_data.base;
    assert_eq!(token_data_base.mint, mint.pubkey());

    assert!(spl_token_2022::extension::cpi_guard::cpi_guard_enabled(
        &token_account_data
    ));
}

#[tokio::test]
async fn create_token2022_account_with_default_account_state_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

    let mut mint_extensions = MintExtensions::new();
    mint_extensions.add_default_account_state(AccountState::Frozen);

    banks_client
        .create_token2022_mint(
            &mint,
            &payer.pubkey(),
            Some(&freeze_pubkey),
            decimals,
            &payer,
            Some(&mint_extensions),
        )
        .await
        .unwrap();

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

    let account_state_ext = mint_data.get_extension::<DefaultAccountState>().unwrap();
    assert_eq!(account_state_ext.state, AccountState::Frozen as u8);

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            None,
        )
        .await
        .unwrap();

    let mut token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack(&mut token_account.data)
            .unwrap();
    let token_data_base = token_account_data.base;
    assert_eq!(token_data_base.state, AccountState::Frozen);
}

#[tokio::test]
async fn create_token2022_account_with_multiple_ext() {
    let (mut program, _) = helpers::add_program();
    let payer = helpers::add_payer(&mut program);
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let freeze_pubkey = Pubkey::new_unique();
    let decimals = 0;

    let (mut banks_client, _payer_keypair, mut _recent_blockhash) = program.start().await;

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

    let mut extensions = TokenExtensions::new();
    extensions.add_enable_cpi_guard();
    extensions.add_immutable_owner();
    extensions.add_memo_required_on_transfer();

    banks_client
        .create_token2022_account(
            &token_account,
            &payer.pubkey(),
            &mint.pubkey(),
            &payer,
            Some(&extensions),
        )
        .await
        .unwrap();

    let mut token_account = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let token_account_data =
        StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack(&mut token_account.data)
            .unwrap();
    let token_data_base = token_account_data.base;
    assert_eq!(token_data_base.mint, mint.pubkey());

    // Cpi guard ext
    assert!(spl_token_2022::extension::cpi_guard::cpi_guard_enabled(
        &token_account_data
    ));

    // Immutable owner ext
    let _ = token_account_data
        .get_extension::<ImmutableOwner>()
        .unwrap();

    // Memo ext
    let memo_ext = token_account_data.get_extension::<MemoTransfer>().unwrap();
    assert_eq!(memo_ext.require_incoming_transfer_memos, true.into());
}
