use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::state::NumberAccount;

/// Defines 3 types of instructions:
/// 0 => Initialize with a number
/// 1 => Double the number
/// 2 => Halve the number
pub enum Instruction {
    Initialize(u64),
    Double,
    Halve,
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Self {
        match input[0] {
            0 => Self::Initialize(u64::from_le_bytes(input[1..9].try_into().unwrap())),
            1 => Self::Double,
            2 => Self::Halve,
            _ => panic!("Unknown instruction"),
        }
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(input);

    // Load the first account (our data account)
    let account_info_iter = &mut accounts.iter();
    let number_account = next_account_info(account_info_iter)?;

    // Deserialize the account
    let mut data = NumberAccount::try_from_slice(&number_account.data.borrow())?;

    // Apply logic
    match instruction {
        Instruction::Initialize(val) => {
            msg!("Initialize with value: {}", val);
            data.value = val;
        }
        Instruction::Double => {
            msg!("Doubling value");
            data.value *= 2;
        }
        Instruction::Halve => {
            msg!("Halving value");
            data.value /= 2;
        }
    }

    // Save it back
    data.serialize(&mut &mut number_account.data.borrow_mut()[..])?;
    Ok(())
}
