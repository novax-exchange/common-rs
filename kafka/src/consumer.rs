use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use tokio::sync::oneshot;
pub (crate) struct KafkaConsumer;

impl KafkaConsumer {
   
    pub (crate) fn stream_consumer(grp_id: String, brokers: Vec<String>, topics: Vec<String> ) 
        -> Result<Consumer, super::KafkaError> 
    {
        let mut consumer = Consumer::from_hosts(brokers)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(grp_id)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka));
            // .create().map_err(super::KafkaError::KafkaErr)?;
        for t in topics.iter() {
            consumer = consumer.with_topic(t.to_owned());
        }
        let consumer = consumer.create()?;
        Ok(consumer)
    }

    pub async fn stream_subscriber(consumer: Consumer) -> Result<(), super::KafkaError>  {
        let (tx, rx) = oneshot::channel();
        rayon::spawn(move || {
            let _ = Self::subscriber(consumer, tx);
        });
        let _ = rx.await;
        Ok(())
    }

    fn subscriber(mut consumer: Consumer, tx: oneshot::Sender::<bool>) -> Result<(), super::KafkaError> {
        loop {
            match consumer.poll() {
                Ok(ms_iter) => {
                    for ms in ms_iter.iter() {
                        for m in ms.messages() {
                            println!("{:?}", m);
                        }
                        let _ = consumer.consume_messageset(ms);
                    }
                    let _ = consumer.commit_consumed()?;
                },
                Err(e) => {
                    let _ = tx.send(true);
                    return Err(super::KafkaError::KafkaErr(e))
                }
            }
        }
    }
}