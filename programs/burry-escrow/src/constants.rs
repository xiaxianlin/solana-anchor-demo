use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const DISCRIMINATOR_SIZE: usize = 8;
pub const ESCROW_SEED: &[u8] = b"MICHAEL BURRY";
pub const SOL_USDC_FEED: &str = "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR";
pub const VRF_STATE_SEED: &[u8] = b"VRFCLIENT";