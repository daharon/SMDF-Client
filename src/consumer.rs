use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use log::{debug, error, info};
use rusoto_sqs::{
    SqsClient, Sqs, ReceiveMessageRequest,
};

use crate::aws;
use crate::check_executor::CheckExecutor;
use crate::config::cli::Config;
use crate::config::ssm;
use crate::messages::registration::{
    ClientRegistrationRequest, ClientDeregistrationRequest,
    ClientRegistrationResponse, ClientDeregistrationResponse
};


pub struct Consumer {
    config: Config,
    stop: AtomicBool,
    command_queue: String,
    result_queue: String,
}

impl Consumer {
    /// Register the client with the monitoring service.
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        // Get registration endpoint.
        let registration_arn = ssm::get_registration_arn(&config.region, &config.registration_parameter)?;
        info!("Registration ARN:  {}", registration_arn);

        // Register
        let reg_req = ClientRegistrationRequest::new(&config.client_name, &config.tags);
        debug!("Registration request:  {:?}", reg_req);
        let reg_res: ClientRegistrationResponse =
            aws::client_registration(&config.region, &registration_arn, &reg_req)?;
        info!("Registered as {}", config.client_name);
        info!("Command queue:  {}", reg_res.command_queue);
        info!("Result queue:  {}", reg_res.result_queue);
        Ok(Consumer {
            config,
            stop: AtomicBool::new(false),
            command_queue: reg_res.command_queue,
            result_queue: reg_res.result_queue,
        })
    }

    /// Start the consumer loop.
    /// The consumer will poll the `command` queue and run the check commands,
    /// sending their responses to the `result` queue.
    /// Call [stop] on the consumer instance to stop polling and return.
    pub fn start(&self) {
        // SQS queue listener.
        let rcv_req = ReceiveMessageRequest {
            attribute_names: None,
            max_number_of_messages: Some(1),
            message_attribute_names: None,
            queue_url: self.command_queue.clone(),
            receive_request_attempt_id: None,  // Only valid for FIFO queues.
            visibility_timeout: Some(300),
            wait_time_seconds: Some(20),  // 20 seconds is the maximum.
        };
        let sqs_client = SqsClient::new(self.config.region.clone());

        info!("Listening for messages...");
        while !self.stop.load(Ordering::SeqCst) {
            // Listen for a message.
            let rcv_res = sqs_client.receive_message(rcv_req.clone()).sync();
            match rcv_res {
                Err(e) => {
                    error!("Error receiving message:  {:?}", e);
                    thread::sleep(Duration::from_secs(5));
                },
                Ok(sqs_messages) => {
                    if let Some(messages) = sqs_messages.messages {
                        for message in messages.iter() {
                            // Clone values for passing into spawned thread.
                            let c_message = message.clone();
                            let c_config = self.config.clone();
                            let c_command_queue = self.command_queue.clone();
                            let c_result_queue = self.result_queue.clone();
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

        if self.config.auto_deregister {
            info!("Auto-deregistering client.");
            match self.deregister() {
                Ok(_) => info!("Successfully de-registered."),
                Err(e) => error!("Client de-registration failed: {}", e),
            }
        }
    }

    /// Stop the consumer loop.
    pub fn stop(&self) {
        info!("Terminating...");
        self.stop.store(true, Ordering::SeqCst);
    }

    /// De-register/de-activate the client.
    fn deregister(&self) -> Result<(), Box<dyn Error>> {
        // Get de-registration endpoint.
        let deregistration_arn = ssm::get_registration_arn(&self.config.region, &self.config.deregistration_parameter)?;
        info!("De-registration ARN:  {}", deregistration_arn);
        // De-register
        let dereg_req = ClientDeregistrationRequest::new(&self.config.client_name);
        debug!("De-registration request:  {:?}", dereg_req);
        let dereg_res: ClientDeregistrationResponse =
            aws::client_registration(&self.config.region, &deregistration_arn, &dereg_req)?;
        if dereg_res.code == 200 {
            Ok(())
        } else {
            Err(Box::new(
                aws::RegistrationError { code: dereg_res.code, description: dereg_res.message }
            ))
        }
    }
}
