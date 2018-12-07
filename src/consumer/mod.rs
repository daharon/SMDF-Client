use chrono::{Utc, SecondsFormat};
use rusoto_sqs::{
    SqsClient, Sqs,
    Message,
    DeleteMessageRequest, SendMessageRequest, ReceiveMessageRequest,
};

use crate::messages::{ClientCheckMessage, ClientCheckResultMessage, CheckResultStatus};
use crate::config::cli::Config;

use std::thread;
use std::process;
use std::str;
use std::time::Duration;


pub fn run(config: Config, command_queue: &'static str, result_queue: &'static str) {
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
                        let message = message.clone();
                        let region = config.region.clone();
                        let client_name = config.client_name.clone();
                        thread::spawn(move || {
                            let check_message = parse_client_check_message(&message).unwrap();
                            let result_msg = execute_command(&check_message, &client_name);
                            println!("Result message:\n{:?}", result_msg);
                            let sqs_client = SqsClient::new(region);
                            if let Ok(result_msg) = result_msg {
                                send_result(&sqs_client, result_queue, result_msg);
                            }
                            delete_message(&sqs_client, command_queue, &message);
                        });
                    }
                }
            }
        }
    }
}

/// Parse the SQS message into [ClientCheckMessage] struct.
fn parse_client_check_message(message: &Message)
    -> Result<ClientCheckMessage, Box<dyn std::error::Error>>
{
    println!("Received the following message:\n{:?}", message.body.as_ref().unwrap());
    let check = serde_json::from_str::<ClientCheckMessage>(&message.body.as_ref().unwrap())?;
    println!("Parsed JSON message:");
    println!("{:?}", check);
    Ok(check)
}

/// Execute the command as specified by the check.
fn execute_command(check: &ClientCheckMessage, client_name: &str)
    -> Result<ClientCheckResultMessage, Box<dyn std::error::Error>>
{
    // Run the check command.
    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    println!("Running check:  {}", check.command);
    let output = process::Command::new("/bin/sh")
        .arg("-c")
        .arg(&check.command)
        .env_clear()
        .output();

    // Marshall the command output into a `ClientCheckResultMessage`.
    let result_msg = match output {
        Ok(opt) => ClientCheckResultMessage {
            group: check.group.clone(),
            name: check.name.clone(),
            source: String::from(client_name),
            timestamp,
            status: CheckResultStatus::from_exit_code(opt.status.code().unwrap()),
            output: String::from(String::from_utf8_lossy(&opt.stdout)),
        },
        Err(e) => {
            eprintln!("Command failed to run:  {:?}", e);
            ClientCheckResultMessage {
                group: check.group.clone(),
                name: check.name.clone(),
                source: String::from(client_name),
                timestamp,
                status: CheckResultStatus::UNKNOWN,
                output: format!("Failed to run command:  {:?}", e),
            }
        }
    };
    Ok(result_msg)
}

/// Send the result to the results queue to be processed on the backend.
fn send_result(sqs_client: &SqsClient, queue: &str, message: ClientCheckResultMessage) {
    let message_body = serde_json::to_string(&message).unwrap();
    let req = SendMessageRequest {
        delay_seconds: None,
        message_attributes: None,
        message_body,
        message_deduplication_id: None,  // Only valid for FIFO queues.
        message_group_id: None,  // Only valid for FIFO queues.
        queue_url: queue.to_string(),
    };
    let res = sqs_client.send_message(req).sync();
    match res {
        Ok(r) => println!("Sent message to result queue:  {}", r.message_id.as_ref().unwrap()),
        Err(e) => println!("Failed to send message to result queue:\n{:?}", e),
    }
}

/// Delete the message from the queue after it has been processed.
fn delete_message(sqs_client: &SqsClient, queue: &str, message: &Message) {
    // Delete message from queue.
    let del_req = DeleteMessageRequest {
        queue_url: queue.to_string(),
        receipt_handle: message.receipt_handle.as_ref().unwrap().to_string(),
    };
    let del_res = sqs_client.delete_message(del_req).sync();
    match del_res {
        Err(e) => eprintln!("Error deleting message:  {:?}", e),
        Ok(_) => println!("Deleted message {}", message.message_id.as_ref().unwrap()),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn generate_sqs_message(command: &str) -> Message {
        let body = format!("{{\"group\":\"test\",\"name\":\"Unknown check\",\"command\":\"{}\",\"timeout\":30,\"subscribers\":[]}}", command);
        Message {
            attributes: None,
            body: Some(body),
            md5_of_body: None,
            md5_of_message_attributes: None,
            message_attributes: None,
            message_id: Some(String::from("50aa8ce2-2ba9-5a30-a2b9-d88aa7418f2b")),
            receipt_handle: None,
        }
    }

    #[test]
    fn parse_sqs_message() {
        const COMMAND: &str = "true";
        let sqs_message = generate_sqs_message(COMMAND);
        let parsed_message = parse_client_check_message(&sqs_message).unwrap();
        assert_eq!("test", parsed_message.group);
        assert_eq!("Unknown check", parsed_message.name);
        assert_eq!(30, parsed_message.timeout);
    }

    #[test]
    fn execute_command_true() {
        const CLIENT_NAME: &str = "test-client";
        let check_message = ClientCheckMessage {
            group: String::from("test"),
            name: String::from("general-check"),
            command: String::from("true"),
            timeout: 30,
            subscribers: vec![],
        };

        let result = execute_command(&check_message, CLIENT_NAME).unwrap();
        assert_eq!("test", result.group);
        assert_eq!("general-check", result.name);
        assert_eq!(CheckResultStatus::OK, result.status);
        assert_eq!("", result.output);
    }
}
