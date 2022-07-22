use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    packet::PACKET_DATA_SIZE,
    signature::Signer,
    transaction::Transaction,
};
use std::fs::{metadata, File};
use std::io::Read;

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
