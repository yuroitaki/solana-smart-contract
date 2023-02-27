use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(handle);

pub fn handle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Pubkey: {:?}, accounts:{:?}, instruction_data: {:?}", program_id, accounts, instruction_data);
    Ok(())
}
