use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    /// 允许提款的 SOL 美元价格（例如，硬编码为 21.53 美元）。
    pub unlock_price: f64,
    /// 跟踪托管账户中持有的 lamport 数量。
    pub escrow_amount: u64,
    /// 当最终掷出两个匹配的骰子时，将翻转这个标志
    pub out_of_jail: bool,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
#[derive(Default)]
pub struct VrfClientState {
    /// 存储帐户的 bump 以便以后轻松签名
    pub bump: u8,
    /// VRF 函数转储原始随机性数据的位置
    pub result_buffer: [u8; 32],
    /// 将此设置为 6，就像在 6 面骰子中一样
    pub dice_type: u8,
    /// 结果 1
    pub die_result_1: u8,
    /// 结果 2
    pub die_result_2: u8,
    /// 跟踪我们上次滚动的时间
    pub timestamp: i64,
    /// VRF 账户的公钥;由 Switchboard 程序拥有
    pub vrf: Pubkey,
    /// 托管账户的公钥
    pub escrow: Pubkey,
}
