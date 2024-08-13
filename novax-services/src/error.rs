use thiserror::Error;
use core::result::Result as SResult;
use tokio::sync::oneshot::error::RecvError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Environment variable {0}")]
    EnvVarErr(#[from] std::env::VarError),
    #[error("std io {0}")]
    IoErr(#[from] std::io::Error ),
    #[error("Net address {0}")]
    NetAddrErr(#[from] std::net::AddrParseError),
    #[error("String message {0}")]
    StrMsgErr(String),
    #[error("Tokio recv() {0}")]
    TokioRecvErr(#[from] RecvError),
}

pub type Result<T> = SResult<T, Error>;