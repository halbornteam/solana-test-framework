use async_trait::async_trait;
use borsh::BorshDeserialize;
use futures::FutureExt;
use solana_program::{bpf_loader_upgradeable, program_pack::Pack};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_transaction,
    sysvar::rent::Rent,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account as create_associated_token_account_ix,
};

#[cfg(feature = "anchor")]
use anchor_lang::AccountDeserialize;

pub use solana_banks_client::{BanksClient, BanksClientError};

mod banks_client;
mod rpc_client;

#[allow(unused_imports)]
pub use banks_client::*;
#[allow(unused_imports)]
pub use rpc_client::*;

use crate::util;

#[cfg(feature = "pyth")]
use pyth_sdk_solana::state::SolanaPriceAccount;

/// Convenience functions for clients
#[async_trait]
pub trait ClientExtensions {
    /// Assemble the given instructions into a transaction and sign it.
    /// All transactions created with this method are signed and payed for by the payer.
    async fn transaction_from_instructions(
        &mut self,
        _ixs: &[Instruction],
        _payer: &Keypair,
        _signers: Vec<&Keypair>,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Return and deserialize an Anchor account at the given address at the time of the most recent root slot.
    /// If the account is not found, `None` is returned.
    #[cfg(feature = "anchor")]
    async fn get_account_with_anchor<T: AccountDeserialize>(
        &mut self,
        _address: Pubkey,
    ) -> Result<T, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Return and deserialize a Borsh account at the given address at the time of the most recent root slot.
    /// If the account is not `found`, None is returned.
    async fn get_account_with_borsh<T: BorshDeserialize>(
        &mut self,
        _address: Pubkey,
    ) -> Result<T, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    #[cfg(feature = "pyth")]
    async fn get_pyth_price_account(
        &mut self,
        _address: Pubkey,
    ) -> Result<SolanaPriceAccount, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Create a new account
    async fn create_account(
        &mut self,
        _from: &Keypair,
        _to: &Keypair,
        _lamports: u64,
        _space: u64,
        _owner: Pubkey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Create a new SPL Token Mint account
    async fn create_token_mint(
        &mut self,
        _mint: &Keypair,
        _authority: &Pubkey,
        _freeze_authority: Option<&Pubkey>,
        _decimals: u8,
        _payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Create a new SPL Token Account
    async fn create_token_account(
        &mut self,
        _account: &Keypair,
        _authority: &Pubkey,
        _mint: &Pubkey,
        _payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Create a new SPL Associated Token Account
    async fn create_associated_token_account(
        &mut self,
        _authority: &Pubkey,
        _mint: &Pubkey,
        _payer: &Keypair,
        _token_program_id: &Pubkey,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    /// Deploy an upgradable program
    async fn deploy_upgradable_program(
        &mut self,
        _path_to_program: &str,
        _buffer_keypair: &Keypair,
        _buffer_authority_signer: &Keypair,
        _program_keypair: &Keypair,
        _payer: &Keypair,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!();
    }
}
