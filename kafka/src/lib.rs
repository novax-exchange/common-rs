use futures::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
// use tokio::time::sleep;


pub mod consumer;
pub mod producer;

// #[tokio::main]
async fn main_o() {
    // Create a producer
    let producer = producer::create_producer("localhost:9092".to_string());
    // Send a message
    let delivery_status = producer
        .send(
            FutureRecord::to("my_topic")
                .payload("hello kafka")
                .key("some key"),
            Duration::from_secs(0),
        )
        .await;

    println!("Delivery status: {:?}", delivery_status);

    // Create a consumer
    let cfg = consumer::ConsumerConfig {
        grp_id: "the_group",
        broker: "localhost:9092",
        time_out: "6000",
        enable_partition: "false",
        auto_commit: "true",
    };
    let consumer: StreamConsumer = consumer::create_consumer(cfg);

    consumer
        .subscribe(&["my_topic"])
        .expect("Can't subscribe to specified topic");

    // Process messages
    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(m) => {
                let payload = m.payload_view::<str>().unwrap().unwrap();
                println!("Received message: {}", payload);
            }
            Err(e) => println!("Error receiving message: {:?}", e),
        }
    }
}