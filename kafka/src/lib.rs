use kafka::consumer::Consumer;
use kafka::producer::Producer;

mod error;
mod consumer;
mod producer;

pub use error::KafkaError as KafkaError;

use consumer::KafkaConsumer;
use producer::KafkaProducer;

pub async fn kafka_consumer(grp_brokers: (String, String), topics: Vec<String>) -> Result<Consumer, KafkaError > {
    let (grp_id, brokers) = grp_brokers;
    let cons = KafkaConsumer::stream_consumer(grp_id, vec![brokers], topics);
    cons
}

pub async fn kafka_producer(brokers: Vec<String>) -> Result<Producer, KafkaError >  {
    let prods = KafkaProducer::producer(brokers);
    prods
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn should_consumer_subscribe_ok() -> Result<(), KafkaError > {
        let grp_brokers = ("group_id".to_string(), "localhost:9193".to_string());
        let topics = vec!["some_topic".to_string(), "QUIT".to_string()];
        let con = kafka_consumer(grp_brokers, topics).await?;

        tokio::spawn(async {
            let _ = KafkaConsumer::stream_subscriber(con).await?;
            Ok::<(),KafkaError >(())
        });
        Ok(())
    }

    #[tokio::test]
    async fn should_producer_publish_ok() -> Result<(), KafkaError > {
        let brokers = vec!["localhost:9193".to_string()];
        let prod = kafka_producer(brokers).await?;
        
        let batch = vec![
            kafka::producer::Record{key: b"key0".to_vec(), value: b"value0".to_vec(), topic: "some_topic", partition:0 },
            kafka::producer::Record{key: b"key0".to_vec(), value: b"value0".to_vec(), topic: "some_topic", partition:0 },
            kafka::producer::Record{key: b"key1".to_vec(), value: b"value1".to_vec(), topic: "QUIT", partition:0 }
        ];

        let _ = KafkaProducer::publish(prod, batch).await?;


        // use tokio::time::{sleep, Duration};
        // sleep(Duration::from_secs(2)).await;
        // let _ = KafkaProducer::publish(
        //     prod, "QUIT".to_string(), format!("Message {}", 1).as_bytes().to_vec()
        // ).await?;
        Ok(())
    }
}
