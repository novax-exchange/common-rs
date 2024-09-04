use std::error::Error;
use std::fmt::{self, Debug, Display};

trait KafkaErrTrait: Debug + Display + Error {}

#[derive(Debug)]
pub enum KafkaError {
    KafkaErr(rdkafka::error::KafkaError)
}

impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KafkaError::KafkaErr(err) => write!(f, "rdkafka::error::KafkaError error: {}", err)
        }
    }
}
impl Error for KafkaError {}

impl KafkaErrTrait for KafkaError {}
