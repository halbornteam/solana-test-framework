mod extensions;
pub mod util;

pub use extensions::*;
pub use solana_program_test::tokio;
pub use solana_program_test::*;

#[macro_export]
macro_rules! processor {
    ($process_instruction:expr) => {
        Some(
            |first_instruction_account: usize,
             invoke_context: &mut solana_program_test::InvokeContext| {
                $crate::builtin_process_instruction(
                    $process_instruction,
                    first_instruction_account,
                    invoke_context,
                )
            },
        )
    };
}