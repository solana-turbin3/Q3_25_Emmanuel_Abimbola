use anchor_lang::error_code;
use constant_product_curve:: CurveError;
//use constant_product

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
}
