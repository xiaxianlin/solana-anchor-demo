pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::consume_randomness::*;
use instructions::get_out_of_jail::*;
use instructions::init_vrf_client::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9jHs1BJrP8pHpnwQYF5gzFJPQYNcrgN8vy9ZAW7bh5EG");

#[program]
pub mod burry_escrow {

    use super::*;

    pub fn deposit(ctx: Context<Deposit>, escrow_amount: u64, unlock_price: f64) -> Result<()> {
        deposit_handler(ctx, escrow_amount, unlock_price)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }

    pub fn init_vrf_client(ctx: Context<InitVrfClient>) -> Result<()> {
        init_vrf_client_handler(ctx)
    }

    pub fn get_out_of_jail(ctx: Context<RequestRandomness>, params: RequestRandomnessParams) -> Result<()> {
        get_out_of_jail_handler(ctx, params)
    }

    pub fn consume_randomness(ctx: Context<ConsumeRandomness>) -> Result<()> {
        consume_randomness_handler(ctx)
    }
}
