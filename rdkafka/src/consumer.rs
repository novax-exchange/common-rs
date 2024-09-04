use rdkafka::consumer::{Consumer, CommitMode, stream_consumer::StreamConsumer};
use rdkafka::{
    config::ClientConfig, Message, message::Headers
};

pub /*(crate)*/ struct KafkaConsumer;

impl KafkaConsumer {
    pub fn stream_consumer(grp_id: String, brokers: String, topics: Vec<String> ) 
        -> Result<StreamConsumer, super::KafkaError> 
    {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &grp_id)
            .set("bootstrap.servers", &brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .create().map_err(super::KafkaError::KafkaErr)?;
        let topic_slice = topics.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let _ = consumer.subscribe(&topic_slice).map_err(super::KafkaError::KafkaErr)?;
        Ok(consumer)
    }

    pub async fn stream_subscriber(consumer: StreamConsumer) -> Result<(), super::KafkaError>  {
        loop {
            match consumer.recv().await {
                Err(e) => eprintln!("Kafka error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            eprintln!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    if m.topic() == "QUIT" {
                        println!(" quitting received ... ");
                        break;
                    }
                    println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                          m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            println!("  Header {:#?}: {:?}", header.key, header.value);
                        }
                    }
                    consumer.commit_message(&m, CommitMode::Async)
                        .map_err(super::KafkaError::KafkaErr)?;
                }
            };
        }
        Ok(())
    }
}