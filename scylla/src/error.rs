use std::error::Error;
use std::fmt::{self, Debug, Display};

trait ScyllaErrTrait: Debug + Display + Error {}

#[derive(Debug)]
pub enum ScyllaError {
    NewSessionErr(scylla::transport::errors::NewSessionError)
}

impl fmt::Display for ScyllaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ScyllaError::TransportErr(err) => write!(f, "scylla::transport::errors error: {}", err),
            ScyllaError::NewSessionErr(err) => write!(f, "scylla::transport::errors::NewSessionError error: {}", err)
        }
    }
}
impl Error for ScyllaError {}

impl ScyllaErrTrait for ScyllaError {}
