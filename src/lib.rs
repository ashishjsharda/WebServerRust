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
    pub fn new() -> KafkaEventsBuffer {
        KafkaEventsBuffer {}
    }
    pub fn send(&self, producer: &FutureProducer, topic: &str, key: &str, value: &str) {
        let _ = producer.send_copy(topic, Some(key), Some(value), None, None);
    }
