//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

use rusoto_core::Region;

use monitoring_client::consumer;


const CLIENT_NAME: &str = "test-client-2.example.com";
const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-1fb3ae34-2472-4f57-8c91-c5a6b0478106";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/mon-dev-MonitoringCheckResultQueue-1JSQZ4TC4K0FU";


fn main() {
    consumer::run(Region::UsEast1, CLIENT_NAME, COMMAND_QUEUE, RESULT_QUEUE);
}

