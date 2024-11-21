use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    /// 允许提款的 SOL 美元价格（例如，硬编码为 21.53 美元）。
    pub unlock_price: f64,
    /// 跟踪托管账户中持有的 lamport 数量。
    pub escrow_amount: u64,
}
