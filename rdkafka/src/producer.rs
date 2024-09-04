use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub /*(crate)*/ struct KafkaProducer;

impl KafkaProducer {

    pub fn future_producer(brokers: String) 
        -> Result<FutureProducer, super::KafkaError> 
    {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create().map_err(super::KafkaError::KafkaErr)?;
        Ok(producer)
    }

    
    pub async fn publish(producer: FutureProducer, topic: String, payload: Vec<u8>) -> Result<(), super::KafkaError>  {
        let _ = producer.send(
            FutureRecord::to(&topic)
                .payload(&payload)
                .key(&format!("Key {}", 1))
                .headers(OwnedHeaders::new().insert(Header {
                    key: "header_key",
                    value: Some("header_value"),
                })),
            Duration::from_secs(0),
        ).await;
        Ok(())
    }

}