//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.





use rusoto_core::Region;

use monitoring_client::consumer;


const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-ee77f092-999e-4344-a22e-0ee36666dadf";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/crap-MonitoringCheckResultQueue-A5KXZVOMP3KT";


fn main() {
    consumer::run(Region::UsEast1, COMMAND_QUEUE, RESULT_QUEUE);
}

