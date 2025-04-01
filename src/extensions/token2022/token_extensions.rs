use solana_sdk::{
    instruction::Instruction, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
};
use spl_token_2022::{
    extension::memo_transfer::instruction::enable_required_transfer_memos,
    extension::ExtensionType, instruction::initialize_immutable_owner,
};

#[derive(Default)]
pub struct TokenExtensions {
    memo_enabled: bool,
    immutable_owner: bool,
}

impl TokenExtensions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables the required memo extension.
    pub fn add_memo_required_on_transfer<'a>(&'a mut self) -> &'a mut TokenExtensions {
        self.memo_enabled = true;
        self
    }

    /// Enables the immutable owner extension.
    pub fn add_immutable_owner<'a>(&'a mut self) -> &'a mut TokenExtensions {
        self.immutable_owner = true;
        self
    }

    /// Calculates mint account data length with all added extensions.
    ///
    /// Fails if any of the extension types has a variable length
    pub fn try_calculate_token_account_length(&self) -> Result<usize, ProgramError> {
        let mut extension_types = Vec::new();
        if self.memo_enabled {
            extension_types.push(ExtensionType::MemoTransfer);
        }

        if self.immutable_owner {
            extension_types.push(ExtensionType::ImmutableOwner);
        }

        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(&extension_types)
    }

    /// Returns the minimal balance for rent exemption.
    pub fn get_minimal_balance_for_rent(&self) -> Result<u64, ProgramError> {
        let space = self.try_calculate_token_account_length()?;
        Ok(Rent::default().minimum_balance(space))
    }

    /// Returns vector of instructions to initialize all added extensions.
    /// These instructions must be invoked before the token account initialization.
    pub fn get_ixs_pre_token_init(
        &self,
        token_account: &Pubkey,
        _owner: &Pubkey,
        _signers: &[&Pubkey],
    ) -> Result<Vec<Instruction>, ProgramError> {
        let mut ixs = Vec::new();
        if self.immutable_owner {
            let ix = initialize_immutable_owner(&spl_token_2022::id(), token_account)?;
            ixs.push(ix);
        }
        Ok(ixs)
    }
    /// Returns vector in instructions to be invoked after the token account is created
    pub fn get_ixs_post_token_init(
        &self,
        token_account: &Pubkey,
        owner: &Pubkey,
        signers: &[&Pubkey],
    ) -> Result<Vec<Instruction>, ProgramError> {
        let mut ixs = Vec::new();

        if self.memo_enabled {
            let ix = enable_required_transfer_memos(
                &spl_token_2022::id(),
                token_account,
                owner,
                signers,
            )?;
            ixs.push(ix);
        }
        Ok(ixs)
    }
}
