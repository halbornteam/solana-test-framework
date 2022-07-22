mod extensions;
mod util;

pub use extensions::*;
pub use solana_program_test::tokio;
pub use solana_program_test::*;

#[macro_export]
macro_rules! processor {
    ($process_instruction:expr) => {
        Some(
            |first_instruction_account: usize,
             input: &[u8],
             invoke_context: &mut InvokeContext| {
                builtin_process_instruction(
                    $process_instruction,
                    first_instruction_account,
                    input,
                    invoke_context,
                )
            },
        )
    };
}