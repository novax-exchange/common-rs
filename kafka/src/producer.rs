//! Connect to a kafka server.
//! Parameter: server location
//! Return a FutureProducer

// use futures::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
// use std::time::Duration;
// use tokio::time::sleep;

/// create_producer
pub fn create_producer(broker: String) ->  FutureProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &broker)
        .create()
        .expect("Producer creation error");
    producer
}