use anchor_lang::prelude::*;

declare_id!("BAs2AN5NU5NwcNAo2w9qaMFWdeZzXP6k3tn88nzPny82");

#[program]
pub mod switchboard_demo {
    use switchboard_on_demand::PullFeedAccountData;

    use super::*;

    pub fn test<'a>(ctx: Context<Test>) -> Result<()> {
        let feed_account = ctx.accounts.feed.data.borrow();

        let feed = PullFeedAccountData::parse(feed_account).unwrap();

        let price = feed.value();

        msg!("price: {:?}", price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Test<'info> {
    pub feed: AccountInfo<'info>,
}
