//! # SMDF Client
//!
//! This application is the client side of the SMDF monitoring project.
//! 1. Register client.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

use std::sync::Arc;

use simplelog::SimpleLogger;
use log::{debug, error, info};

use smdf_client::consumer::Consumer;
use smdf_client::config::{cli, ssm};
use smdf_client::messages::registration::ClientRegistrationRequest;
use smdf_client::aws;


fn main() {
    let config = cli::Config::new();
    SimpleLogger::init(config.log_level, simplelog::Config::default())
        .expect("Failed to initialize logging.");
    debug!("Config:  {:?}", config);

    let registration_arn = ssm::get_registration_arn(&config);
    info!("Registration ARN:  {}", registration_arn);
    let reg_req = ClientRegistrationRequest::new(&config.client_name, &config.tags);
    debug!("Registration request:  {:?}", reg_req);
    let reg_res = aws::register_client(&config.region, &registration_arn, &reg_req);

    let consumer = Arc::new(Consumer::new(config));
    // Set the signal handler for graceful termination.
    let ctrlc_consumer = consumer.clone();
    ctrlc::set_handler(move || {
        info!("Received SIGINT/SIGTERM.");
        ctrlc_consumer.stop();
    }).expect("Error setting the SIGINT/SIGTERM handler.");

    match reg_res {
        Ok(res) => {
            info!("Command queue:  {}", res.command_queue);
            info!("Result queue:  {}", res.result_queue);
            consumer.start(&res.command_queue, &res.result_queue);
        },
        Err(e) => error!("Failed client registration.\n{}", e),
    }
}

