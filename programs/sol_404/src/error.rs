use anchor_lang::prelude::*;

#[error_code]
pub enum TokenUseError {
    #[msg("Not enough balance")]
    NotEnoughBalance
}