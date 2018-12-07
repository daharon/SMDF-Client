//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

use monitoring_client::consumer;
use monitoring_client::config::{cli, ssm};


const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-1fb3ae34-2472-4f57-8c91-c5a6b0478106";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/mon-dev-MonitoringCheckResultQueue-1JSQZ4TC4K0FU";


fn main() {
    let config = cli::Config::new();
    println!("Config:  {:?}", config);

    let registration_arn = ssm::get_registration_arn(&config);
    println!("Registration ARN:  {}", registration_arn);

    // TODO: Register this client, receiving the command & result queues in the process (rusoto_lambda).

    consumer::run(config, COMMAND_QUEUE, RESULT_QUEUE);
}

