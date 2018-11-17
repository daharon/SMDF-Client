//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.





use rusoto_core::Region;

use monitoring_client::consumer;


const CLIENT_NAME: &str = "test-client-1.example.com";
const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-c94fadc6-e919-4c2b-abdb-c571b8a243c2";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/crap-MonitoringCheckResultQueue-LVRJ1B7ZJX0Q";


fn main() {
    consumer::run(Region::UsEast1, CLIENT_NAME, COMMAND_QUEUE, RESULT_QUEUE);
}

