//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.





use rusoto_core::Region;

use monitoring_client::consumer::Consumer;


const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-91da103b-c177-4d56-a744-e61adcecb802";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/crap-MonitoringCheckResultQueue-1OR9J6LJ378HG";


fn main() {
    let consumer = Consumer::new(Region::UsEast1, COMMAND_QUEUE, RESULT_QUEUE);
    consumer.run();
}

