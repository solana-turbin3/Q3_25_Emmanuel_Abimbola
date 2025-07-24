use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
}
pub enum StakeError {
    #[msg("The time has elapsed. Try again.")]
    TimeElapsedError,
}
