use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Trade {}

pub fn trade(_ctx: Context<Trade>, _amount: u64) -> Result<()> {
    Ok(())
}
