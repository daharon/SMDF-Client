//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

extern crate rusoto_core;
extern crate rusoto_sqs;
extern crate monitoring_client;

use rusoto_core::Region;

use monitoring_client::consumer::Consumer;


const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-34176bfc-13d5-4e56-a90c-fd8ee9069d4e";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/crap-MonitoringCheckResultQueue-1O2EGZ4NVCDV";


fn main() {
    let consumer = Consumer::new(Region::UsEast1, COMMAND_QUEUE, RESULT_QUEUE);
    consumer.run();
}

