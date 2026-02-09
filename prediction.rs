use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Predict {}

pub fn predict(_ctx: Context<Predict>, _value: i64) -> Result<()> {
    Ok(())
}
