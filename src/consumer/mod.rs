use chrono::{Utc, SecondsFormat};
use rusoto_core::Region;
use rusoto_sqs::{
    SqsClient, Sqs,
    Message,
    DeleteMessageRequest, SendMessageRequest, ReceiveMessageRequest,
};
use serde_json::Value;

use crate::messages::{ClientCheckMessage, ClientCheckResultMessage, CheckResultStatus};

use std::thread;
use std::process;
use std::str;
use std::time::Duration;


pub fn run(region: Region, client_name: &'static str, command_queue: &'static str, result_queue: &'static str) {
    let rcv_req = ReceiveMessageRequest {
        attribute_names:            Some(vec![String::from("All")]),
        max_number_of_messages:     Some(1),
        message_attribute_names:    Some(vec![String::from("tags")]),
        queue_url:                  command_queue.to_string(),
        receive_request_attempt_id: None,  // Only valid for FIFO queues.
        visibility_timeout:         Some(300),
        wait_time_seconds:          Some(20),  // 20 seconds is the maximum.
    };
    let sqs_client = SqsClient::new(region.clone());
    loop {
        // Listen for a message.
        let rcv_res = sqs_client.receive_message(rcv_req.clone()).sync();
        match rcv_res {
            Err(e) => {
                eprintln!("Error receiving message:  {:?}", e);
                thread::sleep(Duration::from_secs(5));
            },
            Ok(msg_result) => {
                if let Some(messages) = msg_result.messages {
                    for message in messages.iter() {
                        let message = message.clone();
                        let region = region.clone();
                        thread::spawn(move || {
                            let result_msg = execute_command(&message, client_name);
                            println!("Result message:\n{:?}", result_msg);
                            let sqs_client = SqsClient::new(region);
                            if let Ok(result_msg) = result_msg {
                                send_result(&sqs_client, result_queue, result_msg);
                            }
                            delete_message(&sqs_client, command_queue, &message);
                        });
                    }
                }
            },
        }
    }
}

/// Execute the command as specified by the check.
fn execute_command(message: &Message, client_name: &str) -> Result<ClientCheckResultMessage, Box<dyn std::error::Error>>
{
    // TODO: Switch to using raw SQS messages.
    // TODO: Move the message parsing out of this function.
    println!("Received the following message:\n{:?}", message.body.as_ref().unwrap());
    // Parse the JSON message body into object.
    let sqs_notification: Value = serde_json::from_str(message.body.as_ref().unwrap())?;
    let msg_body_json = sqs_notification["Message"].as_str().unwrap();
    let check = serde_json::from_str::<ClientCheckMessage>(msg_body_json)?;
    println!("Parsed JSON message:");
    println!("{:?}", check);

    // Run the command.
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
            client: String::from(client_name),
            timestamp,
            status: CheckResultStatus::from_exit_code(opt.status.code().unwrap()),
            output: String::from(String::from_utf8_lossy(&opt.stdout)),
        },
        Err(e) => {
            eprintln!("Command failed to run:  {:?}", e);
            ClientCheckResultMessage {
                group: check.group.clone(),
                name: check.name.clone(),
                client: String::from(client_name),
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
    use serde_json::json;
    use super::*;

    fn generate_sqs_message(command: &str) -> Message {

        let msg_body = format!("{{\"group\":\"test\",\"name\":\"Unknown check\",\"command\":\"{}\",\"timeout\":30,\"subscribers\":[]}}", command);
        let msg = json!({
            "Type": "Notification",
            "MessageId": "50aa8ce2-2ba9-5a30-a2b9-d88aa7418f2b",
            "TopicArn": "arn:aws:sns:us-east-1:746986273951:test",
            "Message": msg_body.to_string(),
            "Timestamp": "2018-11-16T09:15:20.667Z",
            "SignatureVersion": "1",
            "Signature": "iz6cgXKULV2DDKEnAsrVwhJvtC1k7aNcMaBFVmW925JXN3XYveVuc/5nZRAmQjXlmEbSRFv6vghlA0bCpnqZbmJSh7Lzww5oJvs0ddq53b0s9IaCTY0dzKkcAIKAhug0O3BER/qqsxCSbXOp+sfKvEnii6bY+LBNCRufZMss2Tan4SndTsE7elLwtx6jGE+UojYL/TgSY540LO636xCs6aow4SnWDO5D9mzgTT36O1IjR6zZz6990BQ1kI+tyNRShSNMzA95BGBivOe1xaMcnKsCQw24duEZHDGNw2qzwCXsHMOdJbvJAI8hgdoHOgkgqonVYMxPxdGE7rrMhkdabA==",
            "SigningCertURL": "https://sns.us-east-1.amazonaws.com/SimpleNotificationService-ac565b8b1a6c5d002d285f9598aa1d9b.pem",
            "UnsubscribeURL": "https://sns.us-east-1.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:us-east-1:746986273951:crap-MonitoringClientCheckDistributor-U7FGIL529EU6:41a10da7-b86a-46ed-aa7b-562de7eb0726",
            "MessageAttributes": {"tags": {"Type": "String.Array", "Value": ["test-client"]}}
        }).to_string();
        Message {
            attributes: None,
            body: Some(msg),
            md5_of_body: None,
            md5_of_message_attributes: None,
            message_attributes: None,
            message_id: Some(String::from("50aa8ce2-2ba9-5a30-a2b9-d88aa7418f2b")),
            receipt_handle: None,
        }
    }

    #[test]
    fn execute_command_true() {
        const CLIENT_NAME: &str = "test-client";
        const COMMAND: &str = "true";

        let msg = generate_sqs_message(COMMAND);
        let result = execute_command(&msg, CLIENT_NAME).unwrap();
        assert_eq!("Unknown check", result.name);
        assert_eq!(CheckResultStatus::OK, result.status);
        assert_eq!("", result.output);
    }
}
