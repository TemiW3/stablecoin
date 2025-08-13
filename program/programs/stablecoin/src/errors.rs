use anchor_lang::prelude::*;

#[error_code]
pub enum StablecoinError {
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("Below Minimum Health Factor")]
    BelowMinHealthFactor,
    #[msg("Above Minimum Health Factor, Cannot liquidate a healthy account")]
    AboveMinimumHealthFactor,
}