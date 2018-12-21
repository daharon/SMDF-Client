use std::thread;
use std::str;
use std::time::Duration;

use rusoto_sqs::{
    SqsClient, Sqs, ReceiveMessageRequest,
};

use crate::check_executor::CheckExecutor;
use crate::config::cli::Config;


pub fn run(config: &Config, command_queue: &str, result_queue: &str) {
    // SQS queue listener.
    let rcv_req = ReceiveMessageRequest {
        attribute_names:            None,
        max_number_of_messages:     Some(1),
        message_attribute_names:    None,
        queue_url:                  command_queue.to_string(),
        receive_request_attempt_id: None,  // Only valid for FIFO queues.
        visibility_timeout:         Some(300),
        wait_time_seconds:          Some(20),  // 20 seconds is the maximum.
    };
    let sqs_client = SqsClient::new(config.region.clone());

    println!("Listening for messages...");
    loop {
        // Listen for a message.
        let rcv_res = sqs_client.receive_message(rcv_req.clone()).sync();
        match rcv_res {
            Err(e) => {
                eprintln!("Error receiving message:  {:?}", e);
                thread::sleep(Duration::from_secs(5));
            },
            Ok(sqs_messages) => {
                if let Some(messages) = sqs_messages.messages {
                    for message in messages.iter() {
                        // Clone values for passing into spawned thread.
                        let c_message = message.clone();
                        let c_config = config.clone();
                        let c_command_queue = command_queue.to_string();
                        let c_result_queue = result_queue.to_string();
                        // Spawn thread to perform check.
                        thread::spawn(move || {
                            CheckExecutor
                                ::new(c_config, c_command_queue, c_result_queue, c_message)
                                .execute();
                        });
                    }
                }
            }
        }
    }
}


