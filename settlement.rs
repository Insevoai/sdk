use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Settle {}

pub fn settle(_ctx: Context<Settle>) -> Result<()> {
    Ok(())
}
