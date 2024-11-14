use anchor_lang::prelude::*;

declare_id!("A4v691BGGPQfL9wDtngjt6jFzA7QtR9KXLEB144WajoR");

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
