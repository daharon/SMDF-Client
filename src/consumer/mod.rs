use chrono::Utc;
use rusoto_core::Region;
use rusoto_sqs::{
    SqsClient, Sqs,
    Message,
    DeleteMessageRequest, SendMessageRequest, ReceiveMessageRequest,
};
use serde_json::Value;

use messages::{ClientCheckMessage, ClientCheckResultMessage, CheckResultStatus};

use std::process;
use std::str;


pub struct Consumer {
    command_queue: String,
    result_queue: String,
    sqs_client: SqsClient,
}

impl Consumer {
    pub fn new(region: Region, command_queue: &str, result_queue: &str) -> Self {
        let sqs_client = SqsClient::new(region);
        Self {
            command_queue: command_queue.to_string(),
            result_queue: result_queue.to_string(),
            sqs_client,
        }
    }

    pub fn run(&self) {
        let rcv_req = ReceiveMessageRequest {
            attribute_names:            Some(vec![String::from("All")]),
            max_number_of_messages:     Some(1),
            message_attribute_names:    Some(vec![String::from("tags")]),
            queue_url:                  self.command_queue.clone(),
            receive_request_attempt_id: None,  // Only valid for FIFO queues.
            visibility_timeout:         Some(300),
            wait_time_seconds:          Some(20),  // 20 seconds is the maximum.
        };

        loop {
            // Listen for a message.
            let rcv_res = self.sqs_client.receive_message(rcv_req.clone()).sync();
            match rcv_res {
                Err(e) => eprintln!("Error receiving message:  {:?}", e),
                Ok(msg_result) => {
                    if let Some(messages) = msg_result.messages {
                        for message in messages.iter() {
                            let result_msg = self.execute_command(message);
                            println!("Result message:\n{:?}", result_msg);
                            if let Ok(result_msg) = result_msg {
                                self.send_result(&result_msg);
                            }
                            self.delete_message(message);
                        }
                    }
                },
            }
        }
    }

    /// Execute the command as specified by the check.
    fn execute_command(&self, message: &Message)
        -> Result<ClientCheckResultMessage, Box<std::error::Error>>
    {
        println!("Received the following message:\n{:?}", message.body.as_ref().unwrap());
        // Parse the JSON message body into object.
        let sqs_notification: Value = serde_json::from_str(message.body.as_ref().unwrap())?;
        let msg_body_json = sqs_notification["Message"].as_str().unwrap();
        let check = serde_json::from_str::<ClientCheckMessage>(msg_body_json)?;
        println!("Parsed JSON message:");
        println!("{:?}", check);

        // Run the command.
        let timestamp = format!("{:?}", Utc::now());
        let output = process::Command::new("sh")
            .arg("-c")
            .arg(&check.command)
            .env_clear()
            .output();

        // Marshall the command output into a `ClientCheckResultMessage`.
        let result_msg = match output {
            Ok(opt) => ClientCheckResultMessage {
                name: check.name.clone(),
                timestamp,
                status: CheckResultStatus::OK,
                output: String::from(String::from_utf8_lossy(&opt.stdout)),
            },
            Err(e) => {
                eprintln!("Command failed to run:  {:?}", e);
                ClientCheckResultMessage {
                    name: check.name.clone(),
                    timestamp,
                    status: CheckResultStatus::UNKNOWN,
                    output: format!("Failed to run command:  {:?}", e),
                }
            }
        };
        Ok(result_msg)
    }

    /// Send the result to the results queue to be processed on the backend.
    fn send_result(&self, message: &ClientCheckResultMessage) {
        let message_body = serde_json::to_string(message).unwrap();
        let req = SendMessageRequest {
            delay_seconds: None,
            message_attributes: None,
            message_body,
            message_deduplication_id: None,  // Only valid for FIFO queues.
            message_group_id: None,  // Only valid for FIFO queues.
            queue_url: self.result_queue.clone(),
        };
        let res = self.sqs_client.send_message(req).sync();
        match res {
            Ok(r) => println!("Sent message to result queue:  {}", r.message_id.as_ref().unwrap()),
            Err(e) => println!("Failed to send message to result queue:\n{:?}", e),
        }
    }

    /// Delete the message from the queue after it has been processed.
    fn delete_message(&self, message: &Message) {
        // Delete message from queue.
        let del_req = DeleteMessageRequest {
            queue_url: self.command_queue.clone(),
            receipt_handle: message.receipt_handle.as_ref().unwrap().clone(),
        };
        let del_res = self.sqs_client.delete_message(del_req).sync();
        match del_res {
            Err(e) => eprintln!("Error deleting message:  {:?}", e),
            Ok(_) => println!("Deleted message {}", message.message_id.as_ref().unwrap()),
        }
    }
}
