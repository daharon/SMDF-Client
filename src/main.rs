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
use smdf_client::config::cli;


fn main() {
    let config = cli::Config::new();
    SimpleLogger::init(config.log_level, simplelog::Config::default())
        .expect("Failed to initialize logging.");
    debug!("Config:  {:?}", config);

    let consumer: Arc<Consumer> = match Consumer::new(config) {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("Failed client registration:  {}", e);
            panic!(1);
        },
    };
    // Set the signal handler for graceful termination.
    let ctrlc_consumer = consumer.clone();
    ctrlc::set_handler(move || {
        info!("Received SIGINT/SIGTERM.");
        ctrlc_consumer.stop();
    }).expect("Error setting the SIGINT/SIGTERM handler.");

    consumer.start();
}

