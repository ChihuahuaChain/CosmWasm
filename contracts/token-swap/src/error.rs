use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw20Error(#[from] cw20_base::ContractError),

    #[error("None Error")]
    NoneError {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Max quote token error: max_token: {max_quote_token_amount}, tokens_required: {required_quote_token_amount}")]
    MaxQuoteTokenAmountExceeded {
        max_quote_token_amount: Uint128,
        required_quote_token_amount: Uint128,
    },

    #[error("Insufficient liquidity error: requested: {requested}, available: {available}")]
    InsufficientLiquidityError {
        requested: Uint128,
        available: Uint128,
    },

    #[error("Min token1 error: requested: {requested}, available: {available}")]
    MinToken1Error {
        requested: Uint128,
        available: Uint128,
    },

    #[error("Min token2 error: requested: {requested}, available: {available}")]
    MinToken2Error {
        requested: Uint128,
        available: Uint128,
    },

    #[error("Swap min error: min: {min}, available: {available}")]
    SwapMinError { min: Uint128, available: Uint128 },

    #[error("MsgExpirationError")]
    MsgExpirationError {},

    #[error("InsufficientFunds")]
    InsufficientFunds {},

    #[error("Non zero amount for base and quote tokens is expected")]
    NonZeroInputAmountExpected {},

    #[error("Uknown reply id: {id}")]
    UnknownReplyId { id: u64 },

    #[error("Failed to instantiate lp token")]
    InstantiateLpTokenError {},

    #[error("No native token provided in pair")]
    NativeTokenNotProvidedInPair {},

    #[error("Base denom is not a native token")]
    InvalidBaseDenom {},

    #[error("Quote denom is not a cw20 token")]
    InvalidQuoteDenom {},

    #[error("Swap rate should be between 0.1% and 1.0%")]
    InvalidSwapRate {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
