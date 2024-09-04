use kafka::client::{
    Compression, KafkaClient, RequiredAcks, DEFAULT_CONNECTION_IDLE_TIMEOUT_MILLIS,
};
use kafka::producer::{AsBytes, Producer, Record, DEFAULT_ACK_TIMEOUT_MILLIS};
use tokio::time::Duration;
use tokio::sync::oneshot;

pub struct KafkaProducer;

impl KafkaProducer {

    pub fn producer(brokers: Vec<String>) -> Result<Producer, super::KafkaError> {
        // client
        let mut client = KafkaClient::new(brokers);
        // client.set_client_id("kafka-rust-console-producer".into());
        client.load_metadata_all()?;
        let producer = Producer::from_client(client)
            .with_ack_timeout(Duration::from_millis(300))
            .with_required_acks(kafka::client::RequiredAcks::One)
            .with_compression(kafka::client::Compression::GZIP)
            .with_connection_idle_timeout(Duration::from_millis(5000))
            .create()?;
        Ok(producer)
    }

    pub async fn publish(producer: Producer, recs: Vec::<Record<'static, Vec<u8>, Vec<u8> >>) -> Result<(), super::KafkaError>  {
        let (tx, rx) = oneshot::channel();
        rayon::spawn(move || {
            let _ = Self::publish_inline(producer, recs, tx);
        });
        let _ = rx.await;
        Ok(())
    }

    fn publish_inline(mut producer: Producer, recs: Vec::<Record<'_, Vec<u8>, Vec<u8> >>, tx: oneshot::Sender::<bool>) -> Result<(), super::KafkaError>  {
        let rslt = producer.send_all(&recs)?;
        for r in rslt {
            for tpc in r.partition_confirms {
                if let Err(code) = tpc.offset {
                    let _ = tx.send(true);
                    return Err(super::KafkaError::ErrStr(format!("{:?}", code)))
                }
            }
        }
        let _ = tx.send(true);
        Ok(())
    }

}