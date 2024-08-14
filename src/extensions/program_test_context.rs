use async_trait::async_trait;
use solana_program_test::{ProgramTestContext, ProgramTestError};
use solana_sdk::sysvar::clock::Clock;

#[cfg(feature = "pyth")]
use {
    crate::error::TestFrameWorkError,
    crate::util::PriceAccountWrapper,
    pyth_sdk_solana::state::{PriceAccount, PriceInfo},
};

#[async_trait]
pub trait ProgramTestContextExtension {
    /// Calculate slot number from the provided timestamp
    async fn warp_to_timestamp(&mut self, timestamp: i64) -> Result<(), ProgramTestError>;

    #[cfg(feature = "pyth")]
    async fn update_pyth_oracle(
        &mut self,
        address: Pubkey,
        price_account: Option<PriceAccount>,
        price_info: Option<PriceInfo>,
        timestamp: Option<i64>,
        valid_slots: Option<u64>,
    ) -> Result<(), TestFrameWorkError>;
}

#[async_trait]
impl ProgramTestContextExtension for ProgramTestContext {
    async fn warp_to_timestamp(&mut self, timestamp: i64) -> Result<(), ProgramTestError> {
        const NANOSECONDS_IN_SECOND: i64 = 1_000_000_000;

        let mut clock: Clock = self.banks_client.get_sysvar().await.unwrap();
        let now = clock.unix_timestamp;
        let current_slot = clock.slot;
        clock.unix_timestamp = timestamp;

        if now >= timestamp {
            println!("Timestamp incorrect. Cannot set time backwards.");
            return Err(ProgramTestError::InvalidWarpSlot);
        }

        let ns_per_slot = self.genesis_config().ns_per_slot();
        let timestamp_diff_ns = timestamp
            .checked_sub(now) //calculate time diff
            .expect("Problem with timestamp diff calculation.")
            .checked_mul(NANOSECONDS_IN_SECOND) //convert from s to ns
            .expect("Problem with timestamp diff calculation.")
            as u128;

        let slots = timestamp_diff_ns
            .checked_div(ns_per_slot)
            .expect("Problem with slots from timestamp calculation.") as u64;

        self.set_sysvar(&clock);
        self.warp_to_slot(current_slot + slots)?;

        Ok(())
    }

    #[cfg(feature = "pyth")]
    async fn update_pyth_oracle(
        &mut self,
        address: Pubkey,
        price_account: Option<PriceAccount>,
        price_info: Option<PriceInfo>,
        timestamp: Option<i64>,
        valid_slot: Option<u64>,
    ) -> Result<(), TestFrameWorkError> {
        let mut account = self
            .banks_client
            .get_account(address)
            .await
            .unwrap()
            .unwrap();

        let data = if let Some(price_account) = price_account {
            bincode::serialize(&PriceAccountWrapper(&price_account)).unwrap()
        } else if let (Some(price_info), Some(timestamp), Some(valid_slot)) =
            (price_info, timestamp, valid_slot)
        {
            let mut account_data =
                *pyth_sdk_solana::state::load_price_account(&account.data).unwrap();
            account_data.agg = price_info;
            account_data.timestamp = timestamp;
            account_data.valid_slot = valid_slot;

            bincode::serialize(&PriceAccountWrapper(&account_data)).unwrap()
        } else {
            return Err(TestFrameWorkError::Error(
                "Either provide the price_account or price_info, time_stamp and prev_slot",
            ));
        };

        account.data = data;
        let account = AccountSharedData::from(account);

        self.set_account(&address, &account);

        Ok(())
    }
}
