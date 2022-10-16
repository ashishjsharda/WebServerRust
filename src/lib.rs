use std::collections::HashMap;
use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use serde_json::Value;

pub struct ProducerConfig{
    ack_timeout: u64,
    brokers: Vec<String>,
}

pub struct KafkaEventsBuffer {}
impl KafkaEventsBuffer {
 fn make_producer() -> FutureProducer {
     let producer_config = ProducerConfig {
         ack_timeout: 5000,
         brokers: vec!["localhost:9092".to_string()],
     };
     let mut config = ClientConfig::new();
     config.set("bootstrap.servers", "localhost:9092");
     config.set("message.timeout.ms", "5000");
     config.create()
         .expect("Producer creation error")
 }

}
