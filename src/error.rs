use std::convert::Infallible;
use thiserror::Error;

#[derive(Debug)]
pub struct NamingRestrictionErr;
#[derive(Debug)]
pub struct StartWithForbiden_;

#[derive(Debug, Error)]
pub enum Error {
    #[error("New line `\\n` is not allowed")]
    NewLine,
    #[error("Start meassurment, field key or tag key with `_` is forbiden")]
    StartWithForbieden_,
    #[error("{}", .0)]
    Infallible(#[from] Infallible),
}
