use futures::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::time::sleep;

pub struct ConsumerConfig<'a> {
    pub grp_id: &'a str,
    pub broker: &'a str,
    pub time_out: &'a str,
    pub enable_partition: &'a str,
    pub auto_commit: &'a str,
}

/// create_consumer
pub fn create_consumer<'a>(cfg: ConsumerConfig<'a>) ->  StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", cfg.grp_id)
        .set("bootstrap.servers", cfg.broker)
        .set("enable.partition.eof", cfg.enable_partition)
        .set("session.timeout.ms", cfg.time_out)
        .set("enable.auto.commit", cfg.auto_commit)
        .create()
        .expect("Consumer creation failed");
    consumer
}
