//! # Monitoring Client
//!
//! This application is the client side of the Monitoring project.
//! 1. Register client.
//! 1. Read from the command queue.
//! 1. Execute the specified command.
//! 1. Return the result of the command to the result queue.

use simplelog::SimpleLogger;
use log::{debug, error, info};

use monitoring_client::consumer;
use monitoring_client::config::{cli, ssm};
use monitoring_client::messages::registration::ClientRegistrationRequest;
use monitoring_client::aws;


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

    match reg_res {
        Ok(res) => {
            info!("Command queue:  {}", res.command_queue);
            info!("Result queue:  {}", res.result_queue);
            consumer::run(&config, &res.command_queue, &res.result_queue);
        },
        Err(e) => error!("Failed client registration.\n{}", e),
    }
}

