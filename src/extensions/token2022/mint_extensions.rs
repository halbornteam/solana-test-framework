use solana_sdk::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
};
use spl_token_2022::{
    extension::group_pointer::instruction::initialize as initialize_group_pointer,
    extension::interest_bearing_mint::instruction::initialize as initialize_interest_bearing_mint_config,
    extension::metadata_pointer::instruction::initialize as initialize_metadata_pointer,
    extension::{transfer_fee::instruction::initialize_transfer_fee_config, ExtensionType},
    instruction::initialize_mint_close_authority,
    instruction::initialize_non_transferable_mint,
    instruction::initialize_permanent_delegate,
};
use spl_token_metadata_interface::instruction::update_field;
use spl_token_metadata_interface::state::TokenMetadata;
use spl_token_metadata_interface::{
    instruction::initialize as initialize_metadata_account, state::Field,
};

#[derive(Default)]
pub struct MintExtensions {
    mint_close_authority: Option<Pubkey>,
    transfer_fee: Option<InitializeTransferFeeConfig>,
    interest_bearing: Option<InitializeInterestBearingConfig>,
    non_transferable: Option<()>,
    permanent_delegate: Option<Pubkey>,
    metadata_pointer: Option<TokenMetadataPointerConfig>,
    metadata: Option<TokenMetadataConfig>,
    group_pointer: Option<GroupPointerConfig>,
    // group_pointer / group
    // member_pointer / member
    // transfer_hook
    //
    // Introduced in spl-token-2022 v7.0.0 (solana 2.*)
    // pausable
    // scaled_ui_amount
}

struct InitializeTransferFeeConfig {
    /// Pubkey that may update the fees
    transfer_fee_config_authority: Option<Pubkey>,
    /// Withdraw instructions must be signed by this key
    withdraw_withheld_authority: Option<Pubkey>,
    /// Amount of transfer collected as fees, expressed as basis points of
    /// the transfer amount
    transfer_fee_basis_points: u16,
    /// Maximum fee assessed on transfers
    maximum_fee: u64,
}

struct InitializeInterestBearingConfig {
    /// Optional designated rate authority
    rate_authority: Option<Pubkey>,
    /// Interest rate basis points
    rate: i16,
}

pub struct TokenMetadataPointerConfig {
    pub update_authority: Option<Pubkey>,
    pub metadata_address: Pubkey,
}

pub struct TokenMetadataConfig {
    /// The authority that can sign to update the metadata
    pub update_authority: Option<Pubkey>,
    /// The associated mint, used to counter spoofing to be sure that metadata
    /// belongs to a particular mint
    pub mint: Pubkey,
    /// The associated mint authority
    pub mint_authority: Pubkey,
    /// The longer name of the token
    pub name: String,
    /// The shortened symbol for the token
    pub symbol: String,
    /// The URI pointing to richer metadata
    pub uri: String,
    /// Any additional metadata about the token as key-value pairs. The program
    /// must avoid storing the same key twice.
    pub additional_metadata: Vec<(String, String)>,
}

pub struct GroupPointerConfig {
    pub update_authority: Option<Pubkey>,
    pub group_address: Pubkey,
}

impl MintExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds mint close authority extension
    ///
    /// - `close_authority`: The authority that can close the mint if supply is zero.
    pub fn add_mint_close_authority<'a>(
        &'a mut self,
        close_authority: Pubkey,
    ) -> &'a mut MintExtensions {
        self.mint_close_authority = Some(close_authority);
        self
    }

    /// Adds permanent delegate extension
    ///
    /// - `close_authority`: The authority that can close the mint if supply is zero.
    pub fn add_permanent_delegate<'a>(&'a mut self, delegate: Pubkey) -> &'a mut MintExtensions {
        self.permanent_delegate = Some(delegate);
        self
    }

    /// Adds transfer fee extension.
    ///
    /// - `transfer_fee_config_authority`: Pubkey that may update the fees
    /// - `withdraw_withheld_authority`: Withdraw instructions must be signed by this key
    /// - `transfer_fee_basis_points`: Amount of transfer collected as fees, expressed as basis points of the transfer amount
    /// - `maximum_fee`: Maximum fee assessed on transfers
    pub fn add_transfer_fee<'a>(
        &'a mut self,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
        transfer_fee_config_authority: Option<Pubkey>,
        withdraw_withheld_authority: Option<Pubkey>,
    ) -> &'a mut MintExtensions {
        self.transfer_fee = Some(InitializeTransferFeeConfig {
            transfer_fee_config_authority,
            withdraw_withheld_authority,
            transfer_fee_basis_points,
            maximum_fee,
        });
        self
    }

    /// Adds interest bearing mint extension
    ///
    /// - `rate_authority`: The public key for the account that can update the rate
    /// - `rate`: The initial interest rate
    pub fn add_interest_bearing_mint<'a>(
        &'a mut self,
        rate_authority: Option<Pubkey>,
        rate: i16,
    ) -> &'a mut MintExtensions {
        self.interest_bearing = Some(InitializeInterestBearingConfig {
            rate_authority,
            rate,
        });
        self
    }

    /// Adds metadata pointer extension. This extension points to an external metadata account.
    /// To use the mint as metadata account, use the `add_metadata_account` method.
    ///
    /// - `meta_data_pointer`: Contains information about the meta data pointer configuration
    pub fn add_metadata_pointer<'a>(
        &'a mut self,
        meta_data_pointer_config: TokenMetadataPointerConfig,
    ) -> &'a mut MintExtensions {
        // Set the metadata pointer to the mint account itself
        self.metadata_pointer = Some(meta_data_pointer_config);
        self
    }

    /// Adds metadata account extension. This extension extends the mint account and adds the metadata directly into the mint.
    ///
    /// - `meta_data`: Contains information about the meta data account
    pub fn add_metadata_account<'a>(
        &'a mut self,
        meta_data_config: TokenMetadataConfig,
    ) -> &'a mut MintExtensions {
        // Set the metadata pointer to the mint account itself
        self.metadata_pointer = Some(TokenMetadataPointerConfig {
            update_authority: meta_data_config.update_authority,
            metadata_address: meta_data_config.mint,
        });
        self.metadata = Some(meta_data_config);
        self
    }

    /// Adds group pointer extension. This extension points to an external group account.
    /// To use the mint as metadata account, use the `add_group_account` method.
    ///
    /// - `group_pointer_config`: Contains information about the group pointer configuration
    pub fn add_group_pointer<'a>(
        &'a mut self,
        group_pointer_config: GroupPointerConfig,
    ) -> &'a mut MintExtensions {
        // Set the metadata pointer to the mint account itself
        self.group_pointer = Some(group_pointer_config);
        self
    }

    /// Calculates mint account data length with all added extensions.
    ///
    /// Fails if any of the extension types has a variable length
    pub fn try_calculate_mint_account_length(&self) -> Result<usize, ProgramError> {
        let mut extension_types = Vec::new();
        if let Some(_) = self.mint_close_authority {
            extension_types.push(ExtensionType::MintCloseAuthority);
        }
        if let Some(_) = self.transfer_fee {
            extension_types.push(ExtensionType::TransferFeeConfig);
        }
        if let Some(_) = self.permanent_delegate {
            extension_types.push(ExtensionType::PermanentDelegate);
        }
        if let Some(_) = self.non_transferable {
            extension_types.push(ExtensionType::NonTransferable);
        }
        if let Some(_) = self.interest_bearing {
            extension_types.push(ExtensionType::InterestBearingConfig);
        }
        if let Some(_) = self.metadata_pointer {
            extension_types.push(ExtensionType::MetadataPointer);
        }
        if let Some(_) = self.group_pointer {
            extension_types.push(ExtensionType::GroupPointer);
        }
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extension_types)
    }

    /// Returns the minimal balance for rent exemption.
    /// This method also adds necessary lamports if the mint will contain token metadata.
    pub fn get_minimal_balance_for_rent(&self) -> Result<u64, ProgramError> {
        let mut space = self.try_calculate_mint_account_length()?;
        if let Some(ref metadata_config) = self.metadata {
            space += TokenMetadata {
                update_authority: metadata_config.update_authority.clone().try_into()?,
                mint: metadata_config.mint,
                name: metadata_config.name.clone(),
                symbol: metadata_config.symbol.clone(),
                uri: metadata_config.uri.clone(),
                additional_metadata: metadata_config.additional_metadata.clone(),
            }
            .tlv_size_of()?
                + 4; // Size of MetadataExtension 2 bytes for type, 2 bytes for length
        }
        Ok(Rent::default().minimum_balance(space))
    }

    /// Returns vector of instructions to initialize all added extensions.
    /// These instructions must be invoked before the mint account initialization.
    pub fn get_ixs_pre_mint(&self, mint: &Pubkey) -> Result<Vec<Instruction>, ProgramError> {
        let mut ixs = Vec::new();

        if let Some(close_authority) = self.mint_close_authority {
            let ix = initialize_mint_close_authority(
                &spl_token_2022::id(),
                mint,
                Some(&close_authority),
            )?;
            ixs.push(ix);
        }

        if let Some(ref transfer_fee_config) = self.transfer_fee {
            let ix = initialize_transfer_fee_config(
                &spl_token_2022::id(),
                mint,
                transfer_fee_config.transfer_fee_config_authority.as_ref(),
                transfer_fee_config.withdraw_withheld_authority.as_ref(),
                transfer_fee_config.transfer_fee_basis_points,
                transfer_fee_config.maximum_fee,
            )?;
            ixs.push(ix);
        }

        if let Some(ref interest_bearing_config) = self.interest_bearing {
            let ix = initialize_interest_bearing_mint_config(
                &spl_token_2022::id(),
                mint,
                interest_bearing_config.rate_authority,
                interest_bearing_config.rate,
            )?;
            ixs.push(ix);
        }

        if let Some(delegate) = self.permanent_delegate {
            let ix = initialize_permanent_delegate(&spl_token_2022::id(), mint, &delegate)?;
            ixs.push(ix);
        }

        if let Some(_) = self.non_transferable {
            let ix = initialize_non_transferable_mint(&spl_token_2022::id(), mint)?;
            ixs.push(ix);
        }

        if let Some(ref metadata_pointer_config) = self.metadata_pointer {
            let ix = initialize_metadata_pointer(
                &spl_token_2022::id(),
                mint,
                metadata_pointer_config.update_authority,
                Some(metadata_pointer_config.metadata_address),
            )?;
            ixs.push(ix);
        }

        if let Some(ref group_pointer_config) = self.group_pointer {
            let ix = initialize_group_pointer(
                &spl_token_2022::id(),
                mint,
                group_pointer_config.update_authority,
                Some(group_pointer_config.group_address),
            )?;
            ixs.push(ix);
        }

        Ok(ixs)
    }
    /// Returns vector in instructions to be invoked after the mint account is created
    pub fn get_ixs_post_mint(&self, mint: &Pubkey) -> Result<Vec<Instruction>, ProgramError> {
        let mut ixs = Vec::new();

        // In theory the mint could have the metadata and at the same time point to different external metadata account,
        // so we will not verify if self.metadata_pointer corresponds to the mint pubkey and if it is set or not.
        // However the current implementation does not allow to set mint metadata and then point the pointer to different account.
        // This could be eveventually achieved later after the mint creation but then we do not care anymore.
        if let Some(ref metadata_config) = self.metadata {
            let ix = initialize_metadata_account(
                &spl_token_2022::id(),
                &mint,
                &metadata_config.update_authority.unwrap_or_default(),
                &mint,
                &metadata_config.mint_authority,
                metadata_config.name.clone(),
                metadata_config.symbol.clone(),
                metadata_config.uri.clone(),
            );
            ixs.push(ix);
            let mut custom_metadata_ixs: Vec<_> = metadata_config
                .additional_metadata
                .iter()
                .map(|(field, value)| {
                    update_field(
                        &spl_token_2022::id(),
                        &mint,
                        &metadata_config.update_authority.unwrap_or_default(),
                        Field::Key(field.clone()),
                        value.clone(),
                    )
                })
                .collect();
            ixs.append(&mut custom_metadata_ixs);
        }

        Ok(ixs)
    }
}
