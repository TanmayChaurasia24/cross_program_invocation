use anchor_lang::prelude::*;

declare_id!("G2jURVUazEvGPQoXHzxnLCHBDen5t2jbhag76sxxRVnN");

#[program]
pub mod anchor_caller {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
