use anchor_lang::prelude::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
};

declare_id!("G2jURVUazEvGPQoXHzxnLCHBDen5t2jbhag76sxxRVnN");

#[program]
pub mod anchor_caller {
    use super::*;

    /// CPI to native program to initialize the number account
    pub fn initialize(ctx: Context<DoCpi>, value: u64) -> Result<()> {
        let ix = Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.data_account.key(), false), // not signer
            ],
            data: {
                let mut data = vec![0]; // discriminator for `Initialize`
                data.extend_from_slice(&value.to_le_bytes());
                data
            },
        };

        invoke(
            &ix,
            &[
                ctx.accounts.data_account.to_account_info(),
            ],
        )?;

        Ok(())
    }

    /// CPI to native program to double the value
    pub fn double(ctx: Context<DoCpi>) -> Result<()> {
        let ix = Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.data_account.key(), false),
            ],
            data: vec![1], // discriminator for `Double`
        };

        invoke(
            &ix,
            &[
                ctx.accounts.data_account.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DoCpi<'info> {
    /// CHECK: We trust the structure; passed to native program
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: CPI target; not verified because it's not Anchor-based
    pub native_program: AccountInfo<'info>,
}
