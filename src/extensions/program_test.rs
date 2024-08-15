use borsh::BorshSerialize;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use log::info;
use solana_program::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    program_option::COption,
    program_pack::Pack,
};
use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_sdk::{
    account::Account,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    sysvar::rent::Rent,
};
use spl_associated_token_account::get_associated_token_address;

#[cfg(feature = "anchor")]
use anchor_lang::{AnchorSerialize, Discriminator};

#[cfg(feature = "pyth")]
use {
    crate::util::PriceAccountWrapper,
    pyth_sdk_solana::state::{PriceAccount, PriceInfo},
    solana_program_test::BanksClientError,
};

pub trait ProgramTestExtension {
    /// Adds a requested number of account with initial balance of 1_000 SOL to the test environment
    fn generate_accounts(&mut self, number_of_accounts: u8) -> Vec<Keypair>;

    /// Add a rent-exempt account with some data to the test environment.
    fn add_account_with_data(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: &[u8],
        executable: bool,
    );

    #[cfg(feature = "anchor")]
    /// Adds an Anchor account.
    fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        anchor_data: T,
        executable: bool,
    );

    #[cfg(feature = "anchor")]
    /// Adds an empty anchor account with a discriminator and specified size.
    fn add_empty_account_with_anchor<T: AnchorSerialize + Discriminator>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        size: usize,
    );

    /// Adds an account with the given balance to the test environment.
    fn add_account_with_lamports(&mut self, pubkey: Pubkey, owner: Pubkey, lamports: u64);

    /// Adds a rent-exempt account with some Packable data to the test environment.
    fn add_account_with_packable<P: Pack>(&mut self, pubkey: Pubkey, owner: Pubkey, data: P);

    /// Adds a rent-exempt account with some Borsh-serializable to the test environment
    fn add_account_with_borsh<B: BorshSerialize>(&mut self, pubkey: Pubkey, owner: Pubkey, data: B);

    /// Adds an SPL Token Mint account to the test environment.
    fn add_token_mint(
        &mut self,
        pubkey: Pubkey,
        mint_authority: Option<Pubkey>,
        supply: u64,
        decimals: u8,
        freeze_authority: Option<Pubkey>,
    );

    /// Adds an SPL Token account to the test environment.
    fn add_token_account(
        &mut self,
        pubkey: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    );

    /// Adds an associated token account to the test environment.
    /// Returns the address of the created account.
    fn add_associated_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey;

    /// Adds a BPF program to the test environment.
    /// The program is upgradeable if `Some` `program_authority` is provided.
    fn add_bpf_program(
        &mut self,
        program_name: &str,
        program_id: Pubkey,
        program_authority: Option<Pubkey>,
        process_instruction: Option<BuiltinFunctionWithContext>,
    );

    /// Adds a BPF program to the test environment.
    /// The program is upgradeable if `Some` `program_authority` and then providing the  program data account
    /// This is useful for those programs which the program data has to be a spefic one, if not, use add_bpf_program
    fn add_bpf_program_with_program_data(
        &mut self,
        program_name: &str,
        program_id: Pubkey,
        program_authority: Option<Pubkey>,
        program_data: Pubkey,
        upgrade_slot: u64,
        process_instruction: Option<BuiltinFunctionWithContext>,
    );

    #[cfg(feature = "pyth")]
    /// Adds a Pyth oracle to the test environment.
    fn add_pyth_oracle(
        &mut self,
        oracle: Pubkey,
        program_id: Pubkey,
        price_account: Option<PriceAccount>,
        price_info: Option<PriceInfo>,
        timestamp: Option<i64>,
    ) -> Result<(), BanksClientError>;
}

impl ProgramTestExtension for ProgramTest {
    fn generate_accounts(&mut self, number_of_accounts: u8) -> Vec<Keypair> {
        let mut accounts: Vec<Keypair> = vec![];

        for _ in 0..number_of_accounts {
            let keypair = Keypair::new();
            let initial_lamports = sol_to_lamports(1_000.0);
            self.add_account_with_lamports(keypair.pubkey(), keypair.pubkey(), initial_lamports);
            accounts.push(keypair);
        }
        accounts
    }

    fn add_account_with_data(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: &[u8],
        executable: bool,
    ) {
        self.add_account(
            pubkey,
            Account {
                lamports: Rent::default().minimum_balance(data.len()),
                data: data.to_vec(),
                executable,
                owner,
                rent_epoch: 0,
            },
        );
    }

    #[cfg(feature = "anchor")]
    fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        anchor_data: T,
        executable: bool,
    ) {
        let discriminator = &T::discriminator();
        let data = anchor_data
            .try_to_vec()
            .expect("Cannot serialize provided anchor account");
        let mut v = Vec::new();
        v.extend_from_slice(discriminator);
        v.extend_from_slice(&data);
        self.add_account_with_data(pubkey, owner, &v, executable);
    }

    //Note that the total size is 8 (disciminator) + size
    #[cfg(feature = "anchor")]
    fn add_empty_account_with_anchor<T: AnchorSerialize + Discriminator>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        size: usize,
    ) {
        let discriminator = &T::discriminator();
        let data = vec![0_u8; size];
        let mut v = Vec::new();
        v.extend_from_slice(discriminator);
        v.extend_from_slice(&data);
        self.add_account_with_data(pubkey, owner, &v, false);
    }

    fn add_account_with_lamports(&mut self, pubkey: Pubkey, owner: Pubkey, lamports: u64) {
        self.add_account(
            pubkey,
            Account {
                lamports,
                data: vec![],
                executable: false,
                owner,
                rent_epoch: 0,
            },
        );
    }

    fn add_account_with_packable<P: Pack>(&mut self, pubkey: Pubkey, owner: Pubkey, data: P) {
        let data = {
            let mut buf = vec![0u8; P::LEN];
            data.pack_into_slice(&mut buf[..]);
            buf
        };
        self.add_account_with_data(pubkey, owner, &data, false);
    }

    fn add_account_with_borsh<B: BorshSerialize>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: B,
    ) {
        self.add_account_with_data(
            pubkey,
            owner,
            data.try_to_vec()
                .expect("failed to serialize data")
                .as_ref(),
            false,
        );
    }

    fn add_token_mint(
        &mut self,
        pubkey: Pubkey,
        mint_authority: Option<Pubkey>,
        supply: u64,
        decimals: u8,
        freeze_authority: Option<Pubkey>,
    ) {
        self.add_account_with_packable(
            pubkey,
            spl_token::id(),
            spl_token::state::Mint {
                mint_authority: COption::from(mint_authority),
                supply,
                decimals,
                is_initialized: true,
                freeze_authority: COption::from(freeze_authority),
            },
        );
    }

    fn add_token_account(
        &mut self,
        pubkey: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) {
        self.add_account_with_packable(
            pubkey,
            spl_token::id(),
            spl_token::state::Account {
                mint,
                owner,
                amount,
                delegate: COption::from(delegate),
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::from(is_native),
                delegated_amount,
                close_authority: COption::from(close_authority),
            },
        );
    }

    fn add_associated_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        let pubkey = get_associated_token_address(&owner, &mint);
        self.add_token_account(
            pubkey,
            mint,
            owner,
            amount,
            delegate,
            is_native,
            delegated_amount,
            close_authority,
        );

        pubkey
    }

    fn add_bpf_program(
        &mut self,
        program_name: &str,
        program_id: Pubkey,
        program_authority: Option<Pubkey>,
        process_instruction: Option<BuiltinFunctionWithContext>,
    ) {
        if let Some(program_authority) = program_authority {
            let program_file =
                solana_program_test::find_file(&format!("{}.so", program_name)).unwrap();
            let program_bytes = solana_program_test::read_file(program_file.clone());

            let program_data_pubkey = Pubkey::new_unique();

            let mut program = Vec::<u8>::new();
            bincode::serialize_into(
                &mut program,
                &UpgradeableLoaderState::Program {
                    programdata_address: program_data_pubkey,
                },
            )
            .unwrap();

            let mut program_data = Vec::<u8>::new();
            bincode::serialize_into(
                &mut program_data,
                &UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(program_authority),
                },
            )
            .unwrap();

            info!(
                "\"{}\" BPF program from {}{}",
                program_name,
                program_file.display(),
                std::fs::metadata(&program_file)
                    .map(|metadata| {
                        metadata
                            .modified()
                            .map(|time| {
                                format!(
                                    ", modified {}",
                                    HumanTime::from(time)
                                        .to_text_en(Accuracy::Precise, Tense::Past)
                                )
                            })
                            .ok()
                    })
                    .ok()
                    .flatten()
                    .unwrap_or_default()
            );

            self.add_account_with_data(
                program_id,
                bpf_loader_upgradeable::id(),
                program.as_ref(),
                true,
            );

            self.add_account_with_data(
                program_data_pubkey,
                bpf_loader_upgradeable::id(),
                &[program_data.as_slice(), program_bytes.as_slice()].concat(),
                false,
            );
        } else {
            self.add_program(program_name, program_id, process_instruction);
        }
    }

    fn add_bpf_program_with_program_data(
        &mut self,
        program_name: &str,
        program_id: Pubkey,
        program_authority: Option<Pubkey>,
        program_data_pubkey: Pubkey,
        upgrade_slot: u64,
        process_instruction: Option<BuiltinFunctionWithContext>,
    ) {
        if let Some(program_authority) = program_authority {
            let program_file =
                solana_program_test::find_file(&format!("{}.so", program_name)).unwrap();
            let program_bytes = solana_program_test::read_file(program_file.clone());

            let mut program = Vec::<u8>::new();
            bincode::serialize_into(
                &mut program,
                &UpgradeableLoaderState::Program {
                    programdata_address: program_data_pubkey,
                },
            )
            .unwrap();

            let mut program_data = Vec::<u8>::new();
            bincode::serialize_into(
                &mut program_data,
                &UpgradeableLoaderState::ProgramData {
                    slot: upgrade_slot,
                    upgrade_authority_address: Some(program_authority),
                },
            )
            .unwrap();

            info!(
                "\"{}\" BPF program from {}{}",
                program_name,
                program_file.display(),
                std::fs::metadata(&program_file)
                    .map(|metadata| {
                        metadata
                            .modified()
                            .map(|time| {
                                format!(
                                    ", modified {}",
                                    HumanTime::from(time)
                                        .to_text_en(Accuracy::Precise, Tense::Past)
                                )
                            })
                            .ok()
                    })
                    .ok()
                    .flatten()
                    .unwrap_or_default()
            );

            self.add_account_with_data(
                program_id,
                bpf_loader_upgradeable::id(),
                program.as_ref(),
                true,
            );

            self.add_account_with_data(
                program_data_pubkey,
                bpf_loader_upgradeable::id(),
                &[program_data.as_slice(), program_bytes.as_slice()].concat(),
                false,
            );
        } else {
            self.add_program(program_name, program_id, process_instruction);
        }
    }

    #[cfg(feature = "pyth")]
    /// Adds a Pyth oracle to the test environment.
    fn add_pyth_oracle(
        &mut self,
        oracle: Pubkey,
        program_id: Pubkey,
        price_account: Option<PriceAccount>,
        price_info: Option<PriceInfo>,
        timestamp: Option<i64>,
    ) -> Result<(), BanksClientError> {
        let data = if let Some(price_account) = price_account {
            bincode::serialize(&PriceAccountWrapper(&price_account)).unwrap()
        } else if let (Some(price_info), Some(timestamp)) = (price_info, timestamp) {
            bincode::serialize(&PriceAccountWrapper(
                &pyth_sdk_solana::state::PriceAccount {
                    magic: 0xa1b2c3d4,
                    ver: 2,
                    expo: 5,
                    atype: 3,
                    agg: price_info,
                    timestamp,
                    prev_timestamp: 100,
                    prev_price: 60,
                    prev_conf: 70,
                    prev_slot: 1,
                    ..Default::default()
                },
            ))
            .unwrap()
        } else {
            return Err(BanksClientError::ClientError(
                "Either provide the price_account or price_info and time_stamp",
            ));
        };

        self.add_account_with_data(oracle, program_id, &data, false);

        Ok(())
    }
}
