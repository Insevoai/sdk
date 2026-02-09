use anchor_lang::prelude::*;

pub mod token_factory;
pub mod market;
pub mod prediction;
pub mod settlement;

declare_id!("InSeRv0Ai1111111111111111111111111111111");

#[program]
pub mod inservo_ai {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
