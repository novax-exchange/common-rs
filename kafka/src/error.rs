use std::error::Error;
use std::fmt::{self, Debug, Display};

trait KafkaErrTrait: Debug + Display + Error {}

#[derive(Debug)]
pub enum KafkaError {
    ErrStr(String),
    KafkaErr(kafka::error::Error)
}
impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KafkaError::KafkaErr(err) => write!(f, "kafka::error::Error error: {}", err),
            KafkaError::ErrStr(s) => write!(f, "{}", s)
        }
    }
}
impl Error for KafkaError {}

impl KafkaErrTrait for KafkaError {}

impl From<kafka::Error> for KafkaError{
    fn from(f: kafka::Error) -> Self {
        Self::ErrStr(format!("kafka::Error {:?}", f))
    }
}