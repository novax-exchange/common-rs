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
        .set("message.timeout.ms", "5000")
        // .set("produce.offset.report", "true")
        .set("message.max.bytes",(1 * 1024 * 1024).to_string())
        .set("allow.auto.create.topics", "true")
        .create()
        .expect("Producer creation error");
    producer
}