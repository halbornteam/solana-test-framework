pub mod error;
mod extensions;
pub mod util;

pub use extensions::*;
pub use solana_program_test::tokio;
pub use solana_program_test::*;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::pubkey::Pubkey;

pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;

#[macro_export]
macro_rules! processor {
    ($builtin_function:expr) => {
        Some(|vm, _arg0, _arg1, _arg2, _arg3, _arg4| {
            let vm = unsafe {
                &mut *((vm as *mut u64).offset(-($crate::get_runtime_environment_key() as isize))
                    as *mut $crate::EbpfVm<$crate::InvokeContext>)
            };
            vm.program_result =
                $crate::invoke_builtin_function(
                    unsafe {
                        core::mem::transmute::<
                            ProgramEntry,
                            solana_sdk::entrypoint::ProcessInstruction,
                        >($builtin_function)
                    },
                    vm.context_object_pointer,
                )
                .map_err(|err| $crate::EbpfError::SyscallError(err))
                .into();
        })
    };
}
