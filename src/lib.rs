pub mod error;
mod extensions;
pub mod util;

pub use extensions::*;
pub use solana_program_test::tokio;
pub use solana_program_test::*;

#[macro_export]
macro_rules! correct_entry {
    ($correct_entry:ident, $entry:path) => {
        fn $correct_entry(
            program_id: &solana_program::pubkey::Pubkey,
            accounts: &[solana_program::account_info::AccountInfo],
            data: &[u8],
        ) -> solana_program::entrypoint::ProgramResult {
            $entry(
                program_id,
                unsafe { &*(accounts as *const [solana_program::account_info::AccountInfo]) },
                data,
            )
        }
    };
}

#[macro_export]
macro_rules! processor {
    ($entry:path) => {{
        $crate::correct_entry!(__correct_entry, $entry);
        solana_program_test::processor!(__correct_entry)
    }};
}
