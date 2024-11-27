use anchor_lang::prelude::*;

declare_id!("72sfnJXUajuxfdMNFqpU2S6GtZaf2CNq6FtCaxQphu5R");

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
