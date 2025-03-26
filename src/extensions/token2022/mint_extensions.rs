use solana_sdk::{instruction::Instruction, program_error::ProgramError, pubkey::Pubkey};
use spl_token_2022::{
    extension::interest_bearing_mint::instruction::initialize as initialize_interest_bearing_mint_config,
    extension::{transfer_fee::instruction::initialize_transfer_fee_config, ExtensionType},
    instruction::initialize_mint_close_authority,
    instruction::initialize_non_transferable_mint,
    instruction::initialize_permanent_delegate,
};

#[derive(Default)]
pub struct MintExtensions {
    mint_close_authority: Option<Pubkey>,
    transfer_fee: Option<InitializeTransferFeeConfig>,
    interest_bearing: Option<InitializeInterestBearingConfig>,
    non_transferable: Option<()>,
    permanent_delegate: Option<Pubkey>,
    // metadata_pointer / metadata
    // group_pointer / group
    // member_pointer / member
    // scaled_ui_amount
    // pausable
    // transfer_hook
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
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extension_types)
    }

    /// Returns vector of instructions to initialize all added extensions.
    /// These instructions must be invoked before the mint account initialization.
    pub fn get_init_ixs(&self, mint: &Pubkey) -> Result<Vec<Instruction>, ProgramError> {
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

        Ok(ixs)
    }
}
