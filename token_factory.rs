use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateToken {}

pub fn create_token(_ctx: Context<CreateToken>, _name: String) -> Result<()> {
    Ok(())
}
