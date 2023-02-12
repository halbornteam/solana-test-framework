use thiserror::Error;

/// Errors from the program test environment
#[derive(Error, Debug, PartialEq, Eq)]
pub enum TestFrameWorkError {
    #[error("ProgramTestExtensionError: {0}")]
    Error(&'static str),
}
