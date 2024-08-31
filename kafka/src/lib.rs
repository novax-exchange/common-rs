use futures::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub mod consumer;
pub mod producer;



#[tokio::test]
async fn should_produce_and_consume_same_topic()  {
    let katka_host = "localhost:19094".to_string();
    let msg_topic = "some_topic_12";
    let pay_load = br#"{"field1":"value1"}"#;
    let mut rx = consumer::consumer_async((katka_host.clone(), msg_topic.to_string() ));
    tokio::spawn(async move {
        while let Some(r) = rx.recv().await {
            println!(" the receiver ...... {:?}", r);
        }
    });

    println!("start producer ");
    let producer = producer::create_producer(katka_host);
    let status = producer
    .send(
        FutureRecord::to(msg_topic).payload(pay_load).key("123"), Duration::from_secs(0),
    ).await;
    assert_eq!(status.is_ok(), true);
}