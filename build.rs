// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let anchor_supported = vec!["1.9.0", "1.10.0", "1.14.0"];
    let version = env!("CARGO_PKG_VERSION");

    if cfg!(feature = "anchor") && !anchor_supported.contains(&version) {
        panic!("anchor doesn't support Solana {}", version)
    }
}
