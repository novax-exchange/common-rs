use futures::stream::StreamExt;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::oneshot;
use tokio::sync::mpsc;


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
        .set("allow.auto.create.topics", "true")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");
    consumer
}


pub fn consumer_async(param: (String, String)) -> mpsc::Receiver::<KafkaMessage> {
    let (tx, rx) = mpsc::channel::<KafkaMessage>(10);

    rayon::spawn(move || {
        let (host, topic) = param;
        let mut con = kafka::consumer::Consumer::from_hosts(vec![host])
            .with_topic(topic)
            .with_group("my_consumer_group".to_string())
            .with_fallback_offset(kafka::consumer::FetchOffset::Earliest)
            .with_offset_storage(Some(kafka::consumer::GroupOffsetStorage::Kafka))
            .create().unwrap();
        loop {
            let mss = con.poll().unwrap();
            if mss.is_empty() {
                println!("No messages available right now.");
                continue;
            }
            for ms in mss.iter() {
                // convert here
                let k_msg = KafkaMessage {
                    topic: format!("{}", ms.topic()),
                    partition: ms.partition(),
                    messages: ms.messages().iter().map(|x| KafkInnerMessage {
                        offset: x.offset, key: vec![], value: vec![]
                    }).collect()
                };
                
                for m in ms.messages() {
                    println!(
                        "{}:{}@{}: {:?}",
                        ms.topic(),
                        ms.partition(),
                        m.offset,
                        m.value
                    );
                }
                let _ = con.consume_messageset(ms);
                let _ = tx.clone().blocking_send(k_msg).unwrap();
            }
            con.commit_consumed().unwrap();
        }
    });
    rx
}

#[derive(Debug)]
pub struct KafkInnerMessage {
    pub offset: i64,
    pub key: Vec<u8>,
    pub value: Vec<u8>
}

#[derive(Debug)]
pub struct KafkaMessage {
    pub topic: String,
    pub partition: i32,
    pub messages: Vec::<KafkInnerMessage>
}