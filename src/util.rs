use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    packet::PACKET_DATA_SIZE,
    signature::Signer,
    transaction::Transaction,
    
};
use std::fs::{metadata, File};
use std::io::Read;

#[cfg(feature = "pyth")]
use {
    pyth_sdk_solana::state::{ PriceType, Rational, PriceAccount, PriceInfo, PriceComp},
    solana_sdk::pubkey::Pubkey,
    serde::{Serialize, Deserialize},
};


pub fn load_file_to_bytes(filename: &str) -> (Vec<u8>, usize) {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return (buffer, metadata.len() as usize);
}

pub fn calculate_chunk_size<F: Fn(u32, Vec<u8>) -> Instruction>(
    deploy_ix: F,
    signers: &Vec<&dyn Signer>,
) -> usize {
    let baseline_ix = deploy_ix(0, Vec::new());
    let baseline_tx = Transaction::new_signed_with_payer(
        &[baseline_ix],
        Some(&signers[0].pubkey()),
        signers,
        Hash::default(),
    );
    let tx_size = bincode::serialized_size(&baseline_tx).unwrap() as usize;
    
    // add 1 byte buffer to account for shortvec encoding
    let chunk_size = PACKET_DATA_SIZE.saturating_sub(tx_size).saturating_sub(1);

    return chunk_size;
}

#[cfg(feature = "pyth")]
#[derive(serde::Serialize)]
pub struct PriceAccountWrapper<'a>(
    #[serde(with = "PriceAccountDef")]
    pub &'a PriceAccount
);

#[cfg(feature = "pyth")]
#[derive(Serialize, Deserialize)]
#[serde(remote = "PriceAccount")]
#[repr(C)]
pub struct PriceAccountDef {
    /// pyth magic number
    pub magic:          u32,
    /// program version
    pub ver:            u32,
    /// account type
    pub atype:          u32,
    /// price account size
    pub size:           u32,
    /// price or calculation type
    pub ptype:          PriceType,
    /// price exponent
    pub expo:           i32,
    /// number of component prices
    pub num:            u32,
    /// number of quoters that make up aggregate
    pub num_qt:         u32,
    /// slot of last valid (not unknown) aggregate price
    pub last_slot:      u64,
    /// valid slot-time of agg. price
    pub valid_slot:     u64,
    /// exponentially moving average price
    pub ema_price:      Rational,
    /// exponentially moving average confidence interval
    pub ema_conf:       Rational,
    /// unix timestamp of aggregate price
    pub timestamp:      i64,
    /// min publishers for valid price
    pub min_pub:        u8,
    /// space for future derived values
    pub drv2:           u8,
    /// space for future derived values
    pub drv3:           u16,
    /// space for future derived values
    pub drv4:           u32,
    /// product account key
    pub prod:           Pubkey,
    /// next Price account in linked list
    pub next:           Pubkey,
    /// valid slot of previous update
    pub prev_slot:      u64,
    /// aggregate price of previous update with TRADING status
    pub prev_price:     i64,
    /// confidence interval of previous update with TRADING status
    pub prev_conf:      u64,
    /// unix timestamp of previous aggregate with TRADING status
    pub prev_timestamp: i64,
    /// aggregate price info
    pub agg:            PriceInfo,
    /// price components one per quoter
    pub comp:           [PriceComp; 32],
}