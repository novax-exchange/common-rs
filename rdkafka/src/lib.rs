use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use std::future::Future;

mod consumer;
mod error;
mod producer;

use consumer::KafkaConsumer;
use producer::KafkaProducer;

// type KafkaError = error::KafkaError;
pub use error::KafkaError as KafkaError;

pub fn kafka_consumer(grp_brokers: (String, String), topics: Vec::<String>) -> impl Future<Output = Result<StreamConsumer, KafkaError > > {
    async {
        let (grp_id, brokers) = grp_brokers;
        let cons = KafkaConsumer::stream_consumer(grp_id, brokers, topics);
        cons
    }
}

pub fn kafka_producer(brokers: String) -> impl Future<Output = Result<FutureProducer, KafkaError > > {
    async {
        let prods = KafkaProducer::future_producer(brokers);
        prods
    }
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
        let brokers = "localhost:9193".to_string();
        let topic = "some_topic".to_string();
        let prod = kafka_producer(brokers).await?;
        
        let _ = KafkaProducer::publish(
            prod.clone(), topic, format!("Message {}", 1).as_bytes().to_vec()
        ).await?;
        
        use tokio::time::{sleep, Duration};
        sleep(Duration::from_secs(2)).await;
        let _ = KafkaProducer::publish(
            prod, "QUIT".to_string(), format!("Message {}", 1).as_bytes().to_vec()
        ).await?;
        Ok(())
    }
}
