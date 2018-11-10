//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{SqsClient, Sqs};
use rusoto_sqs::{ReceiveMessageRequest, DeleteMessageRequest};
use rusoto_sqs::Message;


const COMMAND_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/monitoring-8ff3a630-83ea-41d1-9a4e-06fdada0ab7c";
const RESULT_QUEUE: &str = "https://sqs.us-east-1.amazonaws.com/746986273951/crap-MonitoringCheckResultQueue-1VX98CBNHCV9N";


fn main() {
    let cmd_client = SqsClient::new(Region::UsEast1);
    let rcv_req = ReceiveMessageRequest {
        attribute_names: Some(vec![String::from("All")]),
        max_number_of_messages: Some(1),
        message_attribute_names: Some(vec![String::from("tags")]),
        queue_url: COMMAND_QUEUE.to_string(),
        receive_request_attempt_id: None,  // Only valid for FIFO queues.
        visibility_timeout: Some(300),
        wait_time_seconds: Some(20),  // 20 seconds is the maximum.
    };

    loop {
        // Listen for a message.
        let rcv_res = cmd_client.receive_message(rcv_req.clone()).sync();
        match rcv_res {
            Err(e) => eprintln!("Error receiving message:  {:?}", e),
            Ok(msg_result) => {
                if let Some(messages) = msg_result.messages {
                    for message in messages.iter() {
                        process_message(&cmd_client, COMMAND_QUEUE, message);
                    }
                }

            },
        }
    }
}

/// Process the given message and delete it from the queue.
fn process_message(client: &SqsClient, url: &str, message: &Message) {
    // Process message.
    println!("Received the following message:  {}", message.message_id.as_ref().unwrap());
    println!("    {}", message.body.as_ref().unwrap());
    // Delete message from queue.
    let del_req = DeleteMessageRequest {
        queue_url: String::from(url),
        receipt_handle: message.receipt_handle.as_ref().unwrap().clone(),
    };
    let del_res = client.delete_message(del_req).sync();
    match del_res {
        Err(e) => eprintln!("Error deleting message:  {:?}", e),
        Ok(_) => println!("Deleted message {}", message.message_id.as_ref().unwrap()),
    }
}
