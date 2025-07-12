use anchor_lang::error_code;
use constant_product_curve:: CurveError;
//use constant_product

#[error_code]
pub enum AmmError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Offer Expired")]
    OfferExpired,
    #[msg("Slippage Exceeded")]
    SlippageExceeded,
    #[msg("The Pool is locked")]
    PoolLocked,
    #[msg("Overflow Detected")]
    Overflow,
    #[msg("Underflow Detected")]
    Underflow,
    #[msg("Fee is greater than 100%. This is likely a bad deal")]
    InvalidFee,
    #[msg("The amount you are trying to send out is not possible. Refine your transaction and try again.")]
    InvalidAmount,
    #[msg("Insufficient balance. Add some liquidity bro")]
    InsufficientBalance,
    #[msg("Zero Balance. Deposit some sol")]
    ZeroBalance,
    #[msg("It is not up to precision. Use a finer grained decimal")]
    InvalidPrecision
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFee,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
            
        }
    }
}
