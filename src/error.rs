use ordered_float::FloatIsNan;
use std::convert::Infallible;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("New line `\\n` is not allowed")]
    NewLine,
    #[error("Start meassurment, field key or tag key with `_` is forbiden")]
    StartWithForbieden_,
    #[error("{}", .0)]
    Infallible(#[from] Infallible),
    #[error("{}", .0)]
    FloatIsNan(#[from] FloatIsNan),
}
