use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The price from the oracle is invalid")]
    InvalidPrice,
    #[msg("The health factor is below the minimum threshold")]
    BelowMinHealthFactor,
    #[msg("Cannot liquidate a healthy account")]
    AboveMinHealthFactor
}