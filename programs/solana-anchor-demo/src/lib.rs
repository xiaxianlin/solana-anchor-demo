use anchor_lang::prelude::*;

declare_id!("AuYsn8ts1aR9zo9EoVCDrqPqSrSSRFTTdjYtWsihdVhm");

#[program]
pub mod solana_anchor_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
